use crate::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct GameData {
    pub title: String,

    pub screen_width: i32,
    pub screen_height: i32,

    pub map_width: i32,
    pub map_height: i32,
    pub num_monsters: usize,

    pub tile_width: i32,
    pub tile_height: i32,
    pub tile_font_file: String,

    pub text_char_width: i32,
    pub text_char_height: i32,
    pub text_font_file: String,

    pub fps_cap: f32,

    pub entity_templates: Vec<EntityTemplate>,
    pub player_template: PlayerTemplate,
    pub amulet_template: AmuletTemplate,

    pub automata_map_builder: CellularAutomataMapBuilder,
    pub square_map_builder: SquareMapBuilder,
    pub drunkard_map_builder: DrunkardWalkMapBuilder,

    pub forest_map_theme: MapTheme,
    pub dungeon_map_theme: MapTheme,

    pub game_levels: Vec<GameLevel>,

    pub prefabs: Vec<Prefab>,
}
impl GameData {
    pub fn tile_display_width(&self) -> i32 {
        self.screen_width / self.tile_width
    }
    pub fn tile_display_height(&self) -> i32 {
        self.screen_height / self.tile_height
    }
    pub fn text_display_width(&self) -> i32 {
        self.screen_width / self.text_char_width
    }
    pub fn text_display_height(&self) -> i32 {
        self.screen_height / self.text_char_height
    }

    pub fn load(path: &str) -> Self {
        let file = File::open(path).expect("Failed opening file");

        from_reader(file).expect("unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entity_templates
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|e| {
                for _ in 0..e.frequency {
                    available_entities.push(e)
                }
            });

        spawn_points.iter().for_each(|pt| {
            if let Some(template) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(*pt, template, ecs);
            }
        });
    }

    fn spawn_entity(&self, pt: Point, template: &EntityTemplate, ecs: &mut World) {
        let entity = ecs.push((
            pt,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: template.glyph,
            },
            Name(template.name.clone()),
        ));
        let mut entry = ecs.entry(entity).unwrap();

        match template.entity_type {
            EntityType::Item => entry.add_component(Item {}),
            EntityType::Enemy => {
                entry.add_component(Enemy {});
                entry.add_component(FieldOfView::new(template.fov.unwrap()));
                entry.add_component(ChasingPlayer {});
                entry.add_component(Health {
                    current: template.hp.unwrap(),
                    max: template.hp.unwrap(),
                });
            }
        }

        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => entry.add_component(ProvidesHealing { amount: *n }),
                    "MagicMap" => entry.add_component(ProvidesDungeonMap),
                    _ => panic!("Don't know how to provide {provides}"),
                })
        }

        if let Some(damage) = &template.base_damage {
            entry.add_component(Damage(*damage));
            if template.entity_type == EntityType::Item {
                entry.add_component(Weapon { equipped: false });
            }
        }
    }

    pub fn spawn_player(&self, ecs: &mut World, pos: Point) {
        ecs.push((
            Player { map_level: 0 },
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: self.player_template.glyph,
            },
            Health {
                current: self.player_template.hp,
                max: self.player_template.hp,
            },
            FieldOfView::new(self.player_template.fov),
            Damage(self.player_template.base_damage),
        ));
    }

    pub fn spawn_amulet_of_yala(&self, ecs: &mut World, pos: Point) {
        ecs.push((
            Item,
            AmuletOfYala,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: self.amulet_template.glyph,
            },
            Name(self.amulet_template.name.clone()),
        ));
    }

    pub fn apply_prefab(
        &self,
        map: &mut Map,
        rng: &mut RandomNumberGenerator,
        player_start: Point,
        amulet_start: Point,
        entity_spawns: &mut Vec<Point>,
    ) {
        self.prefabs[rng.random_slice_index(&self.prefabs).unwrap()].apply_prefab(
            map,
            rng,
            player_start,
            amulet_start,
            entity_spawns,
        );
    }
}

#[derive(Clone, Copy, Deserialize, Debug)]
pub struct PlayerTemplate {
    pub fov: i32,
    pub hp: i32,
    pub glyph: char,
    pub base_damage: i32,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AmuletTemplate {
    pub glyph: char,
    pub name: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct EntityTemplate {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
    pub fov: Option<i32>,
}

#[derive(Clone, Copy, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct GameLevel {
    pub name: String,
    pub builder: MapBuilderType,
    pub theme: MapThemeType,
}
impl GameLevel {
    pub fn get_builder<'b>(&self, gamedata: &'b GameData) -> &'b dyn MapBuilder {
        match self.builder {
            MapBuilderType::CellularAutomata => &gamedata.automata_map_builder,
            MapBuilderType::Square => &gamedata.square_map_builder,
            MapBuilderType::DrunkardWalk => &gamedata.drunkard_map_builder,
        }
    }

    pub fn get_theme<'b>(&self, gamedata: &'b GameData) -> &'b MapTheme {
        match self.theme {
            MapThemeType::Forest => &gamedata.forest_map_theme,
            MapThemeType::Dungeon => &gamedata.dungeon_map_theme,
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub enum MapBuilderType {
    CellularAutomata,
    Square,
    DrunkardWalk,
}

#[derive(Clone, Deserialize, Debug)]
pub enum MapThemeType {
    Forest,
    Dungeon,
}

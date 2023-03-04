mod camera;
mod components;
mod gamedata;
mod map;
mod map_builder;
mod rect;
mod systems;
mod turn_state;

const GAME_DATA_PATH: &str = "resources/gamedata.ron";

mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::gamedata::*;
    pub use crate::map::*;
    pub use crate::map_builder::prelude::*;
    pub use crate::rect::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
}

use std::env::args;

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}
impl State {
    fn new(gamedata: GameData) -> Self {
        let (ecs, resources) = start_game(gamedata);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            let gamedata = self.resources.get::<GameData>().unwrap().to_owned();

            let (ecs, resources) = start_game(gamedata);
            self.ecs = ecs;
            self.resources = resources;
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        let gamedata = self.resources.get::<GameData>().unwrap().to_owned();

        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved and you can return to your normal life.",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            let (ecs, resources) = start_game(gamedata);
            self.ecs = ecs;
            self.resources = resources;
        }
    }

    fn advance_level(&mut self) {
        let ecs = &mut self.ecs;

        let (player_entity, player) = <(Entity, &mut Player)>::query()
            .iter_mut(ecs)
            .next()
            .unwrap();

        let player_entity = *player_entity;

        let new_map_level = player.map_level + 1;
        player.map_level = new_map_level;

        use std::collections::HashSet;

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query()
            .iter(ecs)
            .filter(|(_item, Carried(carrier))| *carrier == player_entity)
            .for_each(|(item, _carried)| {
                entities_to_keep.insert(*item);
            });

        let mut entities_to_remove = Vec::new();

        for entity in Entity::query().iter_mut(ecs) {
            if !entities_to_keep.contains(entity) {
                entities_to_remove.push(*entity);
            }
        }

        for entity in entities_to_remove {
            ecs.remove(entity);
        }
        let gamedata = self.resources.get::<GameData>().unwrap().to_owned();

        let player_start =
            start_level(&mut self.ecs, &mut self.resources, new_map_level, &gamedata);

        let (player_pos, player_fov) = <(&mut Point, &mut FieldOfView)>::query()
            .filter(component::<Player>())
            .iter_mut(&mut self.ecs)
            .next()
            .unwrap();

        player_pos.x = player_start.x;
        player_pos.y = player_start.y;
        player_fov.is_dirty = true;
    }
}

fn start_game(gamedata: GameData) -> (World, Resources) {
    let mut ecs = World::default();
    let mut resources = Resources::default();
    let new_gamedata = gamedata.clone();
    resources.insert(gamedata);

    let player_start = start_level(&mut ecs, &mut resources, 0, &new_gamedata);
    resources
        .get::<GameData>()
        .unwrap()
        .spawn_player(&mut ecs, player_start);

    (ecs, resources)
}

fn start_level(
    ecs: &mut World,
    resources: &mut Resources,
    map_level: usize,
    gamedata: &GameData,
) -> Point {
    let mut rng = RandomNumberGenerator::new();

    let map_level_def = &gamedata.game_levels[map_level];

    let map_builder = map_level_def.get_builder(gamedata);

    let BuiltMap {
        mut map,
        mut entity_spawns,
        player_start,
        amulet_start,
    } = map_builder.build(
        &mut rng,
        gamedata.map_width,
        gamedata.map_height,
        gamedata.num_monsters,
    );

    gamedata.apply_prefab(
        &mut map,
        &mut rng,
        player_start,
        amulet_start,
        &mut entity_spawns,
    );

    let theme = map_level_def.get_theme(gamedata);
    resources.insert(*theme);
    resources.insert(MapInfo {
        name: map_level_def.name.clone(),
    });

    let mut camera = Camera::new(
        gamedata.tile_display_width(),
        gamedata.tile_display_height(),
        gamedata.map_width,
        gamedata.map_height,
    );
    camera.center_on_point(player_start);
    resources.insert(camera);

    // spawn stuff
    if map_level >= gamedata.game_levels.len() - 1 {
        gamedata.spawn_amulet_of_yala(ecs, amulet_start);
    } else {
        map.set_tile(amulet_start, TileType::Exit);
    }

    gamedata.spawn_entities(ecs, &mut rng, map_level, &entity_spawns);

    resources.insert(map);
    resources.insert(TurnState::AwaitingInput);
    player_start
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        for console in 0..=2 {
            ctx.set_active_console(console);
            ctx.cls();
        }

        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = *self
            .resources
            .get::<TurnState>()
            .as_deref()
            .unwrap_or(&TurnState::AwaitingInput);
        let schedule_opt = match current_state {
            TurnState::AwaitingInput => Some(&mut self.input_systems),
            TurnState::PlayerTurn => Some(&mut self.player_systems),
            TurnState::MonsterTurn => Some(&mut self.monster_systems),
            TurnState::GameOver => {
                self.game_over(ctx);
                None
            }
            TurnState::Victory => {
                self.victory(ctx);
                None
            }
            TurnState::NextLevel => {
                self.advance_level();
                None
            }
        };
        if let Some(schedule) = schedule_opt {
            schedule.execute(&mut self.ecs, &mut self.resources);
            render_draw_buffer(ctx).expect("Render error");
        }
    }
}

fn main() -> BError {
    let args = args().into_iter().collect::<Vec<String>>();
    let gamedata = GameData::load(GAME_DATA_PATH);

    if args.len() > 1 && args[1] == "test" {
        test_harness(gamedata)
    } else {
        let context = BTermBuilder::new()
            .with_title(&gamedata.title)
            .with_fps_cap(gamedata.fps_cap)
            .with_dimensions(
                gamedata.tile_display_width(),
                gamedata.tile_display_height(),
            )
            .with_tile_dimensions(gamedata.tile_width, gamedata.tile_height)
            .with_font(
                &gamedata.tile_font_file,
                gamedata.tile_width,
                gamedata.tile_height,
            )
            .with_font(
                &gamedata.text_font_file,
                gamedata.text_char_width,
                gamedata.text_char_height,
            )
            .with_simple_console(
                gamedata.tile_display_width(),
                gamedata.tile_display_height(),
                &gamedata.tile_font_file,
            )
            .with_simple_console_no_bg(
                gamedata.tile_display_width(),
                gamedata.tile_display_height(),
                &gamedata.tile_font_file,
            )
            .with_simple_console_no_bg(
                gamedata.text_display_width(),
                gamedata.text_display_height(),
                &gamedata.text_font_file,
            )
            .build()?;
        main_loop(context, State::new(gamedata))
    }
}

fn test_harness(gamedata: GameData) -> BError {
    let mut rng = RandomNumberGenerator::new();
    let BuiltMap {
        mut map,
        mut entity_spawns,
        player_start,
        amulet_start,
    } = gamedata.drunkard_map_builder.build(
        &mut rng,
        gamedata.map_width,
        gamedata.map_height,
        gamedata.num_monsters,
    );

    gamedata.apply_prefab(
        &mut map,
        &mut rng,
        player_start,
        amulet_start,
        &mut entity_spawns,
    );
    display(
        "Final Map",
        &map,
        &player_start,
        &amulet_start,
        &entity_spawns,
        &gamedata.dungeon_map_theme,
    )
}

pub fn display(
    title: &str,
    map: &Map,
    player_start: &Point,
    amulet_start: &Point,
    entity_spawns: &[Point],
    theme: &MapTheme,
) -> BError {
    use colored::*;

    let mut output = vec!['.'; map.tiles.len()];

    map.tiles.iter().enumerate().for_each(|(idx, t)| {
        output[idx] = theme.tile_to_render(*t, Revealed::Seen);
    });

    output[map.point2d_to_index(*player_start)] = '@';
    output[map.point2d_to_index(*amulet_start)] = 'A';
    entity_spawns.iter().for_each(|p| {
        output[map.point2d_to_index(*p)] = 'M';
    });

    print!("\x1B[2J"); // CLS!
    println!(
        "----------------------\n{}\n----------------------",
        title.bright_yellow()
    );
    for y in 0..map.height() {
        for x in 0..map.width() {
            match output[map.point2d_to_index(Point::new(x, y))] {
                '#' => print!("{}", "#".bright_green()),
                '@' => print!("{}", "@".bright_yellow()),
                'M' => print!("{}", "M".bright_red()),
                'A' => print!("{}", "A".bright_magenta()),
                _ => print!("{}", ".".truecolor(64, 64, 64)),
            }
        }
        println!();
    }

    Ok(())
}

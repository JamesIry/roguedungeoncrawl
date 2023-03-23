#![allow(clippy::type_complexity)] // queries create complex types

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
    pub use bevy::prelude::*;
    pub use bracket_lib::prelude::*;

    pub type BracketRect = bracket_lib::geometry::Rect;
    pub type DCCamera = crate::camera::Camera;
    pub type DCName = crate::components::Name;
}

use std::env::args;

use prelude::*;

struct State {
    ecs: World,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}
impl State {
    fn new(gamedata: GameData) -> Self {
        let mut ecs = World::new();
        ecs.insert_resource(gamedata);

        init_game(&mut ecs);
        Self {
            ecs,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        for console in 0..=2 {
            ctx.set_active_console(console);
            ctx.cls();
        }

        self.ecs.insert_resource(KeyPress(ctx.key));
        ctx.set_active_console(0);
        self.ecs
            .insert_resource(Position(Point::from_tuple(ctx.mouse_pos())));

        let current_state = self
            .ecs
            .get_resource::<TurnState>()
            .unwrap_or(&TurnState::AwaitingInput);
        let schedule_opt = match current_state {
            TurnState::AwaitingInput => Some(&mut self.input_systems),
            TurnState::PlayerTurn => Some(&mut self.player_systems),
            TurnState::MonsterTurn => Some(&mut self.monster_systems),
            TurnState::GameOver => {
                game_over(ctx, &mut self.ecs);
                None
            }
            TurnState::Victory => {
                victory(ctx, &mut self.ecs);
                None
            }
            TurnState::NextLevel => {
                advance_level(&mut self.ecs);
                None
            }
        };
        if let Some(schedule) = schedule_opt {
            schedule.run(&mut self.ecs);
            render_draw_buffer(ctx).expect("Render error");
        }
    }
}

fn init_game(ecs: &mut World) {
    let gamedata = ecs.resource::<GameData>().to_owned();

    ecs.clear_all();

    ecs.insert_resource(gamedata);

    init_level(ecs);
}

fn advance_level(ecs: &mut World) {
    let (player_entity, mut player) = ecs.query::<(Entity, &mut Player)>().single_mut(ecs);
    player.map_level += 1;

    use std::collections::HashSet;

    let mut entities_to_keep = HashSet::new();
    entities_to_keep.insert(player_entity);

    ecs.query::<(Entity, &Carried)>()
        .iter(ecs)
        .filter(|(_item, Carried(carrier))| *carrier == player_entity)
        .for_each(|(item, _carried)| {
            entities_to_keep.insert(item);
        });

    let mut entities_to_remove = Vec::new();

    for entity in ecs.query::<Entity>().iter_mut(ecs) {
        if !entities_to_keep.contains(&entity) {
            entities_to_remove.push(entity);
        }
    }

    for entity in entities_to_remove {
        ecs.despawn(entity);
    }

    init_level(ecs);
}

fn init_level(ecs: &mut World) {
    let player_opt = ecs.query::<&Player>().get_single(ecs);

    let map_level = if let Ok(player) = player_opt {
        player.map_level
    } else {
        0
    };

    let gamedata = ecs.resource::<GameData>().clone();

    let mut rng = RandomNumberGenerator::new();

    let map_level_def = &gamedata.game_levels[map_level];

    let map_builder = map_level_def.get_builder(&gamedata);

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

    let theme = map_level_def.get_theme(&gamedata);
    ecs.insert_resource(*theme);
    ecs.insert_resource(MapInfo {
        name: map_level_def.name.clone(),
    });

    let mut camera = DCCamera::new(
        gamedata.tile_display_width(),
        gamedata.tile_display_height(),
        gamedata.map_width,
        gamedata.map_height,
    );
    camera.center_on_point(player_start);
    ecs.insert_resource(camera);

    // spawn stuff
    if map_level >= gamedata.game_levels.len() - 1 {
        gamedata.spawn_amulet_of_yala(ecs, amulet_start);
    } else {
        map.set_tile(amulet_start, TileType::Exit);
    }

    let player_opt = ecs
        .query_filtered::<(&mut Position, &mut FieldOfView), With<Player>>()
        .get_single_mut(ecs);
    // spawn a player if there isn't already one, otherwise move the player
    // to the new starting location
    if let Ok((mut player_pos, mut player_fov)) = player_opt {
        player_pos.0.x = player_start.x;
        player_pos.0.y = player_start.y;
        player_fov.is_dirty = true;
    } else {
        gamedata.spawn_player(ecs, player_start);
    }
    gamedata.spawn_entities(ecs, &mut rng, map_level, &entity_spawns);

    ecs.insert_resource(map);
    ecs.insert_resource(TurnState::AwaitingInput);
}

fn game_over(ctx: &mut BTerm, ecs: &mut World) {
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
        init_game(ecs);
    }
}

fn victory(ctx: &mut BTerm, ecs: &mut World) {
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
        init_game(ecs);
    }
}

fn main() -> BError {
    let args = args().collect::<Vec<String>>();
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

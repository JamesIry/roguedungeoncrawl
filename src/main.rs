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
    init_game_systems: Schedule,
    advance_level_systems: Schedule,
    init_level_systems: Schedule,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    game_over_systems: Schedule,
    victory_systems: Schedule,
}
impl State {
    fn new(gamedata: GameData) -> Self {
        let mut ecs = World::new();
        ecs.insert_resource(gamedata);
        ecs.insert_resource(TurnState::InitGame);

        Self {
            ecs,
            init_game_systems: build_init_game_scheduler(),
            advance_level_systems: build_advance_level_scheduler(),
            init_level_systems: build_init_level_scheduler(),
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
            game_over_systems: build_game_over_scheduler(),
            victory_systems: build_victory_scheduler(),
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
        let schedule = match current_state {
            TurnState::InitGame => &mut self.init_game_systems,
            TurnState::NextLevel => &mut self.advance_level_systems,
            TurnState::InitLevel => &mut self.init_level_systems,
            TurnState::AwaitingInput => &mut self.input_systems,
            TurnState::PlayerTurn => &mut self.player_systems,
            TurnState::MonsterTurn => &mut self.monster_systems,
            TurnState::GameOver => &mut self.game_over_systems,
            TurnState::Victory => &mut self.victory_systems,
        };

        schedule.run(&mut self.ecs);
        render_draw_buffer(ctx).expect("Render error");
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

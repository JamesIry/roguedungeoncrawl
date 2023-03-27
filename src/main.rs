#![allow(clippy::type_complexity)] // queries create complex types

mod bterm_plugin;
mod camera;
mod components;
mod gamedata;
mod geometry;
mod map;
mod map_builder;
mod random;
mod systems;
mod turn_state;

const GAME_DATA_PATH: &str = "resources/gamedata.ron";

mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::gamedata::*;
    pub use crate::geometry::dijkstra::DijkstraMap;
    pub use crate::geometry::fov::field_of_view_set;
    pub use crate::geometry::prelude::*;
    pub use crate::map::*;
    pub use crate::map_builder::prelude::*;
    pub use crate::random::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

    pub use bevy::prelude::*;
    pub use rand::rngs::ThreadRng;
    pub use rand::Rng;

    pub use bracket_lib::color::*;
    pub use bracket_lib::terminal::main_loop;
    pub use bracket_lib::terminal::render_draw_buffer;
    pub use bracket_lib::terminal::to_cp437;
    pub use bracket_lib::terminal::BError;
    pub use bracket_lib::terminal::BTerm;
    pub use bracket_lib::terminal::BTermBuilder;
    pub use bracket_lib::terminal::ColorPair;
    pub use bracket_lib::terminal::DrawBatch;
    pub use bracket_lib::terminal::GameState;

    pub type DCCamera = crate::camera::Camera;
    pub type DCName = crate::components::Name;
}

use std::env::args;

use bterm_plugin::{BTermPlugin, BTermResource};
use prelude::*;

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

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugin(BTermPlugin)
            .add_state::<TurnState>()
            .insert_resource(BTermResource(context));

        build_game_schedule(&mut app);

        app.run();

        Ok(())
    }
}

fn test_harness(gamedata: GameData) -> BError {
    let mut rng = rand::thread_rng();
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

    output[map.point_to_index(*player_start)] = '@';
    output[map.point_to_index(*amulet_start)] = 'A';
    entity_spawns.iter().for_each(|p| {
        output[map.point_to_index(*p)] = 'M';
    });

    print!("\x1B[2J"); // CLS!
    println!(
        "----------------------\n{}\n----------------------",
        title.bright_yellow()
    );
    for y in 0..map.height() {
        for x in 0..map.width() {
            match output[map.point_to_index(Point::new(x, y))] {
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

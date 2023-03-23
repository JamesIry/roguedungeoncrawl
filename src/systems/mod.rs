mod advance_level;
mod chasing;
mod check_end_of_level;
mod combat;
mod end_turn;
mod entity_renderer;
mod fov;
mod game_over;
mod hud;
mod init_game;
mod init_level;
mod map_renderer;
mod movement;
mod player_input;
mod tooltips;
mod use_items;
mod victory;

use crate::prelude::*;

pub fn build_init_game_scheduler() -> Schedule {
    let mut schedule = Schedule::new();
    schedule.add_systems((init_game::init_game_system, end_turn::end_turn_system).chain());
    schedule
}

pub fn build_advance_level_scheduler() -> Schedule {
    let mut schedule = Schedule::new();
    schedule.add_systems(
        (
            advance_level::advance_level_system,
            end_turn::end_turn_system,
        )
            .chain(),
    );
    schedule
}

pub fn build_init_level_scheduler() -> Schedule {
    let mut schedule = Schedule::new();
    schedule.add_systems((init_level::init_level_system, end_turn::end_turn_system).chain());
    schedule
}

pub fn build_input_scheduler() -> Schedule {
    let mut schedule = Schedule::new();
    schedule.add_systems(
        (
            player_input::player_input_system,
            apply_system_buffers,
            fov::fov_system,
            apply_system_buffers,
            map_renderer::map_renderer_system,
            entity_renderer::entity_renderer_system,
            hud::hud_system,
            tooltips::tooltips_system,
        )
            .chain(),
    );
    schedule
}

pub fn build_player_scheduler() -> Schedule {
    let mut schedule = Schedule::new();
    schedule
        .add_systems(
            (
                use_items::use_items_system,
                apply_system_buffers,
                combat::combat_system,
                apply_system_buffers,
                movement::movement_system,
                apply_system_buffers,
                fov::fov_system,
                apply_system_buffers,
                map_renderer::map_renderer_system,
                entity_renderer::entity_renderer_system,
                hud::hud_system,
                tooltips::tooltips_system,
                end_turn::end_turn_system,
            )
                .chain(),
        )
        .add_system(check_end_of_level::check_end_of_level_system.after(end_turn::end_turn_system));
    schedule
}

pub fn build_monster_scheduler() -> Schedule {
    let mut schedule = Schedule::new();
    schedule
        .add_systems(
            (
                use_items::use_items_system,
                apply_system_buffers,
                chasing::chasing_system,
                apply_system_buffers,
                combat::combat_system,
                apply_system_buffers,
                movement::movement_system,
                apply_system_buffers,
                fov::fov_system,
                apply_system_buffers,
                map_renderer::map_renderer_system,
                entity_renderer::entity_renderer_system,
                hud::hud_system,
                tooltips::tooltips_system,
                end_turn::end_turn_system,
            )
                .chain(),
        )
        .add_system(check_end_of_level::check_end_of_level_system.after(end_turn::end_turn_system));
    schedule
}

pub fn build_game_over_scheduler() -> Schedule {
    let mut schedule = Schedule::new();
    schedule.add_system(game_over::game_over_system);
    schedule
}

pub fn build_victory_scheduler() -> Schedule {
    let mut schedule = Schedule::new();
    schedule.add_system(victory::victory_system);
    schedule
}

mod chasing;
mod combat;
mod end_turn;
mod entity_renderer;
mod fov;
mod hud;
mod map_renderer;
mod movement;
mod player_input;
mod tooltips;
mod use_items;

use crate::prelude::*;

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
    schedule.add_systems(
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
    );
    schedule
}

pub fn build_monster_scheduler() -> Schedule {
    let mut schedule = Schedule::new();
    schedule.add_systems(
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
    );
    schedule
}

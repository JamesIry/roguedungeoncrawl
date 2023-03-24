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

pub fn build_game_schedule(app: &mut App) {
    app.add_systems(
        (init_game::init_game_system, end_turn::end_turn_system)
            .in_set(OnUpdate(TurnState::InitGame))
            .chain(),
    );

    app.add_systems(
        (
            advance_level::advance_level_system,
            end_turn::end_turn_system,
        )
            .in_set(OnUpdate(TurnState::NextLevel))
            .chain(),
    );

    app.add_systems(
        (init_level::init_level_system, end_turn::end_turn_system)
            .chain()
            .in_set(OnUpdate(TurnState::InitLevel)),
    );

    app.add_systems(
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
            .chain()
            .in_set(OnUpdate(TurnState::AwaitingInput)),
    );

    app.add_systems(
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
            check_end_of_level::check_end_of_level_system,
        )
            .chain()
            .in_set(OnUpdate(TurnState::PlayerTurn)),
    );

    app.add_systems(
        (
            use_items::use_items_system,
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
            check_end_of_level::check_end_of_level_system,
        )
            .chain()
            .in_set(OnUpdate(TurnState::MonsterTurn)),
    );

    app.add_system(game_over::game_over_system.in_set(OnUpdate(TurnState::GameOver)));

    app.add_system(victory::victory_system.in_set(OnUpdate(TurnState::Victory)));
}

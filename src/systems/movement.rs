use crate::prelude::*;

pub fn movement_system(
    mut want_move_query: Query<(
        Entity,
        &WantsToMove,
        Option<&mut FieldOfView>,
        Option<&Player>,
    )>,
    mut map: ResMut<Map>,
    mut camera: ResMut<DCCamera>,

    mut commands: Commands,
) {
    for (entity, want_move, optional_fov, optional_player) in want_move_query.iter_mut() {
        if map.can_enter_tile(want_move.destination) {
            commands
                .entity(entity)
                .insert(Position(want_move.destination));

            if let Some(mut fov) = optional_fov {
                fov.is_dirty = true;
                if optional_player.is_some() {
                    camera.center_on_point(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        let index = map.point2d_to_index(*pos);
                        map.revealed[index] = Revealed::Seen;
                    });
                }
            }
        }
        commands.entity(entity).remove::<WantsToMove>();
    }
}

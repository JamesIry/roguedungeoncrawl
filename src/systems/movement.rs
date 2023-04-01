use crate::prelude::*;

pub fn movement_system(
    mut want_move_query: Query<(Entity, &WantsToMove, Option<&mut FieldOfView>)>,
    map: Res<Map>,

    mut commands: Commands,
) {
    for (entity, want_move, optional_fov) in want_move_query.iter_mut() {
        if map.can_enter_tile(want_move.destination) {
            commands
                .entity(entity)
                .insert(Position(want_move.destination));

            if let Some(mut fov) = optional_fov {
                fov.is_dirty = true;
            }
        }
        commands.entity(entity).remove::<WantsToMove>();
    }
}

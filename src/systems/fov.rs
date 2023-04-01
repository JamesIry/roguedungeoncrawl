use crate::prelude::*;

pub fn fov_system(
    mut map: ResMut<Map>,
    mut camera: ResMut<DCCamera>,
    mut views: Query<(&Position, &mut FieldOfView, Option<&Player>)>,
) {
    views
        .iter_mut()
        .filter(|(_, fov, _)| fov.is_dirty)
        .for_each(|(pos, mut fov, optional_player)| {
            fov.visible_tiles = field_of_view_set(pos.0, fov.radius, map.as_ref());
            if optional_player.is_some() {
                fov.visible_tiles.iter().for_each(|pos| {
                    map.reveal(*pos);
                });
                camera.center_on_point(pos.0);
            }
            fov.is_dirty = false;
        });
}

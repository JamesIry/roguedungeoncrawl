use crate::prelude::*;

pub fn entity_renderer_system(
    camera: Res<DCCamera>,
    fov: Query<&FieldOfView, With<Player>>,
    points: Query<(&Position, &Render)>,
) {
    let player_fov = fov.single();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    points
        .iter()
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos.0))
        .for_each(|(pos, render)| {
            let screen_point = camera.world_point_to_screen_point(pos.0);
            draw_batch.set(screen_point, render.color, to_cp437(render.glyph));
        });
    draw_batch.submit(5000).expect("Batch error");
}

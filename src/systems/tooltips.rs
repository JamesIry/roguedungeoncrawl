use crate::prelude::*;

pub fn tooltips_system(
    mouse_pos: Res<Position>,
    camera: Res<DCCamera>,
    gamedata: Res<GameData>,
    fov: Query<&FieldOfView, With<Player>>,
    positions: Query<(&Position, &DCName, Option<&Health>)>,
) {
    let player_fov = fov.single();

    let world_mouse_pos = camera.screen_point_to_world_point(mouse_pos.0);

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    for (_, name, option_health) in positions
        .iter()
        .filter(|(pos, _, _)| player_fov.visible_tiles.contains(&pos.0) && pos.0 == world_mouse_pos)
    {
        let screen_pos = mouse_pos.0 * gamedata.tile_display_width();
        let display = if let Some(health) = option_health {
            format!("{} : {} hp", &name.0, health.current)
        } else {
            name.0.clone()
        };
        draw_batch.print(screen_pos, &display);
    }

    draw_batch.submit(10100).expect("Batch error");
}

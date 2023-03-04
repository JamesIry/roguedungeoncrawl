use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_renderer(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(pos, render)| {
            let screen_point = camera.world_point_to_screen_point(*pos);
            draw_batch.set(screen_point, render.color, to_cp437(render.glyph));
        });
    draw_batch.submit(5000).expect("Batch error");
}

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
    #[resource] gamedata: &GameData,
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    let mut positions = <(Entity, &Point, &Name)>::query();

    let world_mouse_pos = camera.screen_point_to_world_point(*mouse_pos);

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    for (entity, _, name) in positions
        .iter(ecs)
        .filter(|(_, pos, _)| player_fov.visible_tiles.contains(pos) && **pos == world_mouse_pos)
    {
        let screen_pos = *mouse_pos * gamedata.tile_display_width();
        let display = if let Some(health) = ecs
            .entry_ref(*entity)
            .iter()
            .flat_map(|entity| entity.get_component::<Health>())
            .next()
        {
            format!("{} : {} hp", &name.0, health.current)
        } else {
            name.0.clone()
        };
        draw_batch.print(screen_pos, &display);
    }

    draw_batch.submit(10100).expect("Batch error");
}

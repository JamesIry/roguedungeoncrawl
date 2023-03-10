use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Point)]
#[read_component(Player)]
#[allow(clippy::borrowed_box)]
pub fn map_renderer(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &MapTheme,
) {
    let mut fov = <(&FieldOfView, &Point)>::query().filter(component::<Player>());
    let (player_fov, player_pos) = fov.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    let viewable = camera.intersection(map.world_rect()).unwrap();
    for world_point in viewable.points() {
        let index = map.point2d_to_index(world_point);
        if map.in_bounds(world_point)
            && (player_fov.visible_tiles.contains(&world_point)
                || map.revealed[index] != Revealed::NotSeen)
        {
            let visible = player_fov.visible_tiles.contains(&world_point);
            let tint: (u8, u8, u8) = if visible {
                tint(*player_pos, world_point)
            } else {
                (128, 128, 128)
            };

            let revealed = if visible {
                Revealed::Seen
            } else {
                map.revealed[index]
            };

            let glyph = to_cp437(theme.tile_to_render(map.tile_at(world_point), revealed));

            let screen_point = camera.world_point_to_screen_point(world_point);
            draw_batch.set(screen_point, ColorPair::new(tint, BLACK), glyph);
        }
    }
    draw_batch.submit(0).expect("Batch error");
}

fn tint(point1: Point, point2: Point) -> (u8, u8, u8) {
    let distance = DistanceAlg::Pythagoras.distance2d(point1, point2) as i32;
    let clamped = (distance - 3).clamp(0, 8);
    let c = (255 - clamped * 15).clamp(0, 255);
    (c as u8, c as u8, ((c * 3) / 4) as u8)
}

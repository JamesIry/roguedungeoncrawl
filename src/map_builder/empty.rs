use crate::prelude::*;

pub struct EmptyMapBuilder {}

impl MapBuilder for EmptyMapBuilder {
    fn build(
        &self,
        rng: &mut ThreadRng,
        width: i32,
        height: i32,
        num_monsters: usize,
        max_depth: f32,
    ) -> BuiltMap {
        let mut map = Map::new(width, height, TileType::Floor);
        let player_start = map.closest_floor_point(map.center());
        let amulet_start = map.find_most_distant(player_start, max_depth);
        let entity_spawns = determine_entity_spawn_points(&map, player_start, rng, num_monsters);
        BuiltMap {
            map,
            entity_spawns,
            player_start,
            amulet_start,
        }
    }
}

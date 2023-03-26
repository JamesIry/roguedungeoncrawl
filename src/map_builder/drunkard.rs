use crate::prelude::*;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct DrunkardWalkMapBuilder {
    pub cleared_ratio: f32,
    pub stagger_distance: usize,
}

impl MapBuilder for DrunkardWalkMapBuilder {
    fn build(&self, rng: &mut ThreadRng, width: i32, height: i32, num_monsters: usize) -> BuiltMap {
        let mut map = Map::new(width, height, TileType::Wall);
        let desired_cleared = ((map.width() * map.height()) as f32 * self.cleared_ratio) as i32;
        let mut cleared = 0;

        while cleared < desired_cleared {
            let start = Point::new(rng.gen_range(1..width - 1), rng.gen_range(1..height - 1));
            cleared += self.drunkard(&mut map, start, rng);
        }

        let player_start = map.closest_floor_point(map.center());

        map.connect_disconnected(player_start, rng);

        let amulet_start = map.find_most_distant(player_start);
        let entity_spawns = determine_entity_spawn_points(&map, player_start, rng, num_monsters);
        BuiltMap {
            map,
            entity_spawns,
            player_start,
            amulet_start,
        }
    }
}

impl DrunkardWalkMapBuilder {
    fn drunkard(&self, map: &mut Map, start: Point, rng: &mut ThreadRng) -> i32 {
        let walled_rect = map.walled_rect();
        let mut drunkard_pos = start;
        let mut cleared = 0;
        let mut staggered = 0;

        while staggered < self.stagger_distance {
            let drunk_idx = map.point_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;
            let roll = rng.gen_range(0..4);
            let delta = Point::from_tuple(CARDINALS[roll]);
            let new_pos = drunkard_pos + delta;

            if walled_rect.in_bounds(new_pos) {
                drunkard_pos = new_pos;

                if map.tile_at(drunkard_pos) == TileType::Wall {
                    map.set_tile(drunkard_pos, TileType::Floor);
                    cleared += 1;
                }
            }
            staggered += 1;
        }
        cleared
    }
}

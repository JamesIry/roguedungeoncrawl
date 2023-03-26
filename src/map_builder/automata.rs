use crate::prelude::*;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct CellularAutomataMapBuilder {
    pub num_iterations: i32,
}
impl MapBuilder for CellularAutomataMapBuilder {
    fn build(&self, rng: &mut ThreadRng, width: i32, height: i32, num_monsters: usize) -> BuiltMap {
        let mut map = Self::random_noise_map(width, height, rng);

        for _ in 0..self.num_iterations {
            Self::iteration(&mut map);
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

impl CellularAutomataMapBuilder {
    fn random_noise_map(width: i32, height: i32, rng: &mut ThreadRng) -> Map {
        let mut map = Map::new(width, height, TileType::Wall);
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let roll = rng.gen_range(0..100);
                if roll > 45 {
                    let index = map.point_to_index(Point::new(x, y));
                    map.tiles[index] = TileType::Floor;
                }
            }
        }
        map
    }

    fn count_neighbor_walls(x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                let point = Point::new(x + ix, y + iy);
                if (x != 0 || y != 0) && map.tile_at(point) == TileType::Wall {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn iteration(map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..map.height() - 1 {
            for x in 1..map.width() - 1 {
                let neighbor_walls = Self::count_neighbor_walls(x, y, map);
                let idx = map.point_to_index(Point::new(x, y));
                if neighbor_walls > 4 || neighbor_walls == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }
}

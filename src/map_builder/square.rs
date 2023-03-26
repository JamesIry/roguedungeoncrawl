use crate::prelude::*;
use serde::Deserialize;
use std::cmp::min;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct SquareMapBuilder {
    pub num_rooms: usize,
}
impl MapBuilder for SquareMapBuilder {
    fn build(&self, rng: &mut ThreadRng, width: i32, height: i32, num_monsters: usize) -> BuiltMap {
        let mut map = Map::new(width, height, TileType::Wall);

        let rooms = self.build_random_rooms(&mut map, rng);
        Self::build_corridors(&mut map, rng, &rooms);
        let player_start = rooms[0].center();
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
impl SquareMapBuilder {
    fn build_random_rooms(&self, map: &mut Map, rng: &mut ThreadRng) -> Vec<IRect> {
        let mut rooms = Vec::<IRect>::with_capacity(self.num_rooms);

        while rooms.len() < self.num_rooms {
            let room = IRect::with_size(
                rng.gen_range(1..map.width() - 10),
                rng.gen_range(1..map.height() - 10),
                rng.gen_range(2..10),
                rng.gen_range(2..10),
            );

            let overlap_room = IRect::new(room.x1 - 1, room.x2 + 1, room.y1 - 1, room.y2 + 1);

            let overlap = rooms.iter().any(|r| r.intersect(&overlap_room));

            if !overlap {
                map.clear_rect(room);
                rooms.push(room);
            }
        }

        fn euclid(a: &IRect) -> i64 {
            (a.center().x as i64).pow(2) + (a.center().y as i64).pow(2)
        }
        rooms.sort_by_key(euclid);
        rooms
    }

    fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
        map.clear_rect(IRect::with_size(x, min(y1, y2), 1, (y2 - y1).abs() + 1));
    }

    fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
        map.clear_rect(IRect::with_size(min(x1, x2), y, (x2 - x1).abs() + 1, 1));
    }

    fn build_corridors(map: &mut Map, rng: &mut ThreadRng, rooms: &Vec<IRect>) {
        let index_pairs = (0..rooms.len() - 1).map(|fst| (fst, fst + 1));

        for (idx1, idx2) in index_pairs {
            let center1 = rooms[idx1].center();
            let center2 = rooms[idx2].center();

            if rng.gen_range(0..2) == 1 {
                Self::apply_horizontal_tunnel(map, center1.x, center2.x, center1.y);
                Self::apply_vertical_tunnel(map, center1.y, center2.y, center2.x);
            } else {
                Self::apply_vertical_tunnel(map, center1.y, center2.y, center1.x);
                Self::apply_horizontal_tunnel(map, center1.x, center2.x, center2.y);
            }
        }
    }
}

use crate::prelude::*;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Prefab {
    pub text: Vec<String>,
}
impl Prefab {
    pub fn width(&self) -> i32 {
        self.text.iter().map(|s| s.len()).max().unwrap() as i32
    }

    pub fn height(&self) -> i32 {
        self.text.len() as i32
    }

    pub fn apply_prefab(
        &self,
        map: &mut Map,
        rng: &mut ThreadRng,
        player_start: Point,
        amulet_start: Point,
        entity_spawns: &mut Vec<Point>,
        max_depth: f32,
    ) {
        let mut placement = None;
        let djikstra_map = map.dijkstra_map(player_start, max_depth);

        let width = self.width();
        let height = self.height();

        let mut attempts = 0;
        while placement.is_none() && attempts < 10 {
            let dimensions = IRect::with_size(
                rng.gen_range(1..map.width() - width - 1),
                rng.gen_range(1..map.height() - height - 1),
                width,
                height,
            );
            if !dimensions.in_bounds(amulet_start) {
                let mut can_place = false;

                dimensions.points().for_each(|pt| {
                    let idx = map.point_to_index(pt);
                    let distance = djikstra_map.map[idx];
                    if distance > 20.0 {
                        can_place = true;
                    }
                });

                if can_place {
                    placement = Some(Point::new(dimensions.x1, dimensions.y1));
                    entity_spawns.retain(|pt| !dimensions.in_bounds(*pt));
                }
            }

            attempts += 1;
        }

        if let Some(placement) = placement {
            for (ty, row) in self.text.iter().enumerate() {
                for (tx, c) in row.chars().enumerate() {
                    let pt = Point::new(placement.x + (tx as i32), placement.y + (ty as i32));
                    let idx = map.point_to_index(pt);
                    match c {
                        'M' => {
                            map.tiles[idx] = TileType::Floor;
                            entity_spawns.push(pt);
                        }
                        '.' => {
                            map.tiles[idx] = TileType::Floor;
                        }
                        '#' => {
                            map.tiles[idx] = TileType::Wall;
                        }
                        _ => panic!("No idea what to do with [{}] ({})", c, c as i32),
                    }
                }
            }
        }
    }
}

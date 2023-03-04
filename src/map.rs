use float_ord::FloatOrd;

use crate::prelude::*;

pub const UNREACHABLE: &f32 = &f32::MAX;
pub const CARDINALS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Revealed {
    NotSeen,
    Mapped,
    Seen,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    world_rect: Rect,
    pub revealed: Vec<Revealed>,
}
impl Map {
    pub fn new(width: i32, height: i32, tile: TileType) -> Self {
        let rect = Rect::with_size(0, 0, width, height);
        let num_tiles = rect.max_index();
        Self {
            tiles: vec![tile; num_tiles],
            world_rect: rect,
            revealed: vec![Revealed::NotSeen; num_tiles],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        self.world_rect.in_bounds(point)
    }

    pub fn width(&self) -> i32 {
        self.world_rect.width()
    }

    pub fn height(&self) -> i32 {
        self.world_rect.height()
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point)
            && (self.tile_at(point) == TileType::Floor || self.tile_at(point) == TileType::Exit)
    }

    pub fn tile_at(&self, point: Point) -> TileType {
        self.tiles[self.index_from_point(point)]
    }

    pub fn index_from_point(&self, point: Point) -> usize {
        self.world_rect.index_from_point(point)
    }

    pub fn set_tile(&mut self, point: Point, tile: TileType) {
        let index = self.index_from_point(point);
        self.tiles[index] = tile;
    }

    pub fn set_rect(&mut self, rect: Rect, tile: TileType) {
        rect.points().for_each(|point| {
            if point.x > 0
                && point.x < self.world_rect.width() - 1
                && point.y > 0
                && point.y < self.world_rect.height() - 1
            {
                self.set_tile(point, tile)
            }
        });
    }

    pub fn world_rect(&self) -> &Rect {
        &self.world_rect
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.can_enter_tile(destination) {
            Some(self.point2d_to_index(destination))
        } else {
            None
        }
    }

    pub fn clear_rect(&mut self, rect: Rect) {
        self.set_rect(rect, TileType::Floor);
    }

    pub fn find_most_distant(&self, point: Point) -> Point {
        let djikstra_map = self.djikstra_map(point);

        djikstra_map
            .map
            .iter()
            .enumerate()
            .filter(|(_, dist)| *dist < UNREACHABLE)
            .max_by_key(|(_, dist)| FloatOrd(**dist))
            .map(|(idx, _)| self.index_to_point2d(idx))
            .unwrap_or(point)
    }
    pub fn djikstra_map(&self, point: Point) -> DijkstraMap {
        DijkstraMap::new(
            self.world_rect.width(),
            self.world_rect.height(),
            &[self.point2d_to_index(point)],
            self,
            1024.0,
        )
    }

    pub fn closest_floor_point(&self, point: Point) -> Point {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(point, self.index_to_point2d(idx)),
                )
            })
            .min_by_key(|(_, distance)| FloatOrd(*distance))
            .map(|(idx, _)| self.index_to_point2d(idx))
            .unwrap_or(point)
    }

    pub fn center(&self) -> Point {
        Point::new(self.width() / 2, self.height() / 2)
    }

    pub fn walled_rect(&self) -> Rect {
        Rect::with_size(1, 1, self.width() - 2, self.height() - 2)
    }

    pub fn connect_disconnected(&mut self, player_pos: Point, rng: &mut RandomNumberGenerator) {
        let mut have_unreachable = true;
        let walled_rect = self.walled_rect();
        while have_unreachable {
            let djikstra_map = self.djikstra_map(player_pos);

            let closest_unreachable = djikstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(idx, dist)| {
                    self.can_enter_tile(self.index_to_point2d(*idx)) && *dist == UNREACHABLE
                })
                .min_by_key(|idx| {
                    let point = self.index_to_point2d(idx.0);
                    DistanceAlg::PythagorasSquared.distance2d(point, player_pos) as i32
                });

            match closest_unreachable {
                Some((mut target, _)) => {
                    while djikstra_map.map[target] == *UNREACHABLE {
                        let target_point = self.index_to_point2d(target);
                        let diff = player_pos - target_point;

                        let roll = rng.range(0, diff.x.abs() + diff.y.abs() + 2);
                        let delta = if roll < diff.x.abs() + 1 {
                            Point::new(diff.x.signum(), 0)
                        } else {
                            Point::new(0, diff.y.signum())
                        };
                        let new_target_point = target_point + delta;
                        if walled_rect.in_bounds(new_target_point) {
                            self.set_tile(new_target_point, TileType::Floor);

                            target = self.index_from_point(new_target_point);
                        }
                    }
                }
                None => {
                    have_unreachable = false;
                }
            }
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width(), self.height())
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        for cardinal in CARDINALS {
            if let Some(idx) = self.valid_exit(location, Point::from_tuple(cardinal)) {
                exits.push((idx, 1.0));
            }
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        match self.tiles[idx] {
            TileType::Wall => true,
            TileType::Floor => false,
            TileType::Exit => false,
        }
    }
}

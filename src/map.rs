use std::{
    collections::{HashSet, VecDeque},
    sync::Arc,
};

use float_ord::FloatOrd;

use crate::prelude::*;

pub const UNREACHABLE: f32 = f32::MAX;
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

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<TileType>,
    world_rect: IRect,
    pub revealed: Vec<Revealed>,
    cached_dijkstra_map: Option<(Point, f32, Arc<DijkstraMap>)>,
}
impl Map {
    pub fn new(width: i32, height: i32, tile: TileType) -> Self {
        let rect = IRect::with_size(0, 0, width, height);
        let num_tiles = rect.max_index();
        Self {
            tiles: vec![tile; num_tiles],
            world_rect: rect,
            revealed: vec![Revealed::NotSeen; num_tiles],
            cached_dijkstra_map: None,
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
        self.tiles[self.point_to_index(point)]
    }

    pub fn point_to_index(&self, point: Point) -> usize {
        self.world_rect.point_to_index(point)
    }

    pub fn index_to_point(&self, idx: usize) -> Point {
        self.world_rect.index_to_point(idx)
    }

    pub fn set_tile(&mut self, point: Point, tile: TileType) {
        self.cached_dijkstra_map = None;
        let index = self.point_to_index(point);
        self.tiles[index] = tile;
    }

    pub fn set_rect(&mut self, rect: IRect, tile: TileType) {
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

    pub fn reveal(&mut self, point: Point) {
        let index = self.point_to_index(point);
        if self.revealed[index] != Revealed::Seen {
            self.revealed[index] = Revealed::Seen;

            self.get_neighbors(index)
                .iter()
                .for_each(|pt| self.find_encompassed_tiles(*pt));
        }
    }

    /// Finds wall tiles that have been entirely encompassed
    /// by Seen wall tiles. Such encompassed Wall tiles are
    /// promoted to Seen
    /// Basically this is a flood fill algorithm with a check
    /// to make sure the flood area is fully enclosed
    fn find_encompassed_tiles(&mut self, point: Point) {
        let mut visited = HashSet::new();
        let mut encompassed = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(point);
        while let Some(point) = queue.pop_front() {
            if self.in_bounds(point) {
                let index = self.point_to_index(point);
                if !visited.contains(&index) {
                    visited.insert(index);
                    let tile = self.tiles[index];
                    let revealed = self.revealed[index];
                    match tile {
                        TileType::Wall => {
                            // if seen, we're at the edge and don't need to push more into the queue
                            // othrwise, more into the queue!
                            if revealed != Revealed::Seen {
                                encompassed.push(index);
                                self.get_neighbors(index)
                                    .iter()
                                    .for_each(|pt| queue.push_back(*pt))
                            }
                        }
                        // wandered past the end the area without hitting a seen wall,
                        // so things are not encompassed
                        TileType::Floor => return,
                        TileType::Exit => return,
                    }
                }
            }
        }

        encompassed.iter().for_each(|idx| {
            self.revealed[*idx] = Revealed::Seen;
        });
    }

    pub fn world_rect(&self) -> &IRect {
        &self.world_rect
    }

    fn valid_exit(&self, destination: Point) -> Option<usize> {
        if self.can_enter_tile(destination) {
            Some(self.point_to_index(destination))
        } else {
            None
        }
    }

    pub fn clear_rect(&mut self, rect: IRect) {
        self.set_rect(rect, TileType::Floor);
    }

    pub fn find_most_distant(&mut self, point: Point, max_depth: f32) -> Point {
        let djikstra_map = self.dijkstra_map(point, max_depth);

        djikstra_map
            .map
            .iter()
            .enumerate()
            .filter(|(_, dist)| **dist < UNREACHABLE)
            .max_by_key(|(_, dist)| FloatOrd(**dist))
            .map(|(idx, _)| self.index_to_point(idx))
            .unwrap_or(point)
    }
    pub fn dijkstra_map(&mut self, point: Point, max_depth: f32) -> Arc<DijkstraMap> {
        self.cached_dijkstra_map
            .iter()
            .filter(|(cached_point, cached_depth, _)| {
                *cached_point == point && *cached_depth == max_depth
            })
            .map(|(_, _, cached_map)| cached_map.clone())
            .next()
            .unwrap_or({
                let new_map = Arc::new(self.uncached_dijkstra_map(point, max_depth));
                self.cached_dijkstra_map = Some((point, max_depth, new_map.clone()));
                new_map
            })
    }

    pub fn uncached_dijkstra_map(&mut self, point: Point, max_depth: f32) -> DijkstraMap {
        DijkstraMap::new(
            self.world_rect.width(),
            self.world_rect.height(),
            &[self.point_to_index(point)],
            self,
            max_depth,
        )
    }

    pub fn closest_floor_point(&self, point: Point) -> Point {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| (idx, point.pythagorean_distance(self.index_to_point(idx))))
            .min_by_key(|(_, distance)| FloatOrd(*distance))
            .map(|(idx, _)| self.index_to_point(idx))
            .unwrap_or(point)
    }

    pub fn center(&self) -> Point {
        Point::new(self.width() / 2, self.height() / 2)
    }

    pub fn walled_rect(&self) -> IRect {
        IRect::with_size(1, 1, self.width() - 2, self.height() - 2)
    }

    pub fn connect_disconnected(&mut self, player_pos: Point, rng: &mut ThreadRng, max_depth: f32) {
        let walled_rect = self.walled_rect();
        'outer: loop {
            // no point in using the cached dijkstra map because we'll be changing the map
            // on every iteration, invalidating the cache
            let djikstra_map = self.uncached_dijkstra_map(player_pos, max_depth);

            let closest_unreachable = djikstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(idx, dist)| {
                    self.can_enter_tile(self.index_to_point(*idx)) && **dist == UNREACHABLE
                })
                .min_by_key(|idx| {
                    let point = self.index_to_point(idx.0);
                    point.pythagorean_squared_distance(player_pos)
                });

            match closest_unreachable {
                Some((mut target, _)) => {
                    while djikstra_map.map[target] == UNREACHABLE {
                        let target_point = self.index_to_point(target);
                        let diff = player_pos - target_point;

                        let roll = rng.gen_range(0..diff.x.abs() + diff.y.abs() + 2);
                        let delta = if roll < diff.x.abs() + 1 {
                            Point::new(diff.x.signum(), 0)
                        } else {
                            Point::new(0, diff.y.signum())
                        };
                        let new_target_point = target_point + delta;
                        if walled_rect.in_bounds(new_target_point) {
                            self.set_tile(new_target_point, TileType::Floor);

                            target = self.point_to_index(new_target_point);
                        }
                    }
                }
                None => {
                    break 'outer;
                }
            }
        }
    }

    pub fn get_neighbors(&self, idx: usize) -> Vec<Point> {
        CARDINALS
            .iter()
            .map(|dir| Point::from_tuple(*dir) + self.index_to_point(idx))
            .filter(|pt| self.in_bounds(*pt))
            .collect()
    }

    pub fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        self.get_neighbors(idx)
            .iter()
            .filter_map(|pt| self.valid_exit(*pt))
            .map(|idx| (idx, 1.0))
            .collect()
    }

    pub fn is_opaque(&self, idx: usize) -> bool {
        match self.tiles[idx] {
            TileType::Wall => true,
            TileType::Floor => false,
            TileType::Exit => false,
        }
    }
}

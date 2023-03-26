//! Translated from https://www.albertford.com/shadowcasting/

use std::collections::HashSet;

use num_rational::Rational32;

use crate::prelude::*;

pub fn field_of_view_set(origin: Point, range: i32, map: &Map) -> HashSet<Point> {
    let mut fov = HashSet::new();

    compute_fov(
        origin,
        range,
        &(|point| !map.in_bounds(point) || map.is_opaque(map.point_to_index(point))),
        &mut (|point| {
            if map.in_bounds(point) {
                fov.insert(point);
            }
        }),
    );

    fov
}

pub fn compute_fov(
    origin: Point,
    range: i32,
    is_blocked: &dyn Fn(Point) -> bool,
    mark_visible: &mut dyn FnMut(Point),
) {
    mark_visible(origin);

    for cardinal in Cardinal::values() {
        let mut quadrant = Quadrant {
            origin,
            cardinal: *cardinal,
            range: range as f32,
            is_blocked,
            mark_visible,
        };

        let first_row = Row {
            depth: 1,
            start_slope: Rational32::from_integer(-1),
            end_slope: Rational32::from_integer(1),
        };
        quadrant.scan(first_row);
    }
}

struct Quadrant<'isb, 'mv> {
    origin: Point,
    cardinal: Cardinal,
    range: f32,
    is_blocked: &'isb dyn Fn(Point) -> bool,
    mark_visible: &'mv mut dyn FnMut(Point),
}
impl<'isb, 'mv> Quadrant<'isb, 'mv> {
    fn transform(&self, tile: Point) -> Point {
        let Point { x: row, y: col } = tile;
        let delta = match &self.cardinal {
            Cardinal::North => Point::new(col, -row),
            Cardinal::South => Point::new(col, row),
            Cardinal::East => Point::new(row, col),
            Cardinal::West => Point::new(-row, col),
        };
        self.origin + delta
    }

    fn reveal(&mut self, tile: Point) {
        let transform = self.transform(tile);
        if self.origin.pythagorean_distance(transform) <= self.range {
            (self.mark_visible)(transform);
        }
    }

    fn is_wall(&self, tile: Option<Point>) -> bool {
        match tile {
            None => false,
            Some(tile) => {
                let transform = self.transform(tile);
                // add 2 to the range to "overscan" a bit. 'reveal' will make sure only
                // points in range are actually shown. The overscan ensures that
                // the logic doesn't try to reveal tiles that should be blocked by
                // other tiles that are out of range
                self.origin.pythagorean_distance(transform) <= self.range + 2.0
                    && (self.is_blocked)(transform)
            }
        }
    }

    fn is_floor(&self, tile: Option<Point>) -> bool {
        match tile {
            None => false,
            Some(tile) => {
                let transform = self.transform(tile);
                // add 2 to the range to "overscan" a bit. 'reveal' will make sure only
                // points in range are actually shown. The overscan ensures that
                // the logic doesn't try to reveal tiles that should be blocked by
                // other tiles that are out of range
                self.origin.pythagorean_distance(transform) <= self.range + 2.0
                    && !(self.is_blocked)(transform)
            }
        }
    }

    fn slope(tile: Point) -> Rational32 {
        let Point {
            x: row_depth,
            y: col,
        } = tile;
        Rational32::new(2 * col - 1, 2 * row_depth)
    }
    fn scan(&mut self, row: Row) {
        let mut rows = vec![row];

        while let Some(mut row) = rows.pop() {
            let mut prev_tile = None;
            for tile in row.tiles() {
                if self.is_wall(Some(tile)) || row.is_symmetric(tile) {
                    self.reveal(tile)
                }
                if self.is_wall(prev_tile) && self.is_floor(Some(tile)) {
                    row.start_slope = Self::slope(tile);
                }
                if self.is_floor(prev_tile) && self.is_wall(Some(tile)) {
                    let mut next_row = row.next();
                    next_row.end_slope = Self::slope(tile);
                    rows.push(next_row);
                }
                prev_tile = Some(tile);
            }
            if self.is_floor(prev_tile) {
                rows.push(row.next());
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    pub fn values() -> &'static [Cardinal] {
        static VALUES: [Cardinal; 4] = [
            Cardinal::North,
            Cardinal::South,
            Cardinal::East,
            Cardinal::West,
        ];
        &VALUES
    }
}

struct Row {
    depth: i32,
    start_slope: Rational32,
    end_slope: Rational32,
}
impl Row {
    fn tiles(&self) -> Vec<Point> {
        let min_col = Self::round_ties_up(self.start_slope * self.depth);
        let max_col = Self::round_ties_down(self.end_slope * self.depth);
        (min_col..=max_col)
            .map(|col| Point::new(self.depth, col))
            .collect()
    }

    fn next(&self) -> Self {
        Self {
            depth: self.depth + 1,
            ..*self
        }
    }

    fn is_symmetric(&self, tile: Point) -> bool {
        let Point {
            x: row_depth,
            y: col,
        } = tile;

        Rational32::from_integer(col) >= self.start_slope * row_depth
            && Rational32::from_integer(col) <= self.end_slope * row_depth
    }

    fn round_ties_up(n: Rational32) -> i32 {
        (n + Rational32::new(1, 2)).floor().to_integer()
    }

    fn round_ties_down(n: Rational32) -> i32 {
        (n - Rational32::new(1, 2)).ceil().to_integer()
    }
}

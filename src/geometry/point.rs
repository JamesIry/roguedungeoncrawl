use crate::prelude::*;
pub trait PointExtension {
    fn pythagorean_distance(&self, other: Point) -> f32;
    fn pythagorean_squared_distance(&self, other: Point) -> i32;
}

impl PointExtension for Point {
    fn pythagorean_distance(&self, other: Point) -> f32 {
        (self.pythagorean_squared_distance(other) as f32).sqrt()
    }

    fn pythagorean_squared_distance(&self, other: Point) -> i32 {
        let x_delta = self.x - other.x;
        let y_delta = self.y - other.y;
        x_delta * x_delta + y_delta * y_delta
    }
}

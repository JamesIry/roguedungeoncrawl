use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_tuple(tuple: (i32, i32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }

    pub fn pythagorean_distance(self, other: Point) -> f32 {
        (self.pythagorean_squared_distance(other) as f32).sqrt()
    }

    pub fn pythagorean_squared_distance(self, other: Point) -> i32 {
        let x_delta = self.x - other.x;
        let y_delta = self.y - other.y;
        x_delta * x_delta + y_delta * y_delta
    }

    pub fn to_bracket_point(self) -> bracket_lib::geometry::Point {
        bracket_lib::geometry::Point::new(self.x, self.y)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

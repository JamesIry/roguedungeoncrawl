use std::cmp::{max, min};

use crate::prelude::*;

pub struct RectIter<'a> {
    current_index: usize,
    max_index: usize,
    rect: &'a IRect,
}
impl<'a> RectIter<'a> {
    fn new(rect: &'a IRect) -> Self {
        Self {
            current_index: 0,
            max_index: rect.max_index(),
            rect,
        }
    }
}
impl<'a> Iterator for RectIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.max_index {
            let result = Some(self.rect.index_to_point(self.current_index));
            self.current_index += 1;
            result
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct IRect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl IRect {
    pub fn with_size(x1: i32, y1: i32, width: i32, height: i32) -> Self {
        Self {
            x1,
            y1,
            x2: x1 + width,
            y2: y1 + height,
        }
    }

    pub fn width(&self) -> i32 {
        self.x2 - self.x1
    }

    pub fn height(&self) -> i32 {
        self.y2 - self.y1
    }

    pub fn max_index(&self) -> usize {
        (self.width() * self.height()) as usize
    }

    pub fn index_to_point(&self, index: usize) -> Point {
        let idx = index as i32;
        let offset = Point::new(idx % self.width(), idx / self.width());
        self.upper_left() + offset
    }

    pub fn point_to_index(&self, point: Point) -> usize {
        let offset = self.offset_of(point);
        (offset.y * self.width() + offset.x) as usize
    }

    pub fn points(&self) -> RectIter {
        RectIter::new(self)
    }

    pub fn offset_of(&self, point: Point) -> Point {
        point - self.upper_left()
    }

    pub fn upper_left(&self) -> Point {
        Point {
            x: self.x1,
            y: self.y1,
        }
    }

    pub fn moved_to_point(&self, point: Point) -> Self {
        Self::with_size(point.x, point.y, self.width(), self.height())
    }

    pub fn centered_at_point(&self, point: Point) -> Self {
        self.moved_to_point(Point::new(
            point.x - self.width() / 2,
            point.y - self.height() / 2,
        ))
    }

    pub fn intersection(&self, other: &IRect) -> Option<IRect> {
        let x1 = max(self.x1, other.x1);
        let x2 = min(self.x2, other.x2);
        let y1 = max(self.y1, other.y1);
        let y2 = min(self.y2, other.y2);

        if x1 > x2 || y1 > y2 {
            None
        } else {
            Some(Self { x1, x2, y1, y2 })
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= self.x1 && point.x < self.x2 && point.y >= self.y1 && point.y < self.y2
    }

    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> IRect {
        IRect {
            x1: min(x1, x2),
            x2: max(x1, x2),
            y1: min(y1, y2),
            y2: max(y1, y2),
        }
    }

    pub fn center(&self) -> Point {
        (self.upper_left() + self.lower_right()) / 2
    }

    pub fn lower_right(&self) -> Point {
        Point::new(self.x2, self.y2)
    }

    pub(crate) fn intersect(&self, other: &IRect) -> bool {
        self.intersection(other).is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    fn rect1234() -> IRect {
        IRect {
            x1: 1,
            y1: 2,
            x2: 3,
            y2: 4,
        }
    }

    fn rect0179() -> IRect {
        IRect {
            x1: 0,
            y1: 1,
            x2: 7,
            y2: 9,
        }
    }

    #[test]
    fn test_new() {
        assert_eq!(IRect::new(1, 3, 2, 4), rect1234());
        assert_eq!(IRect::new(3, 1, 2, 4), rect1234());
        assert_eq!(IRect::new(1, 3, 4, 2), rect1234());
        assert_eq!(IRect::new(3, 1, 4, 2), rect1234());
    }

    #[test]
    fn test_upper_left() {
        let upper_left = Point::new(1, 2);
        assert_eq!(IRect::new(1, 3, 2, 4).upper_left(), upper_left);
        assert_eq!(IRect::new(3, 1, 2, 4).upper_left(), upper_left);
        assert_eq!(IRect::new(1, 3, 4, 2).upper_left(), upper_left);
        assert_eq!(IRect::new(3, 1, 4, 2).upper_left(), upper_left);
    }

    #[test]
    fn test_in_bounds() {
        assert!(rect0179().in_bounds(Point::new(0, 1)));
        assert!(rect0179().in_bounds(Point::new(6, 8)));
        assert!(!rect0179().in_bounds(Point::new(0, 0)));
        assert!(!rect0179().in_bounds(Point::new(-1, 1)));
        assert!(!rect0179().in_bounds(Point::new(7, 8)));
        assert!(!rect0179().in_bounds(Point::new(6, 9)));
    }

    #[test]
    fn test_move_to_point() {
        assert_eq!(
            rect0179().moved_to_point(Point::new(2, 3)),
            IRect::new(2, 9, 3, 11)
        );
    }

    #[test]
    fn test_center_at_point() {
        assert_eq!(
            rect0179().centered_at_point(Point::new(5, 6)),
            IRect::new(2, 9, 2, 10)
        );
    }

    #[test]
    fn test_intersection() {
        assert_eq!(
            rect1234().intersection(&rect0179()),
            Some(IRect::new(1, 3, 2, 4))
        );

        assert_eq!(
            IRect::new(2, 10, 5, 23).intersection(&rect0179()),
            Some(IRect::new(2, 7, 5, 9))
        );

        assert_eq!(IRect::new(21, 22, 23, 24).intersection(&rect0179()), None);
    }

    #[test]
    fn test_index() {
        assert_eq!(rect1234().max_index(), 4);
        assert_eq!(rect0179().max_index(), 56);

        assert_eq!(rect0179().index_to_point(0), Point::new(0, 1));
        assert_eq!(rect0179().index_to_point(6), Point::new(6, 1));
        assert_eq!(rect0179().index_to_point(49), Point::new(0, 8));
        assert_eq!(rect0179().index_to_point(55), Point::new(6, 8));

        let rect = rect0179();
        let mut index = 0;

        for y in 1..9 {
            for x in 0..7 {
                let point = Point::new(x, y);

                assert_eq!(rect.index_to_point(index), point);
                assert_eq!(rect.point_to_index(point), index);
                index += 1;
            }
        }
    }

    #[test]
    fn test_points() {
        {
            let points1234: Vec<Point> = rect1234().points().collect();
            assert_eq!(
                points1234,
                vec![
                    Point::new(1, 2),
                    Point::new(2, 2),
                    Point::new(1, 3),
                    Point::new(2, 3),
                ]
            )
        }
        {
            let rect0179 = rect0179();
            let points0179: Vec<Point> = rect0179.points().collect();
            assert_eq!(points0179.len(), 56);
            let mut index = 0;

            for y in 1..9 {
                for x in 0..7 {
                    let point = Point::new(x, y);

                    assert_eq!(points0179[index], point);
                    index += 1;
                }
            }
        }
    }
}

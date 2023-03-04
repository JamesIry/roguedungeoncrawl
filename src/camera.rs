use crate::prelude::*;

#[derive(PartialEq, Debug)]
pub struct Camera {
    view_rect: Rect,
    world_width: i32,
    world_height: i32,
}
impl Camera {
    pub fn new(
        display_width: i32,
        display_height: i32,
        world_width: i32,
        world_height: i32,
    ) -> Self {
        Self {
            view_rect: Rect::with_size(0, 0, display_width, display_height),
            world_width,
            world_height,
        }
    }

    pub fn center_on_point(&mut self, pos: Point) {
        let new_view = self.view_rect.centered_at_point(pos);

        let mut x1 = new_view.x1;
        let mut x2 = new_view.x2;
        let mut y1 = new_view.y1;
        let mut y2 = new_view.y2;

        if x1 < 0 {
            x1 = 0;
            x2 = self.view_rect.width();
        } else if x2 > self.world_width {
            x2 = self.world_width;
            x1 = x2 - self.view_rect.width();
        }

        if y1 < 0 {
            y1 = 0;
            y2 = self.view_rect.height();
        } else if y2 > self.world_height {
            y2 = self.world_height;
            y1 = y2 - self.view_rect.height();
        }

        self.view_rect = Rect::new(x1, x2, y1, y2);
    }

    pub fn world_point_to_screen_point(&self, point: Point) -> Point {
        self.view_rect.offset_of(point)
    }

    pub fn screen_point_to_world_point(&self, point: Point) -> Point {
        self.view_rect.upper_left() + point
    }

    pub fn intersection(&self, rect: &Rect) -> Option<Rect> {
        self.view_rect.intersection(rect)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Camera::new(20, 15, 23, 47),
            Camera {
                view_rect: Rect::new(0, 20, 0, 15),
                world_width: 23,
                world_height: 47
            }
        );
    }

    #[test]
    fn test_center_at_point() {
        let mut camera = Camera::new(20, 15, 100, 100);
        camera.center_on_point(Point::new(30, 25));
        assert_eq!(camera.view_rect, Rect::new(20, 40, 18, 33));
    }

    #[test]
    fn offset() {
        let mut camera = Camera::new(20, 15, 100, 100);
        camera.center_on_point(Point::new(30, 25));
        let point = Point::new(21, 20);

        assert_eq!(camera.world_point_to_screen_point(point), Point::new(1, 2));
    }
}

use super::{point::Point, size::Size};

#[derive(Debug, Clone)]
pub struct Rect {
    pub point: Point,
    pub size: Size,
}

impl Rect {
    pub fn new<P: Into<Point>, S: Into<Size>>(point: P, size: S) -> Rect {
        Rect {
            point: point.into(),
            size: size.into(),
        }
    }
    pub fn center(&self) -> Point {
        Point::new(
            self.point.x + self.size.width / 2.0,
            self.point.y + self.size.height / 2.0,
        )
    }
}

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
}
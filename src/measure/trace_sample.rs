use crate::geometry::point::Point;

use super::{pen_angle::PenAngle, pressure::Pressure32};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TraceSample {
    point: Point,
    pressure: Pressure32,
    pen_angle: PenAngle,
}

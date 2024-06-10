use crate::geometry::angle::A16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PenAngle {
    pitch: A16,
    yaw: A16,
}

use modifier::Modifier;
use wgpu::Instance;

use crate::geometry::point::Point;

pub mod modifier;
pub mod geometry;
pub mod style;
pub trait Object {
    fn render(&self, position: &Point, ctx: &RenderContext);
}

pub struct RenderContext {
    instance: Instance,
}

pub struct UnitObject {
    modifier: Box<dyn Modifier>,
    unit: Box<dyn Object>,
}

pub struct Spirit<O: Object> {
    pub position: Point,
    pub object: O,
}

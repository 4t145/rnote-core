use modifier::Modifier;
use wgpu::Instance;

use crate::geometry::{angle::A16, point::Point, rect::Rect};

pub mod geometry;
pub mod modifier;
pub mod style;
pub trait Object {
    fn render(&self, position: &Point, ctx: &mut RenderContext);
}

pub struct RenderContext {
    camera: Camera2D,
    instance: Instance,
    render_pipeline: wgpu::RenderPipeline,
    encoder: wgpu::CommandEncoder,
    device: wgpu::Device,
    config: Config,
}

pub struct Camera2D {
    pub region: Rect,
    pub rotation: A16,
}
pub struct Config {
    view_formats: Vec<wgpu::TextureFormat>,
}

pub struct UnitObject {
    modifier: Box<dyn Modifier>,
    unit: Box<dyn Object>,
}

pub struct Spirit<O: Object> {
    pub position: Point,
    pub object: O,
}

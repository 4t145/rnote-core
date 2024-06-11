use crate::geometry::point::Point;

pub mod polygon;
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ColoredVertex {
    pub position: Point,
    pub color: wgpu::Color,
}
unsafe impl bytemuck::NoUninit for ColoredVertex {}
pub trait VertexLayout {
    const LAYOUT: wgpu::VertexBufferLayout<'static>;
}


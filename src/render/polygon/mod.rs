use crate::geometry::point::Point;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Polygon<const N: usize> {
    pub points: [ColoredVertex; N],
}

unsafe impl<const N: usize> bytemuck::NoUninit for Polygon<N> {}

use wgpu::{util::DeviceExt, VertexBufferLayout};

use super::{ColoredVertex, VertexLayout};
impl<const N: usize> Polygon<N> {
    pub fn render() {
        // render polygon
    }
    pub fn create_buffer_init(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Polygon Buffer"),
            contents: bytemuck::cast_slice(&self.points),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
}

impl<const N: usize> VertexLayout for Polygon<N> {
    const LAYOUT: VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<ColoredVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
    };
}

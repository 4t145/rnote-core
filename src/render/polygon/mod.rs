use crate::{camera::Camera2D, geometry::point::Point};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Polygon<const N: usize> {
    pub points: [ColoredVertex; N],
}
const CAMERA_BIND_GROUP: u32 = 0;

use wgpu::{util::DeviceExt, BindGroup, BufferSlice, VertexBufferLayout};

use super::{ColoredVertex, ObjectRef, Ur, VertexLayout};
impl<const N: usize> ObjectRef<'_, Polygon<N>> {
    pub fn create_buffer_init(self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Polygon Buffer"),
            contents: bytemuck::cast_slice(&self.points),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
    pub fn sync_buffer(self, buffer: &wgpu::Buffer, queue: &wgpu::Queue) {
        queue.write_buffer(buffer, 0, bytemuck::cast_slice(&self.points))
    }
    pub fn render<'render_pass>(
        self,
        rp: &mut wgpu::RenderPass<'render_pass>,
        buffer: BufferSlice<'render_pass>,
        camera: &'render_pass Ur<Camera2D, BindGroup>,
    ) {
        rp.set_bind_group(CAMERA_BIND_GROUP, camera, &[]);
        rp.set_vertex_buffer(0, buffer);
        rp.draw(0..self.points.len() as u32, 0..1);
    }


}

impl<const N: usize> VertexLayout for Polygon<N> {
    const LAYOUT: VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<ColoredVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
    };
}

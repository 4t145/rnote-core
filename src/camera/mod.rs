use wgpu::util::DeviceExt;

use crate::{
    geometry::{angle::A16, rect::Rect},
    render::Ur,
};

pub struct Camera2D {
    pub region: Rect,
    pub rotation: A16,
    pub scale: f32,
}

impl Camera2D {
    fn to_matrix(&self) -> [f32; 16] {
        let center = self.region.center();
        let translation = glam::Mat4::from_translation(glam::Vec3::new(center.x, center.y, 0.0));
        let rotation = glam::Mat4::from_rotation_z(self.rotation.to_rad());
        let scale = glam::Mat4::from_scale(glam::Vec3::new(self.scale, self.scale, 1.0));
        (translation * rotation * scale).to_cols_array()
    }

    fn create_buffer(&self, device: &wgpu::Device) -> Ur<Self, wgpu::Buffer> {
        let resource = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera2D Buffer"),
            contents: bytemuck::cast_slice(&[self.to_matrix()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        Ur::new(resource)
    }

    fn create_bind_group_layout(device: &wgpu::Device) -> Ur<Self, wgpu::BindGroupLayout> {
        device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera2D Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            })
            .into()
    }

    fn create_bind_group(
        device: &wgpu::Device,
        layout: &Ur<Self, wgpu::BindGroupLayout>,
        buffer: &Ur<Self, wgpu::Buffer>,
    ) -> Ur<Self, wgpu::BindGroup> {
        device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Camera2D Bind Group"),
                layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            })
            .into()
    }
}

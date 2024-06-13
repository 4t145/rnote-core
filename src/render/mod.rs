use std::ops::Deref;

use crate::{camera::Camera2D, geometry::point::Point, handle::Extract};

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

/// # Unique Resource
///
/// A resource that is bound to a type.
///
pub struct Ur<T, R> {
    pub resource: R,
    bind_type_marker: std::marker::PhantomData<T>,
}

impl<T, R> Ur<T, R> {
    pub fn new(resource: R) -> Self {
        Self {
            resource,
            bind_type_marker: std::marker::PhantomData,
        }
    }
}

impl<T, R> std::ops::Deref for Ur<T, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<T, R> std::ops::DerefMut for Ur<T, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource
    }
}

impl<T, R> From<R> for Ur<T, R> {
    fn from(resource: R) -> Self {
        Self::new(resource)
    }
}

pub struct RenderContext<'render> {
    pub camera: Ur<Camera2D, wgpu::BindGroup>,
    pub instance: wgpu::Instance,
    pub render_pipeline: wgpu::RenderPipeline,
    pub encoder: wgpu::CommandEncoder,
    pub device: wgpu::Device,
    pub render_pass: wgpu::RenderPass<'render>,
}
#[derive(Debug, PartialEq)]
pub struct ObjectRef<'r, R>(pub &'r R);
impl<R> Clone for ObjectRef<'_, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> Copy for ObjectRef<'_, R> {}

impl<'a, R> Deref for ObjectRef<'a, R> {
    type Target = &'a R;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ObjectRenderContext<'render, R> {
    pub object: ObjectRef<'render, R>,
    pub buffer: &'render wgpu::Buffer,
    pub camera: &'render Ur<Camera2D, wgpu::BindGroup>,
    pub instance: &'render wgpu::Instance,
    pub render_pipeline: &'render wgpu::RenderPipeline,
    pub encoder: &'render wgpu::CommandEncoder,
    pub device: &'render wgpu::Device,
    pub render_pass: &'render mut wgpu::RenderPass<'render>,
}

impl<'r, R> Extract<'r, ObjectRenderContext<'r, R>> for ObjectRef<'r, R> {
    fn extract(c: &mut ObjectRenderContext<'r, R>) -> Self {
        c.object
    }
}

impl<'r, R> Extract<'r, ObjectRenderContext<'r, R>> for &'r wgpu::Buffer {
    fn extract(c: &mut ObjectRenderContext<'r, R>) -> Self {
        c.buffer
    }
}

impl<'r, R> Extract<'r, ObjectRenderContext<'r, R>> for &'r mut wgpu::RenderPass<'r> {
    fn extract(c: &'r mut ObjectRenderContext<'r, R>) -> Self {
        &mut c.render_pass
    }
}

macro_rules! extract_render_context {
    ($($i:ident: $t: ty),*) => {
        $(
            impl<'r, R> Extract<'r, ObjectRenderContext<'r, R>> for &'r $t {
                fn extract(c: &mut ObjectRenderContext<'r, R>) -> Self {
                    c.$i
                }
            }
        )*
    };
}

extract_render_context!(
    camera: Ur<Camera2D, wgpu::BindGroup>,
    instance: wgpu::Instance,
    render_pipeline: wgpu::RenderPipeline,
    encoder: wgpu::CommandEncoder,
    device: wgpu::Device
);

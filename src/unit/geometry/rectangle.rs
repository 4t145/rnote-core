use wgpu::RenderPassDescriptor;

use crate::{
    geometry::{rect::Rect, size::Size},
    unit::{
        style::color::{Color, PURE_TEXTURE_SIZE},

    },
};
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rectangle<S> {
    pub size: Size,
    pub style: S,
}


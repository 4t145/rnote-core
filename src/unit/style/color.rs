#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub const PURE_TEXTURE_SIZE: wgpu::Extent3d = wgpu::Extent3d {
    width: 1,
    height: 1,
    depth_or_array_layers: 1,
};

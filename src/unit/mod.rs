use modifier::Modifier;
use wgpu::Instance;

use crate::geometry::{angle::A16, point::Point, rect::Rect};

pub mod geometry;
pub mod modifier;
pub mod style;




pub struct Config {
    view_formats: Vec<wgpu::TextureFormat>,
}


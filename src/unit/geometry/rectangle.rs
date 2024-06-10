use crate::{
    geometry::{rect::Rect, size::Size},
    unit::{style::color::Color, Object, RenderContext},
};
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rectangle<S> {
    pub size: Size,
    pub style: S,
}

pub trait RectangleStyle {
    fn render(&self, rect: Rect, ctx: &RenderContext);
}

impl<S: RectangleStyle> Object for Rectangle<S> {
    fn render(&self, position: &crate::geometry::point::Point, ctx: &RenderContext) {
        let rect = Rect::new(*position, self.size);
        self.style.render(rect, ctx);
    }
}



impl RectangleStyle for Color {
    fn render(&self, rect: Rect, ctx: &RenderContext) {
        ctx.instance.request_adapter(&wgpu::RequestAdapterOptions::default());
    }
}
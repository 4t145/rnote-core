#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        Size { width, height }
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}
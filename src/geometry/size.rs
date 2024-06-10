#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Size {
        Size { width, height }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}
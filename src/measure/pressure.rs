#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Pressure32(u32);


impl Pressure32 {
    pub fn new(value: u32) -> Pressure32 {
        Pressure32(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}
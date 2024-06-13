/// Angle 16-bit
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct A16(u16);

impl A16 {
    pub fn to_deg(&self) -> f32 {
        const RATIO: f32 = 360.0 / (u16::MAX as f32);
        self.0 as f32 * RATIO
    }

    pub fn to_rad(&self) -> f32 {
        const RATIO: f32 = std::f32::consts::PI / (u16::MAX as f32);
        self.0 as f32 * RATIO
    }
    
    pub fn new(value: u16) -> A16 {
        A16(value)
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

impl std::ops::Add for A16 {
    type Output = A16;

    fn add(self, other: A16) -> A16 {
        A16(self.0.wrapping_add(other.0))
    }
}

impl std::ops::AddAssign for A16 {
    fn add_assign(&mut self, other: A16) {
        self.0 = self.0.wrapping_add(other.0);
    }
}

impl std::ops::Sub for A16 {
    type Output = A16;

    fn sub(self, other: A16) -> A16 {
        A16(self.0.wrapping_sub(other.0))
    }
}

impl std::ops::SubAssign for A16 {
    fn sub_assign(&mut self, other: A16) {
        self.0 = self.0.wrapping_sub(other.0);
    }
}

impl std::ops::Mul<usize> for A16 {
    type Output = A16;

    fn mul(self, other: usize) -> A16 {
        let deg =
            (((self.0 as u32) * (other % u16::MAX as usize) as u32) % (u16::MAX as u32)) as u16;
        A16(deg)
    }
}

impl std::ops::MulAssign<usize> for A16 {
    fn mul_assign(&mut self, other: usize) {
        self.0 = (*self * other).0;
    }
}

use std::ops::Mul;

#[derive(Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

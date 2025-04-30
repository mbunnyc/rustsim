#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn alpha_blended(&self, other: Color) -> Color {
        let a = self.a as f32 / 255.0;
        let b = other.a as f32 / 255.0;
        let r = (self.r as f32 * a + other.r as f32 * b * (1.0 - a)) as u8;
        let g = (self.g as f32 * a + other.g as f32 * b * (1.0 - a)) as u8;
        let b_val = (self.b as f32 * a + other.b as f32 * b * (1.0 - a)) as u8;
        let a = (self.a as f32 * a + other.a as f32 * b * (1.0 - a)) as u8;
        Color::new(r, g, b_val, a)
    }
}

pub const WHITE: Color = Color::new(255, 255, 255, 255);

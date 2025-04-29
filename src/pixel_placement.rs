use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct PixelPlacement {
    pub x: usize,
    pub y: usize,
    pub color: Color,
    pub depth: f32,
}
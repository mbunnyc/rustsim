use crate::{color::Color, pixel_placement::PixelPlacement, pixel_shader::PixelShader, triangle::Triangle};

pub struct EverythingIsRedShader;

impl PixelShader for EverythingIsRedShader {
    fn process(&self, pp: &mut PixelPlacement, _triangle: &Triangle) {
        pp.color = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}
use crate::{color::Color, pixel_placement::PixelPlacement, pixel_shader::PixelShader};

pub struct EverythingIsRedShader;

impl PixelShader for EverythingIsRedShader {
    fn process(&self, pp: &PixelPlacement) -> PixelPlacement {
        PixelPlacement {
            x: pp.x,
            y: pp.y,
            color: Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
        }
    }
}
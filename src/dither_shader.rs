use crate::{color::Color, pixel_placement::PixelPlacement, pixel_shader::PixelShader};

pub struct DitherShader;

impl PixelShader for DitherShader {
    fn process(&self, pp: &PixelPlacement) -> PixelPlacement {
        let alt: bool = if pp.y % 2 == 0 { false } else { true };
        PixelPlacement {
            x: pp.x,
            y: pp.y,
            color: Color {
                r: pp.color.r,
                g: pp.color.g,
                b: pp.color.b,
                a: if alt && pp.x % 2 != 0 { 0 } else { pp.color.a },
            },
        }
    }
}

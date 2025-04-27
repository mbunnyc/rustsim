use crate::{color::Color, pixel_placement::PixelPlacement, pixel_shader::PixelShader, triangle::Triangle};

pub struct EvenLineMissingShader;

impl PixelShader for EvenLineMissingShader {
    fn process(&self, pp: &PixelPlacement, _triangle: &Triangle) -> PixelPlacement {
        PixelPlacement {
            x: pp.x,
            y: pp.y,
            color: Color {
                r: pp.color.r,
                g: pp.color.g,
                b: pp.color.b,
                a: if pp.y % 2 != 0 {
                    pp.color.a
                } else {
                    0
                }
            }            
        }
    }
}
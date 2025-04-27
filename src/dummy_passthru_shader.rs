use crate::{pixel_placement::PixelPlacement, pixel_shader::PixelShader, triangle::Triangle};

pub struct DummyPassthruShader;

impl PixelShader for DummyPassthruShader {
    fn process(&self, pp: &PixelPlacement, _triangle: &Triangle) -> PixelPlacement {
        PixelPlacement {
            x: pp.x,
            y: pp.y,
            color: pp.color,
        }
    }
}
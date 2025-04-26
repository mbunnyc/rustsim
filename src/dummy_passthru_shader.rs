use crate::{pixel_placement::PixelPlacement, pixel_shader::PixelShader};

pub struct DummyPassthruShader;

impl PixelShader for DummyPassthruShader {
    fn process(&self, pp: &PixelPlacement) -> PixelPlacement {
        PixelPlacement {
            x: pp.x,
            y: pp.y,
            color: pp.color,
        }
    }
}
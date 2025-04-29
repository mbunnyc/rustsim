use crate::{pixel_placement::PixelPlacement, pixel_shader::PixelShader, triangle::Triangle};

pub struct DummyPassthruShader;

impl PixelShader for DummyPassthruShader {
    fn process(&self, pp: &mut PixelPlacement, _triangle: &Triangle) {
        
    }
}
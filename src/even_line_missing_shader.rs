use crate::{pixel_placement::PixelPlacement, pixel_shader::PixelShader, triangle::Triangle};

pub struct EvenLineMissingShader;

impl PixelShader for EvenLineMissingShader {
    fn process(&self, pp: &mut PixelPlacement, _triangle: &Triangle) {
        pp.color.a = if pp.y % 2 != 0 {
            pp.color.a
        } else {
            0
        }
    }
}
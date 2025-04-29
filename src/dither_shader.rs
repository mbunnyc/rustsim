use crate::{pixel_placement::PixelPlacement, pixel_shader::PixelShader, triangle::Triangle};

pub struct DitherShader;

impl PixelShader for DitherShader {
    fn process(&self, pp: &mut PixelPlacement, _triangle: &Triangle) {
        let alt: bool = if pp.y % 2 == 0 { false } else { true };
        pp.color.a = if alt && pp.x % 2 != 0 { 0 } else { pp.color.a }
    }
}

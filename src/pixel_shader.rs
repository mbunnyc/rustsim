use crate::pixel_placement::PixelPlacement;

pub trait PixelShader {
    fn process(&self, pp: &PixelPlacement) -> PixelPlacement;
}
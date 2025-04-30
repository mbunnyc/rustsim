use crate::color::Color;

#[derive(Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0, 0, 0, 255); (width * height) as usize],
        }
    }
    
    pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut Color {
        let index = (y * self.width + x) as usize;
        &mut self.pixels[index]
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &Color {
        let index = (y * self.width + x) as usize;
        &self.pixels[index]
    }
}

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

    // pub fn load_from_png(path: &str) -> Self {
    //     let decoder = png::Decoder::new(std::fs::File::open(path).unwrap());
    //     let mut reader = decoder.read_info().unwrap();
    //     let info = reader.info();
    //     let mut buf = vec![0; info.raw_bytes()];
    //     reader.next_frame(&mut buf).unwrap();

    //     let mut pixels = Vec::with_capacity((info.width * info.height) as usize);
    //     for chunk in buf.chunks_exact(4) {
    //         pixels.push(Color::new(chunk[0], chunk[1], chunk[2], chunk[3]));
    //     }

    //     Self {
    //         width: info.width,
    //         height: info.height,
    //         pixels,
    //     }
    // }

    pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut Color {
        let index = (y * self.width + x) as usize;
        &mut self.pixels[index]
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &Color {
        let index = (y * self.width + x) as usize;
        &self.pixels[index]
    }
}

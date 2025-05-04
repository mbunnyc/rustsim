use crate::{
    camera::Camera, color::Color, pixel_placement::PixelPlacement, pixel_shader::{DepthFogShader, PixelShader}, texture::Texture, triangle::Triangle
};

pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 480;
pub const SCREEN_PIXEL_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub struct Screen {
    pub pixels: Box<[Color]>,
    pub depth_buffer: Box<[f32]>,
    pub fog_shader: DepthFogShader,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: vec![Color::new(0, 0, 0, 255); SCREEN_PIXEL_COUNT].into_boxed_slice(),
            depth_buffer: vec![f32::INFINITY; SCREEN_PIXEL_COUNT].into_boxed_slice(),
            fog_shader: DepthFogShader::new(Color::new(255, 255, 255, 80), 12.0, 17.0),
        }
    }

    pub fn clear(&mut self, clear_color: &Color) {
        self.pixels.fill(*clear_color);
        self.depth_buffer.fill(f32::INFINITY);
    }

    pub fn draw_pixel(
        &mut self,
        pp: &PixelPlacement,
        shader: &dyn PixelShader,
        triangle: &Triangle,
    ) {
        if pp.depth < 0.0 {
            return
        }
        let pixel_depth = self.depth_buffer[pp.y * SCREEN_WIDTH + pp.x];
        if pp.depth < pixel_depth {
            let mut pp = pp.clone();
            shader.process(&mut pp, &triangle);
            self.fog_shader.process(&mut pp, &triangle);
            if pp.color.a > 0 {
                self.pixels[pp.y * SCREEN_WIDTH + pp.x] = pp.color;
                self.depth_buffer[pp.y * SCREEN_WIDTH + pp.x] = pp.depth
            }
        }
    }

    pub fn draw_triangle(&mut self, tri: &Triangle, cam: &Camera, shader: &dyn PixelShader, texture: &Texture) {
        tri.project_and_fill(self, cam, shader, texture);
    }
}

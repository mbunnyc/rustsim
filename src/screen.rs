use crate::{camera::Camera, color::Color, pixel_placement::PixelPlacement, pixel_shader::PixelShader, rect::Rect, triangle::Triangle};

pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 480;
pub const SCREEN_PIXEL_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub struct Screen {
    pub pixels: Vec<Color>,
    pub depth_buffer: Vec<f32>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: vec![Color::new(0, 0, 0, 255); SCREEN_PIXEL_COUNT],
            depth_buffer: vec![f32::INFINITY; SCREEN_PIXEL_COUNT],
        }
    }

    pub fn clear(&mut self, clear_color: &Color) {
        self.pixels.fill(*clear_color);

        // Reset depth buffer
        self.depth_buffer.fill(f32::INFINITY);
    }

    pub fn draw_pixel(&mut self, pp: &PixelPlacement, shader: &dyn PixelShader, depth: f32) {        
        if depth < 0.0 {
            return;
        }
        let processed_pp = shader.process(pp);
        // Only draw the pixel if it's closer than what's already there
        let index = pp.y * SCREEN_WIDTH + pp.x;
        if depth < self.depth_buffer[index] {
            if processed_pp.color.a > 0 {
                self.pixels[index] = processed_pp.color;
                self.depth_buffer[index] = depth;
            }
        }
    }

    pub fn draw_triangle(&mut self, tri: &Triangle, cam: &Camera, shader: &dyn PixelShader) {
        tri.project_and_fill(self, cam, shader);
    }

    pub fn copy(&mut self, src: Rect, dst: Rect, to: &mut Screen) {
        // Calculate actual dimensions based on source and destination
        let src_width = src.size.x.floor() as usize;
        let src_height = src.size.y.floor() as usize;
        let src_x = src.pos.x.floor() as usize;
        let src_y = src.pos.y.floor() as usize;
        let dst_x = dst.pos.x.floor() as usize;
        let dst_y = dst.pos.y.floor() as usize;

        // Ensure we don't copy outside screen bounds
        let copy_width = src_width
            .min(SCREEN_WIDTH - dst_x)
            .min(SCREEN_WIDTH - src_x);
        let copy_height = src_height
            .min(SCREEN_HEIGHT - dst_y)
            .min(SCREEN_HEIGHT - src_y);

        // Copy pixels row by row
        for y in 0..copy_height {
            for x in 0..copy_width {
                let src_index = (src_y + y) * SCREEN_WIDTH + (src_x + x);
                let dst_index = (dst_y + y) * SCREEN_WIDTH + (dst_x + x);

                // Bounds check to prevent any possible overflow
                if src_index < SCREEN_PIXEL_COUNT && dst_index < SCREEN_PIXEL_COUNT {
                    to.pixels[dst_index] = self.pixels[src_index];
                }
            }
        }
    }
}

use crate::{camera::Camera, pixel_shader::PixelShader, screen::Screen, triangle::Triangle};

pub struct DrawList {
    triangles: Vec<Triangle>,
}

impl DrawList {
    pub fn new() -> Self {
        Self { triangles: vec![] }
    }
    pub fn add(&mut self, triangles: &Vec<Triangle>) {
        for triangle in triangles {
            self.triangles.push(triangle.clone());
        }
    }

    pub fn draw(&self, screen: &mut Screen, cam: &Camera, shader: &dyn PixelShader) {
        for triangle in &self.triangles {
            screen.draw_triangle(&triangle, &cam, shader);
        }
    }
}

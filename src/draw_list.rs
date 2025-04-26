use crate::{camera::Camera, pixel_shader::PixelShader, screen::Screen, triangle::Triangle};

pub struct DrawList {
    triangles: Vec<Triangle>,
}

impl DrawList {
    fn add(&mut self, triangles: &Vec<Triangle>) {
        for triangle in triangles {
            self.triangles.push(triangle.clone());
        }
    }

    fn draw(&self, screen: &mut Screen, cam: &Camera, shader: &dyn PixelShader) {
        for triangle in &self.triangles {
            screen.draw_triangle(&triangle, &cam, shader);
        }
    }
}

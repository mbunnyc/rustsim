use crate::screen::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::{
    camera::Camera, color::Color, pixel_placement::PixelPlacement, pixel_shader::PixelShader,
    screen::Screen, vec2::Vector2, vec3::Vector3, vertex::Vertex,
};

#[derive(Clone)]
pub struct Triangle {
    pub v1: Vertex,
    pub v2: Vertex,
    pub v3: Vertex,
}

impl Triangle {
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex) -> Self {
        Self { v1, v2, v3 }
    }
        
    /*pub fn z_at(&self, x: f32, y: f32) -> f32 {
        let v1 = &self.v1.pos;
        let v2 = &self.v2.pos;
        let v3 = &self.v3.pos;

        let denom = (v2.z - v3.z) * (v1.x - v3.x) + (v3.x - v2.x) * (v1.z - v3.z);
        let a = ((v2.z - v3.z) * (x - v3.x) + (v3.x - v2.x) * (y - v3.z)) / denom;
        let b = ((v3.z - v1.z) * (x - v3.x) + (v1.x - v3.x) * (y - v3.z)) / denom;
        let c = 1.0 - a - b;

        a * v1.y + b * v2.y + c * v3.y
    }*/
        
    pub fn fill(&self, screen: &mut Screen, shader: &dyn PixelShader) {
        let min_x = self.v1.pos.x.min(self.v2.pos.x).min(self.v3.pos.x).floor() as usize;
        let max_x = self.v1.pos.x.max(self.v2.pos.x).max(self.v3.pos.x).ceil() as usize;
        let min_y = self.v1.pos.y.min(self.v2.pos.y).min(self.v3.pos.y).floor() as usize;
        let max_y = self.v1.pos.y.max(self.v2.pos.y).max(self.v3.pos.y).ceil() as usize;

        let min_x = min_x.clamp(0, SCREEN_WIDTH - 1);
        let max_x = max_x.clamp(0, SCREEN_WIDTH - 1);
        let min_y = min_y.clamp(0, SCREEN_HEIGHT - 1);
        let max_y = max_y.clamp(0, SCREEN_HEIGHT - 1);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let px = x as f32 + 0.5;
                let py = y as f32 + 0.5;

                let (alpha, beta, gamma) = self.barycentric_coords(px, py);

                if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0 {
                    // Interpolate depth using z coordinates
                    let depth =
                        alpha * self.v1.pos.z + beta * self.v2.pos.z + gamma * self.v3.pos.z;

                    let color = self.interpolate_color(alpha, beta, gamma);
                    let pixel = PixelPlacement { x, y, color, depth };
                    screen.draw_pixel(&pixel, shader, &self);
                }
            }
        }
    }

    pub fn barycentric_coords(&self, px: f32, py: f32) -> (f32, f32, f32) {
        let v0 = Vector2::new(self.v2.pos.x - self.v1.pos.x, self.v2.pos.y - self.v1.pos.y);
        let v1 = Vector2::new(self.v3.pos.x - self.v1.pos.x, self.v3.pos.y - self.v1.pos.y);
        let v2 = Vector2::new(px - self.v1.pos.x, py - self.v1.pos.y);

        let d00 = v0.x * v0.x + v0.y * v0.y;
        let d01 = v0.x * v1.x + v0.y * v1.y;
        let d11 = v1.x * v1.x + v1.y * v1.y;
        let d20 = v2.x * v0.x + v2.y * v0.y;
        let d21 = v2.x * v1.x + v2.y * v1.y;

        let denom = d00 * d11 - d01 * d01;

        let beta = (d11 * d20 - d01 * d21) / denom;
        let gamma = (d00 * d21 - d01 * d20) / denom;
        let alpha = 1.0 - beta - gamma;

        (alpha, beta, gamma)
    }

    fn interpolate_color(&self, alpha: f32, beta: f32, gamma: f32) -> Color {
        let r = (self.v1.color.r as f32 * alpha
            + self.v2.color.r as f32 * beta
            + self.v3.color.r as f32 * gamma) as u8;

        let g = (self.v1.color.g as f32 * alpha
            + self.v2.color.g as f32 * beta
            + self.v3.color.g as f32 * gamma) as u8;

        let b = (self.v1.color.b as f32 * alpha
            + self.v2.color.b as f32 * beta
            + self.v3.color.b as f32 * gamma) as u8;

        Color { r, g, b, a: 255 }
    }

    pub fn project_and_fill(&self, screen: &mut Screen, camera: &Camera, shader: &dyn PixelShader) {
        let projected_triangle = self.with_applied_perspective(camera, SCREEN_WIDTH, SCREEN_HEIGHT);
        if let Some(triangle) = projected_triangle {
            triangle.fill(screen, shader);
        }
    }

    fn with_applied_perspective(
        &self,
        camera: &Camera,
        screen_width: usize,
        screen_height: usize,
    ) -> Option<Triangle> {
        let forward = Vector3::normalize_v(&(camera.pointing_at - camera.pos));
        let right = Vector3::normalize_v(&forward.cross(&Vector3::new(0.0, 1.0, 0.0)));
        let up = Vector3::cross(&right, &forward);

        let aspect_ratio = screen_width as f32 / screen_height as f32;
        let fov_radians = camera.fov.to_radians();
        let tan_half_fov = (fov_radians / 2.0).tan();

        const NEAR_PLANE: f32 = 0.01;
        const FAR_PLANE: f32 = 60.0;

        let project_vertex = |vertex: &Vertex| -> Option<Vertex> {
            let relative_pos = Vector3::subtract(&vertex.pos, &camera.pos);

            let camera_x = Vector3::dot(&relative_pos, &right);
            let camera_y = Vector3::dot(&relative_pos, &up);
            let camera_z = Vector3::dot(&relative_pos, &forward);

            if camera_z < NEAR_PLANE || camera_z > FAR_PLANE {
                return None;
            }

            let ndc_x = camera_x / (tan_half_fov * camera_z);
            let ndc_y = camera_y / (tan_half_fov * camera_z / aspect_ratio);

            let screen_x = (ndc_x * 0.5 + 0.5) * screen_width as f32;
            let screen_y = (1.0 - (ndc_y * 0.5 + 0.5)) * screen_height as f32;

            Some(Vertex::new(
                &Vector3::new(screen_x, screen_y, camera_z),
                &vertex.texture_coord,
                &vertex.color,
            ))
        };

        let projected_v1 = project_vertex(&self.v1);
        let projected_v2 = project_vertex(&self.v2);
        let projected_v3 = project_vertex(&self.v3);
        if projected_v1.is_none() || projected_v2.is_none() || projected_v3.is_none() {
            return None;
        }
        let projected_v1 = projected_v1.unwrap();
        let projected_v2 = projected_v2.unwrap();
        let projected_v3 = projected_v3.unwrap();

        Some(Triangle::new(projected_v1, projected_v2, projected_v3))
    }
}

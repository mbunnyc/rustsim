use crate::{vec2::Vector2, vec3::Vector3};

pub struct Camera {
    pub fov: f32,
    pub pos: Vector3,
    pub pointing_at: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            fov: 45.0,
            pos: Vector3::new(7.0, 5.0, 8.0),
            pointing_at: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn first_person_look(&mut self, mouse_delta: &Vector2) {
        let mouse_delta = mouse_delta.clone() * Vector2 { x: 0.001, y: -0.001 };
        let mut direction = self.pointing_at - self.pos;
        let right = direction.cross(&Vector3::new(0.0, 1.0, 0.0)).normalizeV();
        let up = right.cross(&direction).normalize();
        
        // Rotate around Y axis (yaw)
        let cos_yaw = mouse_delta.x.cos();
        let sin_yaw = mouse_delta.x.sin();
        direction = Vector3::new(
            direction.x * cos_yaw - direction.z * sin_yaw,
            direction.y,
            direction.x * sin_yaw + direction.z * cos_yaw
        );

        // Rotate around right axis (pitch)
        let cos_pitch = mouse_delta.y.cos();
        let sin_pitch = mouse_delta.y.sin();
        direction = Vector3::new(
            direction.x,
            direction.y * cos_pitch - direction.z * sin_pitch,
            direction.y * sin_pitch + direction.z * cos_pitch
        );

        self.pointing_at = self.pos + direction;
    }
}

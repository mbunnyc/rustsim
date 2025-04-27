use crate::vec3::Vector3;

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
}

use crate::{color::Color, vec2::Vector2, vec3::Vector3};

pub struct Vertex {
    pub pos: Vector3,
    pub texture_coord: Vector2,
    pub color: Color,
}

impl Vertex {
    pub fn new(pos: &Vector3, texture_coord: &Vector2, color: &Color) -> Self {
        Self {
            pos: Vector3 {
                x: pos.x,
                y: pos.y,
                z: pos.z,
            },
            texture_coord: Vector2 {
                x: texture_coord.x,
                y: texture_coord.y,
            },
            color: Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            },
        }
    }
}

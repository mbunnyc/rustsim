use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::subtract(&self, &rhs)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::add(&self, &rhs)
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn add(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3::new(a.x + b.x, a.y + b.y, a.z + b.z)
    }

    pub fn subtract(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3::new(a.x - b.x, a.y - b.y, a.z - b.z)
    }

    pub fn cross2(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn cross(&self, b: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }

    pub fn normalize(&mut self) -> Vector3 {
        let len = self.length();
        if len > 0.0001 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn dot(a: &Vector3, b: &Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalizeV(&self) -> Vector3 {
        let len = self.length();
        if len > 0.0001 {
            Vector3::new(self.x / len, self.y / len, self.z / len)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
}

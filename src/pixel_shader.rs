use crate::{color::Color, pixel_placement::PixelPlacement, triangle::Triangle};

pub trait PixelShader {
    fn process(&self, pp: &PixelPlacement, triangle: &Triangle) -> PixelPlacement;
}

pub struct SuperShader {
    child_shaders: Vec<Box<dyn PixelShader>>,
}

impl SuperShader {
    pub fn new(child_shaders: Vec<Box<dyn PixelShader>>) -> Self {
        SuperShader { child_shaders }
    }
}

impl PixelShader for SuperShader {
    fn process(&self, pp: &PixelPlacement, triangle: &Triangle) -> PixelPlacement {
        let mut result = pp.clone();
        for shader in &self.child_shaders {
            result = shader.process(&result, triangle);
        }
        result
    }
}

pub struct TexturedRainbowShader {
    time: f32,
    speed: f32,
}

impl TexturedRainbowShader {
    pub fn new(speed: f32) -> Self {
        TexturedRainbowShader {
            time: 0.0,
            speed,
        }
    }
}

impl PixelShader for TexturedRainbowShader {
    fn process(&self, pp: &PixelPlacement, triangle: &Triangle) -> PixelPlacement {
        // Get barycentric coordinates for texture mapping
        let (u, v, _xx) = triangle.barycentric_coords(pp.x as f32, pp.y as f32);
        
        // Create a rainbow effect using the texture coordinates
        let hue = ((u + v) * 2.0 + self.time * self.speed) % 1.0;
        
        // Convert HSV to RGB (with V=1, S=1)
        let h = hue * 6.0;
        let i = h.floor();
        let f = h - i;
        let p = 0.0;
        let q = 1.0 - f;
        let t = f;

        let (r, g, b) = match i as i32 % 6 {
            0 => (1.0, t, p),
            1 => (q, 1.0, p),
            2 => (p, 1.0, t),
            3 => (p, q, 1.0),
            4 => (t, p, 1.0),
            5 => (1.0, p, q),
            _ => (1.0, 1.0, 1.0),
        };

        PixelPlacement {
            x: pp.x,
            y: pp.y,
            color: Color {
                r: (r * 255.0) as u8,
                g: (g * 255.0) as u8,
                b: (b * 255.0) as u8,
                a: pp.color.a,
            },
        }
    }
}

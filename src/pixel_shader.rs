use crate::{color::Color, pixel_placement::PixelPlacement, triangle::Triangle};

pub trait PixelShader {
    fn process(&self, pp: &mut PixelPlacement, triangle: &Triangle);
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
    fn process(&self, pp: &mut PixelPlacement, triangle: &Triangle) {
        let mut result = pp.clone();
        for shader in &self.child_shaders {
            shader.process(&mut result, triangle);
        }
    }
}

pub struct TexturedRainbowShader {
    pub time: f32,
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
    fn process(&self, pp: &mut PixelPlacement, triangle: &Triangle) {
        if pp.color.a == 0 {
            return;
        }
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

        pp.color = Color {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: pp.color.a,
        }
    }
}

pub struct DepthFogShader {
    fog_color: Color,
    fog_start: f32,
    fog_end: f32,
}

impl DepthFogShader {
    pub fn new(fog_color: Color, fog_start: f32, fog_end: f32) -> Self {
        DepthFogShader {
            fog_color,
            fog_start,
            fog_end,
        }
    }
}

impl PixelShader for DepthFogShader {
    fn process(&self, pp: &mut PixelPlacement, _triangle: &Triangle) {
        if pp.color.a == 0 {
            return;
        }

        // Calculate fog factor based on depth
        let fog_factor = ((pp.depth - self.fog_start) / (self.fog_end - self.fog_start))
            .clamp(0.0, 1.0);

        // Linearly interpolate between pixel color and fog color
        pp.color = Color {
            r: ((1.0 - fog_factor) * pp.color.r as f32 + fog_factor * self.fog_color.r as f32) as u8,
            g: ((1.0 - fog_factor) * pp.color.g as f32 + fog_factor * self.fog_color.g as f32) as u8,
            b: ((1.0 - fog_factor) * pp.color.b as f32 + fog_factor * self.fog_color.b as f32) as u8,
            a: ((1.0 - fog_factor) * pp.color.a as f32 + fog_factor * self.fog_color.a as f32) as u8,
        };
    }
}
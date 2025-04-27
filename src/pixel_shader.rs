use crate::{color::Color, pixel_placement::PixelPlacement};

pub trait PixelShader {
    fn process(&self, pp: &PixelPlacement) -> PixelPlacement;
}

pub struct RainbowShader {
    time: f32,
    speed: f32,
}

impl RainbowShader {
    pub fn new(speed: f32) -> Self {
        RainbowShader {
            time: 0.0,
            speed,
        }
    }
}

impl PixelShader for RainbowShader {
    fn process(&self, pp: &PixelPlacement) -> PixelPlacement {
        // Create a rainbow effect by using position and time
        let hue = (pp.x as f32 * 0.01 + pp.y as f32 * 0.01 + self.time) % 1.0;
        
        // Convert HSV to RGB
        let h = hue * 6.0;
        let i = h.floor();
        let f = h - i;
        let p = 0.0;
        let q = 1.0 - f;
        let t = f;

        let (r, g, b) = match i as i32 {
            0 => (1.0, t, p),
            1 => (q, 1.0, p),
            2 => (p, 1.0, t),
            3 => (p, q, 1.0),
            4 => (t, p, 1.0),
            _ => (1.0, p, q),
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
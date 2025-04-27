use crate::{
    camera::Camera, color::Color, dither_shader::DitherShader, draw_list::DrawList, dummy_passthru_shader::DummyPassthruShader, even_line_missing_shader::EvenLineMissingShader, game::Game, key_event::KeyEvent, keycode::KeyCode, mouse_event::MouseEvent, pixel_shader::RainbowShader, screen::Screen, triangle::Triangle, vec2::Vector2, vec3::Vector3
};

pub struct TestGame {
    pub cam: Camera,
    pub up_key_held: bool,
    pub down_key_held: bool,
    pub left_key_held: bool,
    pub right_key_held: bool,
    pub space_key_held: bool,
    pub shift_key_held: bool,
    pub w_key_held: bool,
    pub s_key_held: bool,
    pub a_key_held: bool,
    pub d_key_held: bool,
}

impl TestGame {
    pub fn new() -> Self {
        TestGame {
            cam: Camera::new(),
            up_key_held: false,
            down_key_held: false,
            left_key_held: false,
            right_key_held: false,
            space_key_held: false,
            shift_key_held: false,
            w_key_held: false,
            s_key_held: false,
            a_key_held: false,
            d_key_held: false,
        }
    }
}

impl Game for TestGame {
    fn key_event(&mut self, key_ev: &KeyEvent) {
        let state = matches!(key_ev, KeyEvent::Pressed { .. });
        let key_held = match key_ev {
            KeyEvent::Pressed { key } | KeyEvent::Released { key } => match key {
                KeyCode::Up => &mut self.up_key_held,
                KeyCode::Down => &mut self.down_key_held,
                KeyCode::Left => &mut self.left_key_held,
                KeyCode::Right => &mut self.right_key_held,
                KeyCode::Space => &mut self.space_key_held,
                KeyCode::LShift => &mut self.shift_key_held,
                KeyCode::W => &mut self.w_key_held,
                KeyCode::S => &mut self.s_key_held,
                KeyCode::A => &mut self.a_key_held,
                KeyCode::D => &mut self.d_key_held,
                _ => return,
            },
        };
        *key_held = state;
    }

    fn update_tick(&mut self) {
        let amt = 0.1;
        if self.up_key_held {
            self.cam.pos.z += amt;
        }
        if self.down_key_held {
            self.cam.pos.z -= amt;
        }
        if self.left_key_held {
            self.cam.pos.x -= amt;
        }
        if self.right_key_held {
            self.cam.pos.x += amt;
        }
        if self.shift_key_held {
            self.cam.pos.y -= amt;
        }
        if self.space_key_held {
            self.cam.pos.y += amt;
        }
    }

    fn render_tick(&self, screen: &mut Screen) {
        screen.clear(&Color::new(0, 0, 0, 0));
        let sh = DummyPassthruShader;
        // Create a 2x3 floor at height 0
        let floor_tris = Triangle::create_floor_rect(
            Vector2::new(-1.0, -1.5),
            Vector2::new(1.0, 1.5),
            0.0,
            Color::new(128, 128, 0, 255),
        );

        let floor2_tris = Triangle::create_floor_rect(
            Vector2::new(-1.0, -1.5),
            Vector2::new(1.0, 1.5),
            1.5,
            Color::new(10, 128, 50, 255),
        );

        let wall1_tris = Triangle::create_wall(
            &Vector3 {
                x: -1.0,
                y: 0.0,
                z: -1.5,
            },
            2.0,
            1.5,
            0.0,
            &Color::new(0, 0, 20, 255),
        );

        let mut draw_list = DrawList::new();

        let wall2_tris = Triangle::create_wall(
            &Vector3 {
                x: -1.0,
                y: 0.0,
                z: 1.5,
            },
            2.0,
            1.5,
            0.0,
            &Color::new(0, 0, 20, 255),
        );

        draw_list.add(&wall2_tris);

        let elm_sh = EvenLineMissingShader;

        for triangle in floor2_tris {
            screen.draw_triangle(&triangle, &self.cam, &elm_sh);
        }

        // Draw both triangles that make up the floor
        for triangle in floor_tris {
            screen.draw_triangle(&triangle, &self.cam, &sh);
        }

        for triangle in wall1_tris {
            screen.draw_triangle(&triangle, &self.cam, &elm_sh);
        }

        let dith_sh = RainbowShader::new(5.0);

        draw_list.draw(screen, &self.cam, &dith_sh);
    }

    fn mouse_event(&mut self, mouse_ev: &MouseEvent) {}
}

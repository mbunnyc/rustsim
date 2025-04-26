use crate::{
    camera::Camera, color::Color, dummy_passthru_shader::DummyPassthruShader,
    even_line_missing_shader::EvenLineMissingShader, game::Game, key_event::KeyEvent,
    keycode::KeyCode, mouse_event::MouseEvent, screen::Screen, triangle::Triangle, vec2::Vector2,
    vec3::Vector3,
};

pub struct TestGame {
    pub cam: Camera,
    pub up_key_held: bool,
    pub down_key_held: bool,
    pub left_key_held: bool,
    pub right_key_held: bool,
}

impl TestGame {
    pub fn new() -> Self {
        TestGame {
            cam: Camera::new(),
            up_key_held: false,
            down_key_held: false,
            left_key_held: false,
            right_key_held: false,
        }
    }
}

impl Game for TestGame {
    fn update_tick(&mut self) {
        let amt = 0.1;
        if self.up_key_held {
            self.cam.pos.y += amt;
        }
        if self.down_key_held {
            self.cam.pos.y -= amt;
        }
        if self.left_key_held {
            self.cam.pos.x -= amt;
        }
        if self.right_key_held {
            self.cam.pos.x += amt;
        }
    }
    fn render_tick(&self, screen: &mut Screen) {
        screen.clear(&Color::new(0, 255, 0, 0));
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
                x: 0.0,
                y: -5.0,
                z: 0.0,
            },
            4.0,
            2.0,
            0.0,
            &Color::new(0, 255, 180, 80),
        );


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
    }
    
    fn key_event(&mut self, key_ev: &KeyEvent) {
        match key_ev {
            KeyEvent::Pressed { key } => match key {
                KeyCode::Up => {
                    self.up_key_held = true;
                }
                KeyCode::Down => {
                    self.down_key_held = true;
                    //self.cam.pos.y -= 1.0;
                },
                KeyCode::Left => {
                    self.left_key_held = true;
                    //self.cam.pos.x -= 1.0;
                },
                KeyCode::Right => {
                    self.right_key_held = true;
                    //self.cam.pos.x += 1.0;
                },
                _ => {}
            },
            KeyEvent::Released { key } => match key {
                KeyCode::Up => {
                    self.up_key_held = false;
                }
                KeyCode::Down => {
                    self.down_key_held = false;
                },
                KeyCode::Left => {
                    self.left_key_held = false;
                },
                KeyCode::Right => {
                    self.right_key_held = false;
                },
                _ => {}
            }
        }
    }
    fn mouse_event(&mut self, mouse_ev: &MouseEvent) {}
}

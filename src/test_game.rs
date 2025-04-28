use crate::{
    camera::Camera, color::Color, draw_list::DrawList, dummy_passthru_shader::DummyPassthruShader, even_line_missing_shader::EvenLineMissingShader, game::Game, input_handler::InputHandler, key_event::KeyEvent, mouse_event::MouseEvent, pixel_shader::{SuperShader, TexturedRainbowShader}, screen::Screen, triangle::Triangle, vec2::Vector2, vec3::Vector3
};

pub struct TestGame {
    pub cam: Camera,
    pub input: InputHandler,
    dith_sh: TexturedRainbowShader,
}

impl TestGame {
    pub fn new() -> Self {
        TestGame {
            cam: Camera::new(),
            input: InputHandler::new(),
            dith_sh: TexturedRainbowShader::new(5.0),
        }
    }
}

impl Game for TestGame {
    fn update_tick(&mut self) {
        self.input.new_frame();

        let amt = 0.1;
        if self.input.up.pressed {
            self.cam.pos.z += amt;
        }
        if self.input.down.pressed {
            self.cam.pos.z -= amt;
        }
        if self.input.left.pressed {
            self.cam.pos.x -= amt;
        }
        if self.input.right.pressed
        /*|| self.mouse_left_click*/
        {
            self.cam.pos.x += amt;
        }
        if self.input.shift.pressed {
            self.cam.pos.y -= amt;
        }
        if self.input.space.pressed {
            self.cam.pos.y += amt;
        }       
        
        self.dith_sh.time += 0.01;
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

        //let elm_sh = EvenLineMissingShader;

        let super_shader = SuperShader::new(vec![
            Box::new(EvenLineMissingShader),
            //Box::new(TexturedRainbowShader::new(5.0)),
        ]);

        let mut elm_dl = DrawList::new();
        elm_dl.add(&floor2_tris);
        elm_dl.add(&wall1_tris);

        elm_dl.draw(screen, &self.cam, &super_shader);

        let mut sh_dl = DrawList::new();
        sh_dl.add(&floor_tris);
        sh_dl.draw(screen, &self.cam, &sh);

        draw_list.draw(screen, &self.cam, &self.dith_sh);
    }

    fn key_event(&mut self, key_ev: &KeyEvent) {
        self.input.handle_key_event(&key_ev);
    }
    
    fn mouse_event(&mut self, mouse_ev: &MouseEvent) {
        self.input.handle_mouse_event(&mouse_ev);
        

        if self.input.mouse_left.pressed {
            self.cam.first_person_look(&self.input.mouse_delta)
        }
    }
}

use crate::{
    camera::Camera,
    color::Color,
    dither_shader::DitherShader,
    draw_list::DrawList,
    dummy_passthru_shader::DummyPassthruShader,
    even_line_missing_shader::EvenLineMissingShader,
    game::Game,
    input_handler::InputHandler,
    key_event::KeyEvent,
    mouse_event::MouseEvent,
    pixel_shader::{SuperShader, TexturedRainbowShader},
    screen::{Screen, SCREEN_HEIGHT, SCREEN_WIDTH},
    triangle::Triangle,
    triangle_gen::TriangleGen,
    vec2::Vector2,
    vec3::Vector3
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
        screen.clear(&Color::new(0, 190, 255, 255));
        let sh = DummyPassthruShader;

        let floor_tris = TriangleGen::create_floor_rect(
            Vector2::new(-1.0, -1.5),
            Vector2::new(1.0, 1.5),
            0.0,
            Color::new(128, 128, 0, 255),
        );

        let floor2_tris = TriangleGen::create_floor_rect(
            Vector2::new(-1.0, -1.5),
            Vector2::new(1.0, 1.5),
            1.5,
            Color::new(10, 128, 50, 255),
        );

        let wall1_tris = TriangleGen::create_wall(
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

        let wall2_tris = TriangleGen::create_wall(
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

        let super_shader = SuperShader::new(vec![
            Box::new(EvenLineMissingShader),
            Box::new(DitherShader),
        ]);

        let mut elm_dl = DrawList::new();
        elm_dl.add(&floor2_tris);
        elm_dl.add(&wall1_tris);

        elm_dl.draw(screen, &self.cam, &super_shader);

        let mut sh_dl = DrawList::new();
        sh_dl.add(&floor_tris);
        sh_dl.draw(screen, &self.cam, &sh);

        draw_list.draw(screen, &self.cam, &self.dith_sh);

        // Add this to the render_tick method
        let big_floor = TriangleGen::create_floor_rect(
            Vector2::new(-50.0, -50.0),
            Vector2::new(100.0, 100.0), 
            -5.0,
            Color::new(0, 0, 0, 255)
        );

        let mut big_floor_dl = DrawList::new();
        big_floor_dl.add(&big_floor);
        big_floor_dl.draw(screen, &self.cam, &sh);    

        let start = Vector3::new(0.0, 10.0, 0.0);
        let end = Vector3::new(0.0, 0.0, 0.0);
        let line_tris = TriangleGen::create_3d_line(
            &start,
            &end,
            &self.cam,
            &Color::new(255, 0, 0, 255),  // Start color (red)
            &Color::new(255, 0, 0, 255),  // End color (red)
            0.1  // Line thickness
        );

        // Create a draw list for the line
        let mut line_dl = DrawList::new();
        line_dl.add(&line_tris);
        line_dl.draw(screen, &self.cam, &sh);
}

    fn key_event(&mut self, key_ev: &KeyEvent) {
        self.input.handle_key_event(&key_ev);

        
    }
    
    fn mouse_event(&mut self, mouse_ev: &MouseEvent) {
        self.input.handle_mouse_event(&mouse_ev);

        let screen_res = Vector2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);
        /*if self.input.mouse_left.pressed {
            self.cam.first_person_look(&self.input.mouse_delta, &screen_res)
        }*/
        if self.input.mouse_left/*_right*/.pressed {
            self.cam.drag_move(&self.input.mouse_pos_last, &self.input.mouse_pos, &screen_res)
        }

        if self.input.scroll_y != 0 {
            self.cam.zoom(self.input.scroll_y as f32)
        }
    }
}

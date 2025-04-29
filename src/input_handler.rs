use crate::{key_event::{KeyEvent, KeyState}, keycode::KeyCode, mouse_button::MouseButton, mouse_event::MouseEvent, screen::{SCREEN_HEIGHT, SCREEN_WIDTH}, vec2::Vector2};

pub struct InputHandler {    
    pub up: KeyState,
    pub down: KeyState,
    pub left: KeyState,
    pub right: KeyState,
    
    pub space: KeyState,
    
    pub shift: KeyState,
    
    pub w: KeyState,
    pub s: KeyState,
    pub a: KeyState,
    pub d: KeyState,
    
    pub mouse_left: KeyState,
    pub mouse_middle: KeyState,
    pub mouse_right: KeyState,
    
    pub mouse_pos: Vector2,
    pub mouse_pos_last: Vector2,
    pub mouse_delta: Vector2,
    pub mouse_pos_on_click: Vector2,
}

//get more useful input from events
impl InputHandler {
    pub fn new() -> Self {
        Self {
            up: KeyState::new(),
            down: KeyState::new(),
            left: KeyState::new(),
            right: KeyState::new(),
            space: KeyState::new(),
            shift: KeyState::new(),
            w: KeyState::new(),
            a: KeyState::new(),
            s: KeyState::new(),
            d: KeyState::new(),
            mouse_left: KeyState::new(),
            mouse_middle: KeyState::new(),
            mouse_right: KeyState::new(),
            mouse_pos: Vector2::new(0.0, 0.0),
            mouse_pos_last: Vector2::new(0.0, 0.0),
            mouse_delta: Vector2::new(0.0, 0.0),
            mouse_pos_on_click: Vector2::new(0.0, 0.0),
        }
    }

    pub fn new_frame(&mut self) {
        self.mouse_left.click = false;
        self.mouse_right.click = false;
        self.mouse_middle.click = false;
        self.mouse_left.released = false;
        self.mouse_right.released = false;
        self.mouse_middle.released = false;
        self.mouse_delta.x = self.mouse_pos.x - self.mouse_pos_last.x;
        self.mouse_delta.y = self.mouse_pos.y - self.mouse_pos_last.y;
    }

    pub fn handle_key_event(&mut self, key_ev: &KeyEvent) {
        let state = matches!(key_ev, KeyEvent::Pressed { .. });
        let key_held = match key_ev {
            KeyEvent::Pressed { key } | KeyEvent::Released { key } => match key {
                KeyCode::Up => &mut self.up.pressed,
                KeyCode::Down => &mut self.down.pressed,
                KeyCode::Left => &mut self.left.pressed,
                KeyCode::Right => &mut self.right.pressed,
                KeyCode::Space => &mut self.space.pressed,
                KeyCode::LShift => &mut self.shift.pressed,
                KeyCode::W => &mut self.w.pressed,
                KeyCode::S => &mut self.s.pressed,
                KeyCode::A => &mut self.a.pressed,
                KeyCode::D => &mut self.d.pressed,
                _ => return,
            },
        };
        *key_held = state;
    }

    pub fn handle_mouse_event(&mut self, mouse_ev: &MouseEvent) {
        match mouse_ev {
            MouseEvent::ButtonDown { x, y, btn } => {
                
                self.mouse_pos_last.x = self.mouse_pos.x;
                self.mouse_pos_last.y = self.mouse_pos.y;
                self.mouse_pos.x = (*x as f32).clamp(0.0, SCREEN_WIDTH as f32);
                self.mouse_pos.y = (*y as f32).clamp(0.0, SCREEN_HEIGHT as f32);
                match btn {
                    MouseButton::Left => {
                        if !self.mouse_left.pressed {
                            self.mouse_left.click = true;
                            self.mouse_pos_on_click.x = self.mouse_pos.x;
                            self.mouse_pos_on_click.y = self.mouse_pos.y;
                        }
                        self.mouse_left.pressed = true
                    }
                    MouseButton::Middle => {
                        if !self.mouse_middle.pressed {
                            self.mouse_middle.click = true
                        }
                        self.mouse_middle.pressed = true
                    }
                    MouseButton::Right => {
                        if !self.mouse_right.pressed {
                            self.mouse_right.click = true
                        }
                        self.mouse_right.pressed = true
                    }
                    _ => {}
                }
            }
            MouseEvent::ButtonRelease { x, y, btn } => {
                
                self.mouse_pos_last.x = self.mouse_pos.x;
                self.mouse_pos_last.y = self.mouse_pos.y;
                self.mouse_pos.x = (*x as f32).clamp(0.0, SCREEN_WIDTH as f32);
                self.mouse_pos.y = (*y as f32).clamp(0.0, SCREEN_HEIGHT as f32);
                match btn {
                    MouseButton::Left => {
                        if self.mouse_left.pressed {
                            self.mouse_left.released = true
                        }
                        self.mouse_left.pressed = false
                    }
                    MouseButton::Middle => {
                        if self.mouse_middle.pressed {
                            self.mouse_middle.released = true
                        }
                        self.mouse_middle.pressed = false
                    }
                    MouseButton::Right => {
                        if self.mouse_right.pressed {
                            self.mouse_right.released = true
                        }
                        self.mouse_right.pressed = false
                    }
                    _ => {}
                }
            }
            MouseEvent::NewPosition { x, y } => {
                self.mouse_pos_last.x = self.mouse_pos.x;
                self.mouse_pos_last.y = self.mouse_pos.y;
                self.mouse_pos.x = (*x as f32).clamp(0.0, SCREEN_WIDTH as f32);
                self.mouse_pos.y = (*y as f32).clamp(0.0, SCREEN_HEIGHT as f32);
            }
        }
    }
}
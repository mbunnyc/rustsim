use crate::key_event::KeyState;


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
    
    pub mouse_x: u32,
    pub mouse_x_last: u32,
    
    pub mouse_y: u32,
    pub mouse_y_last: u32,
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
            mouse_x: 0,
            mouse_x_last: 0,
            mouse_y: 0,
            mouse_y_last: 0,
        }
    }

    pub fn new_frame(&mut self) {
        self.mouse_left.click = false;
        self.mouse_right.click = false;
        self.mouse_middle.click = false;
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
                self.mouse_x_last = self.mouse_x;
                self.mouse_y_last = self.mouse_y;
                self.mouse_x = *x as u32;
                self.mouse_y = *y as u32;
                match btn {
                    MouseButton::Left => {
                        if !self.mouse_left.pressed {
                            self.mouse_left.click = true
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
                self.mouse_x_last = self.mouse_x;
                self.mouse_y_last = self.mouse_y;
                self.mouse_x = *x as u32;
                self.mouse_y = *y as u32;
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
                self.mouse_x_last = self.mouse_x;
                self.mouse_y_last = self.mouse_y;
                self.mouse_x = *x;
                self.mouse_y = *y;
            }
        }
    }
}
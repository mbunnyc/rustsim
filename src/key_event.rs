use crate::{keycode::KeyCode, mouse_button::MouseButton, mouse_event::MouseEvent};

#[derive(Debug)]
pub enum KeyEvent {
    Pressed { key: KeyCode },
    Released { key: KeyCode },
}

pub struct InputHandler {    
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
    pub mouse_x: u32,
    pub mouse_y: u32,
    pub mouse_left_down: bool,
    pub mouse_left_click: bool,
    pub mouse_right_down: bool,
    pub mouse_right_click: bool,
    pub mouse_middle_down: bool,
    pub mouse_middle_click: bool,
}

//get more useful input from events
impl InputHandler {
    pub fn new() -> Self {
        Self {
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
            mouse_x: 0,
            mouse_y: 0,
            mouse_left_down: false,
            mouse_left_click: false,
            mouse_right_down: false,
            mouse_right_click: false,
            mouse_middle_down: false,
            mouse_middle_click: false,
        }
    }

    pub fn new_frame(&mut self) {
        self.mouse_left_click = false;
        self.mouse_right_click = false;
        self.mouse_middle_click = false;
    }

    pub fn handle_key_event(&mut self, key_ev: &KeyEvent) {
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

    pub fn handle_mouse_event(&mut self, mouse_ev: &MouseEvent) {
        match mouse_ev {
            MouseEvent::ButtonDown { x, y, btn } => {
                self.mouse_x = *x as u32;
                self.mouse_y = *y as u32;
                match btn {
                    MouseButton::Left => {
                        if !self.mouse_left_down {
                            self.mouse_left_click = true
                        }
                        self.mouse_left_down = true
                    }
                    MouseButton::Right => {
                        if !self.mouse_right_down {
                            self.mouse_right_click = true
                        }
                        self.mouse_right_down = true
                    }
                    MouseButton::Middle => {
                        if !self.mouse_middle_down {
                            self.mouse_middle_click = true
                        }
                        self.mouse_middle_down = true
                    }
                    _ => {}
                }
            }
            MouseEvent::ButtonRelease { x, y, btn } => {
                self.mouse_x = *x as u32;
                self.mouse_y = *y as u32;
                match btn {
                    MouseButton::Left => self.mouse_left_down = false,
                    MouseButton::Right => self.mouse_right_down = false,
                    MouseButton::Middle => self.mouse_middle_down = false,
                    _ => {}
                }
            }
            MouseEvent::NewPosition { x, y } => {
                self.mouse_x = *x;
                self.mouse_y = *y;
            }
        }
    }
}
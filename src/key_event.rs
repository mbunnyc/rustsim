use crate::{keycode::KeyCode, mouse_button::MouseButton, mouse_event::MouseEvent};

#[derive(Debug)]
pub enum KeyEvent {
    Pressed { key: KeyCode },
    Released { key: KeyCode },
}

//mouse or kb
pub struct KeyState {
    pub click: bool,
    pub pressed: bool,
    pub released: bool,
}

impl KeyState {
    pub fn new() -> Self {
        Self {
            click: false,
            pressed: false,
            released: false,
        }
    }
}

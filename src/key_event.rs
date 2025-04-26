use crate::keycode::KeyCode;

#[derive(Debug)]
pub enum KeyEvent {
    Pressed { key: KeyCode },
    Released { key: KeyCode },
}

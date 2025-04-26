use crate::{key_event::KeyEvent, mouse_event::MouseEvent, screen::Screen};

pub trait Game {
    fn update_tick(&mut self);
    fn render_tick(&self, screen: &mut Screen);
    fn key_event(&mut self, key_ev: &KeyEvent);
    fn mouse_event(&mut self, mouse_ev: &MouseEvent);
}

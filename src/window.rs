use crate::{game::Game, screen::Screen};

pub trait Window {
    fn start(&self, screen: &mut Screen, game: &mut dyn Game);
}

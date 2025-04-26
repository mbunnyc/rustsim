use camera::Camera;
use game::Game;
use screen::Screen;
use sdl2win::SDL2Window;
use test_game::TestGame;
use window::Window;

mod color;
mod camera;
mod dummy_passthru_shader;
mod even_line_missing_shader;
mod everything_is_red_shader;
mod key_event;
mod keycode;
mod mouse_button;
mod mouse_event;
mod rect;
mod screen;
mod triangle;
mod vec2;
mod vec3;
mod vertex;
mod window;
mod game;
mod test_game;
mod sdl2win;
mod pixel_placement;
mod pixel_shader;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;
const SCREEN_PIXEL_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

fn start_game(screen: &mut Screen, win: &dyn Window, game: &mut dyn Game) {
    win.start(screen, game);
}

fn main() {
    let mut screen = Screen::new();
    let win = SDL2Window;
    let mut game = TestGame { cam: Camera::new() };
    start_game(&mut screen, &win, &mut game);
}

use camera::Camera;
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

fn main() {
    let mut screen = Screen::new();
    let win = SDL2Window;
    let mut game = TestGame { cam: Camera::new() };
    win.start(&mut screen, &mut game);
}

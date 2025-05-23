//#![allow(warnings)]

use screen::Screen;
use sdl2win::SDL2Window;
use nameless_3d_game::Nameless3DThing;
use window::Window;

mod triangle_gen;
mod input_handler;
mod texture;
mod camera;
mod color;
mod dither_shader;
mod draw_list;
mod dummy_passthru_shader;
mod even_line_missing_shader;
mod everything_is_red_shader;
mod game;
mod key_event;
mod keycode;
mod mouse_button;
mod mouse_event;
mod pixel_placement;
mod pixel_shader;
mod rect;
mod screen;
mod sdl2win;
mod nameless_3d_game;
mod triangle;
mod vec2;
mod vec3;
mod vertex;
mod window;

fn main() {
    let mut screen = Screen::new();
    let win = SDL2Window;
    let mut game = Nameless3DThing::new();
    win.start(&mut screen, &mut game);
}

use crate::screen::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::{game::Game, key_event::KeyEvent, keycode::KeyCode, screen::Screen, window::Window};

pub struct SDL2Window;

impl Window for SDL2Window {
    fn start(&self, screen: &mut Screen, game: &mut dyn Game) {
        let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rustsim", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .position_centered()
            //.fullscreen()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::ARGB8888,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                use sdl2::event::Event;

                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        repeat: false,
                        ..
                    } => {
                        if let Some(key) = KeyCode::from_sdl2_key(keycode) {
                            if key == KeyCode::Escape {
                                break 'running;
                            }
                            game.key_event(&KeyEvent::Pressed { key });
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(keycode),
                        repeat: false,
                        ..
                    } => {
                        if let Some(key) = KeyCode::from_sdl2_key(keycode) {
                            game.key_event(&KeyEvent::Released { key });
                        }
                    }
                    _ => {}
                }
            }

            game.update_tick();
            game.render_tick(screen);

            texture
                .with_lock(None, |pixels: &mut [u8], pitch: usize| {
                    for y in 0..SCREEN_HEIGHT {
                        for x in 0..SCREEN_WIDTH {
                            let i = y * SCREEN_WIDTH + x;
                            let col = screen.pixels[i];
                            let offset = y * pitch + x * 4;
                            pixels[offset] = col.b;
                            pixels[offset + 1] = col.g;
                            pixels[offset + 2] = col.r;
                            pixels[offset + 3] = col.a;
                        }
                    }
                })
                .unwrap();

            canvas.clear();
            canvas
                .copy(
                    &texture,
                    None,
                    Some(sdl2::rect::Rect::new(
                        0,
                        0,
                        SCREEN_WIDTH as u32,
                        SCREEN_HEIGHT as u32,
                    )),
                )
                .unwrap();
            canvas.present();

            std::thread::sleep(std::time::Duration::from_millis(8));
        }
    }
}

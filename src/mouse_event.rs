use sdl2::mouse::MouseButton;

pub enum MouseEvent {
    ButtonDown { btn: MouseButton },
    ButtonRelease { btn: MouseButton },
    NewPosition { x: u32, y: u32 },
}

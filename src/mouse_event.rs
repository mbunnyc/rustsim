use crate::mouse_button::MouseButton;

pub enum MouseEvent {
    ButtonDown { btn: MouseButton },
    ButtonRelease { btn: MouseButton },
    NewPosition { x: u32, y: u32 },
}

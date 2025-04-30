use crate::mouse_button::MouseButton;

pub enum MouseEvent {
    ButtonDown { x: u32, y: u32, btn: MouseButton },
    ButtonRelease { x: u32, y: u32, btn: MouseButton },
    NewPosition { x: u32, y: u32 },
    WheelScroll { y: i32 }
}

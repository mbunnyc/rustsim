#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyCode {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Numbers
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Special keys
    Space,
    Return,
    Escape,
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    Tab,
}

impl KeyCode {
    pub fn from_sdl2_key(key: sdl2::keyboard::Keycode) -> Option<KeyCode> {
        use sdl2::keyboard::Keycode;
        
        match key {
            // Letters
            Keycode::A => Some(KeyCode::A),
            Keycode::B => Some(KeyCode::B),
            Keycode::C => Some(KeyCode::C),
            Keycode::D => Some(KeyCode::D),
            Keycode::E => Some(KeyCode::E),
            Keycode::F => Some(KeyCode::F),
            Keycode::G => Some(KeyCode::G),
            Keycode::H => Some(KeyCode::H),
            Keycode::I => Some(KeyCode::I),
            Keycode::J => Some(KeyCode::J),
            Keycode::K => Some(KeyCode::K),
            Keycode::L => Some(KeyCode::L),
            Keycode::M => Some(KeyCode::M),
            Keycode::N => Some(KeyCode::N),
            Keycode::O => Some(KeyCode::O),
            Keycode::P => Some(KeyCode::P),
            Keycode::Q => Some(KeyCode::Q),
            Keycode::R => Some(KeyCode::R),
            Keycode::S => Some(KeyCode::S),
            Keycode::T => Some(KeyCode::T),
            Keycode::U => Some(KeyCode::U),
            Keycode::V => Some(KeyCode::V),
            Keycode::W => Some(KeyCode::W),
            Keycode::X => Some(KeyCode::X),
            Keycode::Y => Some(KeyCode::Y),
            Keycode::Z => Some(KeyCode::Z),

            // Numbers
            Keycode::Num0 => Some(KeyCode::Num0),
            Keycode::Num1 => Some(KeyCode::Num1),
            Keycode::Num2 => Some(KeyCode::Num2),
            Keycode::Num3 => Some(KeyCode::Num3),
            Keycode::Num4 => Some(KeyCode::Num4),
            Keycode::Num5 => Some(KeyCode::Num5),
            Keycode::Num6 => Some(KeyCode::Num6),
            Keycode::Num7 => Some(KeyCode::Num7),
            Keycode::Num8 => Some(KeyCode::Num8),
            Keycode::Num9 => Some(KeyCode::Num9),

            // Arrow keys
            Keycode::Up => Some(KeyCode::Up),
            Keycode::Down => Some(KeyCode::Down),
            Keycode::Left => Some(KeyCode::Left),
            Keycode::Right => Some(KeyCode::Right),

            // Special keys
            Keycode::Space => Some(KeyCode::Space),
            Keycode::Return => Some(KeyCode::Return),
            Keycode::Escape => Some(KeyCode::Escape),
            Keycode::LShift => Some(KeyCode::LShift),
            Keycode::RShift => Some(KeyCode::RShift),
            Keycode::LCtrl => Some(KeyCode::LCtrl),
            Keycode::RCtrl => Some(KeyCode::RCtrl),
            Keycode::Tab => Some(KeyCode::Tab),
            
            _ => None,
        }
    }
}


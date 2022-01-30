use crate::paint::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub enum MouseButton
{
    Primary,
    Secondary,
    Terciary
}

#[derive(Clone, Copy, PartialEq)]
pub enum Key
{
    Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,  
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Escape, Delete, Back, Return,
    End, PageDown, PageUp,
    Left, Up, Right, Down,
    Space, Slash,
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadDivide, NumpadDecimal, NumpadComma, NumpadEquals, NumpadMultiply, NumpadSubtract, NumpadEnter,
    Colon, Comma, Period, Semicolon, Equals,
    LAlt, LBracket, LControl, LShift, RAlt, RBracket, RControl, RShift,
    Minus, Plus,
    Tab, Copy, Paste, Cut
}

impl Key
{
    pub fn ch(self, shift: bool) -> Option<char>
    {
        match self
        {
            Key::Key1 | Key::Numpad1 => if shift { Some('!') } else { Some('1') },
            Key::Key2 | Key::Numpad2 => if shift { Some('"') } else { Some('2') },
            Key::Key3 | Key::Numpad3 => if shift { Some('§') } else { Some('3') },
            Key::Key4 | Key::Numpad4 => if shift { Some('$') } else { Some('4') },
            Key::Key5 | Key::Numpad5 => if shift { Some('%') } else { Some('5') },
            Key::Key6 | Key::Numpad6 => if shift { Some('&') } else { Some('6') },
            Key::Key7 | Key::Numpad7 => if shift { Some('/') } else { Some('7') },
            Key::Key8 | Key::Numpad8 => if shift { Some('(') } else { Some('8') },
            Key::Key9 | Key::Numpad9 => if shift { Some(')') } else { Some('9') },
            Key::Key0 | Key::Numpad0 => if shift { Some('=') } else { Some('!') },
            Key::A => if shift { Some('A') } else { Some('a') },
            Key::B => if shift { Some('B') } else { Some('b') },
            Key::C => if shift { Some('C') } else { Some('c') },
            Key::D => if shift { Some('D') } else { Some('d') },
            Key::E => if shift { Some('E') } else { Some('e') },
            Key::F => if shift { Some('F') } else { Some('f') },
            Key::G => if shift { Some('G') } else { Some('g') },
            Key::H => if shift { Some('H') } else { Some('h') },
            Key::I => if shift { Some('I') } else { Some('i') },
            Key::J => if shift { Some('J') } else { Some('j') },
            Key::K => if shift { Some('K') } else { Some('k') },
            Key::L => if shift { Some('L') } else { Some('l') },
            Key::M => if shift { Some('M') } else { Some('m') },
            Key::N => if shift { Some('N') } else { Some('n') },
            Key::O => if shift { Some('O') } else { Some('o') },
            Key::P => if shift { Some('P') } else { Some('p') },
            Key::Q => if shift { Some('Q') } else { Some('q') },
            Key::R => if shift { Some('R') } else { Some('r') },
            Key::S => if shift { Some('S') } else { Some('s') },
            Key::T => if shift { Some('T') } else { Some('t') },
            Key::U => if shift { Some('U') } else { Some('u') },
            Key::V => if shift { Some('V') } else { Some('v') },
            Key::W => if shift { Some('W') } else { Some('w') },
            Key::X => if shift { Some('X') } else { Some('x') },
            Key::Y => if shift { Some('Y') } else { Some('y') },
            Key::Z => if shift { Some('Z') } else { Some('z') },
            Key::Space => Some(' '),
            Key::Slash | Key::NumpadDivide => Some('/'),
            Key::NumpadMultiply => Some('*'),
            Key::Colon => Some(':'),
            Key::Comma => Some(','),
            Key::Period | Key::NumpadDecimal => Some('.'),
            Key::Semicolon | Key::NumpadComma => Some(';'),
            Key::Equals | Key::NumpadEquals => Some('='),
            Key::Minus | Key::NumpadSubtract => Some('-'),
            Key::Plus | Key::NumpadAdd => Some('+'),
            Key::Tab => Some('\t'),
            _ => None
        }
    }
}

#[derive(Clone)]
pub enum Event
{
    PointerMoved { pos: Vec2 },
    PointerClicked { pos: Vec2, button: MouseButton, pressed: bool },
    PointerGone,
    Key { key: Key, pressed: bool }
}

impl Event
{
    pub(crate) fn scale(&mut self, scale: f32) -> &mut Self
    {
        match self
        {
            Self::PointerMoved { pos } => *pos *= scale,
            Self::PointerClicked { pos, .. } => *pos *= scale,
            _ => {}
        }
        self
    }

    pub(crate) fn offset(&mut self, offset: Vec2) -> &mut Self
    {
        match self
        {
            Self::PointerMoved { pos } => *pos += offset,
            Self::PointerClicked { pos, .. } => *pos += offset,
            _ => {}
        }
        self
    }
}

pub struct EventPod
{
    pub event: Event,
    pub used: bool
}

impl EventPod
{
    pub(crate) fn new(event: Event) -> Self
    {
        Self
        {
            event,
            used: false
        }
    }
}

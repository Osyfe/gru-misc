use std::fmt::Display;

use crate::paint::Vec2;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MouseButton
{
    Primary,
    Secondary,
    Terciary
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Key
{
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,

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

    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    Snapshot,
    Scroll,
    Pause,

    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    Back,
    Return,
    Space,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,

    LAlt,
    LControl,
    LShift,
    RAlt,
    RControl,
    RShift,
    Tab
}

impl Display for Key
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
        let string = match self {
            Key::Key1 => "1",
            Key::Key2 => "2",
            Key::Key3 => "3",
            Key::Key4 => "4",
            Key::Key5 => "5",
            Key::Key6 => "6",
            Key::Key7 => "7",
            Key::Key8 => "8",
            Key::Key9 => "9",
            Key::Key0 => "0",
            Key::A => "A",
            Key::B => "B",
            Key::C => "C",
            Key::D => "D",
            Key::E => "E",
            Key::F => "F",
            Key::G => "G",
            Key::H => "H",
            Key::I => "I",
            Key::J => "J",
            Key::K => "K",
            Key::L => "L",
            Key::M => "M",
            Key::N => "N",
            Key::O => "O",
            Key::P => "P",
            Key::Q => "Q",
            Key::R => "R",
            Key::S => "S",
            Key::T => "T",
            Key::U => "U",
            Key::V => "V",
            Key::W => "W",
            Key::X => "X",
            Key::Y => "Y",
            Key::Z => "Z",
            Key::Escape => "ESC",
            Key::F1 => "F1",
            Key::F2 => "F2",
            Key::F3 => "F3",
            Key::F4 => "F4",
            Key::F5 => "F5",
            Key::F6 => "F6",
            Key::F7 => "F7",
            Key::F8 => "F8",
            Key::F9 => "F9",
            Key::F10 => "F10",
            Key::F11 => "F11",
            Key::F12 => "F12",
            Key::F13 => "F13",
            Key::F14 => "F14",
            Key::F15 => "F15",
            Key::F16 => "F16",
            Key::F17 => "F17",
            Key::F18 => "F18",
            Key::F19 => "F19",
            Key::F20 => "F20",
            Key::F21 => "F21",
            Key::F22 => "F22",
            Key::F23 => "F23",
            Key::F24 => "F24",
            Key::Snapshot => "Snapshot",
            Key::Scroll => "Scroll",
            Key::Pause => "Pause",
            Key::Insert => "INS",
            Key::Home => "Home",
            Key::Delete => "DEL",
            Key::End => "End",
            Key::PageUp => "PageUp",//"Page↑",
            Key::PageDown => "PageDown",//"Page↓",
            Key::Left => "Left",//"◄",
            Key::Up => "Up",//"▲",
            Key::Right => "Right",//"►",
            Key::Down => "Down",//"▼",
            Key::Back => "Back",
            Key::Return => "Return",
            Key::Space => "Space",
            Key::Numlock => "Num",
            Key::Numpad0 => "Num0",
            Key::Numpad1 => "Num1",
            Key::Numpad2 => "Num2",
            Key::Numpad3 => "Num3",
            Key::Numpad4 => "Num4",
            Key::Numpad5 => "Num5",
            Key::Numpad6 => "Num6",
            Key::Numpad7 => "Num7",
            Key::Numpad8 => "Num8",
            Key::Numpad9 => "Num9",
            Key::NumpadAdd => "Num+",
            Key::NumpadDivide => "Num/",
            Key::NumpadDecimal => "Num.",
            Key::NumpadComma => "Num,",
            Key::NumpadEnter => "NumEnter",
            Key::NumpadEquals => "Num=",
            Key::NumpadMultiply => "Num*",
            Key::NumpadSubtract => "Num-",
            Key::LAlt => "LAlt",
            Key::LControl => "LCtrl",
            Key::LShift => "LShift",
            Key::RAlt => "RAlt",
            Key::RControl => "RCtrl",
            Key::RShift => "RShift",
            Key::Tab => "Tab",
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Event
{
    RawMouseDelta(Vec2),
    PointerMoved { pos: Vec2, delta: Vec2 },
    PointerClicked { pos: Vec2, button: MouseButton, pressed: bool },
    PointerGone,
    CloseWindow,
    Scroll { pos: Vec2, dx: f32, dy: f32 },
    Key { key: Key, pressed: bool },
    Char(char)
}

impl Event
{
    pub(crate) fn scale(&mut self, scale: f32) -> &mut Self
    {
        match self
        {
            Self::PointerMoved { pos, delta } =>
            {
                *pos *= scale;
                *delta *= scale;
            },
            Self::PointerClicked { pos, .. } => *pos *= scale,
            Self::Scroll { pos, .. } => *pos *= scale,
            _ => {}
        }
        self
    }

    pub(crate) fn offset(&mut self, offset: Vec2) -> &mut Self
    {
        match self
        {
            Self::PointerMoved { pos, .. } => *pos += offset,
            Self::PointerClicked { pos, .. } => *pos += offset,
            Self::Scroll { pos, .. } => *pos += offset,
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

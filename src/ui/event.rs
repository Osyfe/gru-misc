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

#[derive(Clone, PartialEq)]
pub enum Event
{
    RawMouseDelta(Vec2),
    PointerMoved { pos: Vec2, delta: Vec2 },
    PointerClicked { pos: Vec2, button: MouseButton, pressed: bool },
    PointerGone,
    CloseWindow,
    Scroll { dx: f32, dy: f32 },
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

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
    Char(char),
    Back,
    Escape
}

#[derive(Clone, PartialEq)]
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

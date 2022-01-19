use crate::paint::Vec2;

#[derive(Clone)]
pub enum MouseButton
{
    Primary,
    Secondary,
    Terciary
}

#[derive(Clone)]
pub enum Event
{
    PointerMoved { pos: Vec2 },
    PointerClicked { pos: Vec2, button: MouseButton, pressed: bool },
    PointerGone
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

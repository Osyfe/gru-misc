pub mod widget;
pub mod lens;
pub mod layout;
pub mod event;

use crate::{paint, text::Font};

const DEFAULT_SCALE: f32 = 20.0;

#[derive(Clone, PartialEq)]
pub struct UiConfig
{
    pub size: paint::Vec2,
    pub scale: f32,
    pub display_scale_factor: f32
}

pub struct Frame<'a>
{
    pub events: std::vec::Drain<'a, event::EventPod>,
    pub paint: paint::Frame<'a>
}

pub struct Ui<'a, T: 'a>
{
    config_current: UiConfig,
    config_fetch: fn(&T) -> UiConfig,
    events: Vec<event::EventPod>,
    painter: paint::Painter<'a>,
    widgets: Vec<Box<dyn Widget<T> + 'a>>
}

impl<'a, T> Ui<'a, T>
{
    pub fn new(font: Font<'a>, config: fn(&T) -> UiConfig) -> Self
    {
        Self
        {
            config_current: UiConfig { size: paint::Vec2(1.0, 1.0), scale: DEFAULT_SCALE, display_scale_factor: 1.0 },
            config_fetch: config,
            events: Vec::new(),
            painter: paint::Painter::new(font),
            widgets: Vec::new()
        }
    }

    pub fn add<W: Widget<T> + 'a>(&mut self, widget: W)
    {
        self.widgets.push(Box::new(widget));
    }

    pub fn frame<'b>(&mut self, data: &mut T, events: impl Iterator<Item = &'b event::Event>) -> Frame
    {
        self.config_current = (self.config_fetch)(data);
        self.config_current.scale *= DEFAULT_SCALE;
        let scale = self.config_current.scale * self.config_current.display_scale_factor;

        let mut layout_requested = false;
        let mut repaint_requested = false;
        let mut ctx = EventCtx { layout_requested: &mut layout_requested, repaint_requested: &mut repaint_requested };
        for event in events
        {
            let mut event = event::EventPod::new(event.clone());
            for widget in &mut self.widgets
            {
                widget.event(&mut ctx, data, &mut event);
            }
            self.events.push(event);
        }

        self.painter.begin_frame(scale);
        for widget in &mut self.widgets
        {
            self.painter.set_offset(paint::Vec2(0.0, 0.0));
            let mut ctx = LayoutCtx { painter: &self.painter };
            let size = widget.layout(&mut ctx, data, paint::Rect { min: paint::Vec2(0.0, 0.0), max: paint::Vec2::from(self.config_current.size) / scale });
            let mut ctx = PaintCtx { painter: &mut self.painter };
            widget.paint(&mut ctx, data, size);
        }
        
        Frame { events: self.events.drain(..), paint: self.painter.end_frame() }
    }
}

pub trait Widget<T>
{
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut event::EventPod);
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: paint::Rect) -> paint::Vec2;
    fn paint(&self, ctx: &mut PaintCtx, data: &T, size: paint::Vec2);
}

pub struct EventCtx<'a>
{
    layout_requested: &'a mut bool,
    repaint_requested: &'a mut bool
}

impl<'a> EventCtx<'a>
{
    #[inline]
    pub fn request_layout(&mut self)
    {
        *self.layout_requested = true;
    }

    #[inline]
    pub fn request_repaint(&mut self)
    {
        *self.repaint_requested = true;
    }
}

pub struct LayoutCtx<'a, 'b>
{
    painter: &'b paint::Painter<'a>
}

impl<'a, 'b> LayoutCtx<'a, 'b>
{
    #[inline]
    pub fn text_width(&mut self, text: &str, size: paint::TextSize) -> f32
    {
        self.painter.text_width(text, size)
    }
}

pub struct PaintCtx<'a, 'b>
{
    pub painter: &'b mut paint::Painter<'a>
}

pub trait Lens<U, T>
{
    fn with<A, F: FnOnce(&T) -> A>(&self, data: &U, f: F) -> A;
    fn with_mut<A, F: FnOnce(&mut T) -> A>(&mut self, data: &mut U, f: F) -> A;
}

pub use gru_ui_derive::Lens;

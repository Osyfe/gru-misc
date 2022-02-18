pub mod widget;
pub mod lens;
pub mod layout;
pub mod event;
pub mod interact;

use crate::{paint, text::Font};
use std::marker::PhantomData;

const DEFAULT_SCALE: f32 = 20.0;

#[derive(Clone, PartialEq)]
pub struct UiConfig
{
    pub size: paint::Vec2,
    pub scale: f32,
    pub display_scale_factor: f32
}

pub struct UiKey(usize);

pub struct Query<'a>
{
    responses: &'a [interact::ResponseState],
    update_requested: &'a mut bool
}

pub struct Frame<'a>
{
    pub events: &'a mut [event::EventPod],
    pub paint: paint::Frame<'a>,
    pub query: Query<'a>
}

pub struct Ui<'a, T: 'a>
{
    config_current: UiConfig,
    config_fetch: fn(&T) -> UiConfig,
    events: Vec<event::EventPod>,
    painter: paint::Painter<'a>,
    widgets: Vec<(Box<dyn Widget<T> + 'a>, fn(&T) -> bool, bool)>,
    responses: Vec<interact::ResponseState>,
    update_requested: bool
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
            widgets: Vec::new(),
            responses: Vec::new(),
            update_requested: true
        }
    }

    pub fn add<W: Widget<T> + 'a>(&mut self, widget: W, active: fn(&T) -> bool) -> UiKey
    {
        let key =  UiKey(self.widgets.len());
        self.widgets.push((Box::new(widget), active, false));
        key
    }

    pub fn register<'b, U, W: Widget<U>>(&mut self, mut response: interact::Response<'b, U, W>) -> (interact::Response<'b, U, W>, interact::ResponseKey)
    {
        let key = interact::ResponseKey(self.responses.len());
        self.responses.push(interact::ResponseState::new());
        response.key = Some(key);
        (response, key)
    }

    pub fn request_update(&mut self)
    {
        self.update_requested = true;
    }

    pub fn frame<'b>(&mut self, data: &mut T, events: impl Iterator<Item = &'b event::Event>) -> Frame
    {
        let old_config = self.config_current.clone();
        self.config_current = (self.config_fetch)(data);
        self.config_current.scale *= DEFAULT_SCALE;
        let scale = self.config_current.scale * self.config_current.display_scale_factor;
        if old_config != self.config_current { self.update_requested = true; }

        for response in &mut self.responses { response.clicked = None; }
        for (widget, active_fetch, active) in &mut self.widgets
        {
            let new_active = active_fetch(data);
            if (*active != new_active) || (*active && widget.update(data)) { self.update_requested = true; }
            *active = new_active;
        }

        self.events.clear();
        let mut ctx = EventCtx { update_requested: &mut self.update_requested, responses: &mut self.responses[..] };
        for event in events
        {
            let mut event = event::EventPod::new(event.clone());
            event.event.scale(1.0 / scale);
            for (widget, _, active) in &mut self.widgets
            {
                if *active { widget.event(&mut ctx, data, &mut event); }
            }
            event.event.scale(scale);
            self.events.push(event);
        }

        if self.update_requested
        {
            //println!("Updating UI: {:?}", std::time::Instant::now());
            self.update_requested = false;
            self.painter.clear_frame(scale);
            for (widget, _, active) in &mut self.widgets
            {
                if *active
                {
                    self.painter.set_offset(paint::Vec2(0.0, 0.0));
                    let mut ctx = LayoutCtx { painter: &mut self.painter };
                    let size = widget.layout(&mut ctx, data, paint::Rect { min: paint::Vec2(0.0, 0.0), max: paint::Vec2::from(self.config_current.size) / scale });
                    let mut ctx = PaintCtx { painter: &mut self.painter, state: WidgetState::Cold };
                    widget.paint(&mut ctx, data, size);
                }
            }
        }

        let paint = self.painter.get_frame();
        let query = Query { responses: &self.responses, update_requested: &mut self.update_requested };
        Frame { events: &mut self.events, paint, query }
    }
}

impl<'a> Query<'a>
{
    pub fn query(&self, key: interact::ResponseKey) -> &interact::ResponseState
    {
        &self.responses[key.0]
    }

    pub fn request_update(&mut self)
    {
        *self.update_requested = true;
    }
}

pub trait Widget<T>
{
    fn update(&mut self, _: &T) -> bool;
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut event::EventPod);
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: paint::Rect) -> paint::Vec2;
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: paint::Vec2) -> paint::Vec2;
    fn response(&mut self, _: &mut T, _: Option<event::MouseButton>) -> bool { false }
}

pub struct EventCtx<'a>
{
    update_requested: &'a mut bool,
    responses: &'a mut [interact::ResponseState]
}

impl<'a> EventCtx<'a>
{
    #[inline]
    pub fn request_update(&mut self)
    {
        *self.update_requested = true;
    }
}

pub struct LayoutCtx<'a, 'b>
{
    painter: &'b mut paint::Painter<'a>
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
    pub painter: &'b mut paint::Painter<'a>,
    pub state: WidgetState
}

#[derive(Clone, Copy, PartialEq)]
pub enum WidgetState
{
    Cold,
    Hot,
    Hover
}

pub trait Lens<U, T>
{
    fn with<A, F: FnOnce(&T) -> A>(&self, data: &U, f: F) -> A;
    fn with_mut<A, F: FnOnce(&mut T) -> A>(&mut self, data: &mut U, f: F) -> A;
}

pub use gru_ui_derive::Lens;

struct WidgetPod<T, W: Widget<T>>
{
    widget: W,
    _phantom: PhantomData<T>
}

impl<T, W: Widget<T>> WidgetPod<T, W>
{
    fn new(widget: W) -> Self
    {
        Self { widget, _phantom: PhantomData }
    }
}

struct WidgetPodS<T, W: Widget<T>>
{
    widget: W,
    _phantom: PhantomData<T>,
    size: paint::Vec2
}

impl<T, W: Widget<T>> WidgetPodS<T, W>
{
    fn new(widget: W) -> Self
    {
        Self { widget, _phantom: PhantomData, size: paint::Vec2(0.0, 0.0) }
    }
}

struct WidgetPodP<T, W: Widget<T>>
{
    widget: W,
    _phantom: PhantomData<T>,
    pos: paint::Vec2,
    size: paint::Vec2
}

impl<T, W: Widget<T>> WidgetPodP<T, W>
{
    fn new(widget: W) -> Self
    {
        Self { widget, _phantom: PhantomData, pos: paint::Vec2(0.0, 0.0), size: paint::Vec2(0.0, 0.0) }
    }
}

//type WidgetBox<'a, T> = WidgetPod<T, Box<dyn Widget<T> + 'a>>;
//type WidgetBoxS<'a, T> = WidgetPod<T, Box<dyn Widget<T> + 'a>>;
type WidgetBoxP<'a, T> = WidgetPodP<T, Box<dyn Widget<T> + 'a>>;

pub mod widget;
pub mod lens;
pub mod layout;
pub mod event;
pub mod interact;
pub mod dynamic;
pub mod style;
mod pods;

use crate::{paint, text_sdf::Font};
use std::{marker::PhantomData, hash::Hash, rc::Rc, cell::{RefCell, Ref}};
use ahash::AHashMap;

const DEFAULT_SCALE: f32 = 20.0;

#[derive(Clone, PartialEq)]
pub struct UiConfig
{
    pub size: paint::Vec2,
    pub scale: f32,
    pub display_scale_factor: f32
}

pub struct UiKey(usize);

pub struct Register<'a, K: Hash + Eq>(&'a Rc<RefCell<AHashMap<K, interact::ResponseState>>>);

pub struct Query<'a, K: Hash + Eq>
{
    responses: Ref<'a, AHashMap<K, interact::ResponseState>>,
    update_requested: &'a mut bool
}

pub struct Frame<'a, K: Hash + Eq>
{
    pub events: &'a mut [event::EventPod],
    pub paint: paint::Frame<'a>,
    pub query: Query<'a, K>
}

pub struct Ui<'a, T: 'a, K: Hash + Eq>
{
    config_current: UiConfig,
    config_fetch: fn(&T) -> UiConfig,
    events: Vec<event::EventPod>,
    painter: paint::Painter,
    widgets: Vec<(Box<dyn Widget<T> + 'a>, fn(&T) -> bool, bool)>,
    responses: Rc<RefCell<AHashMap<K, interact::ResponseState>>>,
    update_requested: bool
}

impl<'a, T, K: Hash + Eq> Ui<'a, T, K>
{
    pub fn new(font: Font, config: fn(&T) -> UiConfig) -> Self
    {
        Self
        {
            config_current: UiConfig { size: paint::Vec2(1.0, 1.0), scale: DEFAULT_SCALE, display_scale_factor: 1.0 },
            config_fetch: config,
            events: Vec::new(),
            painter: paint::Painter::new(font),
            widgets: Vec::new(),
            responses: Rc::new(RefCell::new(AHashMap::new())),
            update_requested: true
        }
    }

    pub fn add_box(&mut self, widget: Box<dyn Widget<T> + 'a>, active: fn(&T) -> bool) -> UiKey
    {
        let key =  UiKey(self.widgets.len());
        self.widgets.push((widget, active, false));
        key
    }

    pub fn add<W: Widget<T> + 'a>(&mut self, widget: W, active: fn(&T) -> bool) -> UiKey
    {
        self.add_box(Box::new(widget), active)
    }

    pub fn register(&self) -> Register<K>
    {
        Register(&self.responses)
    }

    pub fn request_update(&mut self)
    {
        self.update_requested = true;
    }

    pub fn frame<'b>(&mut self, data: &mut T, events: impl Iterator<Item = &'b event::Event>) -> Frame<K>
    {
        let old_config = self.config_current.clone();
        self.config_current = (self.config_fetch)(data);
        self.config_current.scale *= DEFAULT_SCALE;
        let scale = self.config_current.scale * self.config_current.display_scale_factor;
        if old_config != self.config_current { self.update_requested = true; }

        let mut gone = event::EventPod::new(event::Event::PointerGone);
        for response in self.responses.borrow_mut().values_mut() { response.reset(); }
        for (widget, active_fetch, active) in &mut self.widgets
        {
            let new_active = active_fetch(data);
            if *active != new_active { self.update_requested = true; }
            if *active && !new_active { widget.event(&mut EventCtx { update_requested: &mut self.update_requested }, data, &mut gone); }
            *active = new_active;
        }

        let mut ctx = EventCtx { update_requested: &mut self.update_requested };
        self.events.clear();
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

        for (widget, _, active) in &mut self.widgets
        {
            if *active && widget.update(data) { self.update_requested = true; }
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
                    let mut ctx = PaintCtx { painter: &mut self.painter, state: interact::WidgetState::Cold, style: Default::default() };
                    widget.paint(&mut ctx, data, size);
                }
            }
        }

        let paint = self.painter.get_frame();
        let query = Query { responses: self.responses.borrow(), update_requested: &mut self.update_requested };
        Frame { events: &mut self.events, paint, query }
    }
}

impl<'a, K: Hash + Eq> Query<'a, K>
{
    pub fn query<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> Option<&interact::ResponseState> where K: std::borrow::Borrow<Q>
    {
        self.responses.get(key)
    }

    pub fn request_update(&mut self)
    {
        *self.update_requested = true;
    }
}

pub trait Widget<T>
{
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut event::EventPod);
    fn update(&mut self, data: &mut T) -> bool;
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: paint::Rect) -> paint::Vec2;
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: paint::Vec2) -> paint::Vec2;
    fn response(&mut self, _: &mut T, _: Option<event::MouseButton>) -> bool { false }
}

pub struct EventCtx<'a>
{
    update_requested: &'a mut bool
}

impl<'a> EventCtx<'a>
{
    #[inline]
    pub fn request_update(&mut self)
    {
        *self.update_requested = true;
    }
}

pub struct LayoutCtx<'a>
{
    painter: &'a mut paint::Painter
}

impl<'a> LayoutCtx<'a>
{
    #[inline]
    pub fn text_width(&mut self, text: &str, size: paint::TextSize) -> f32
    {
        self.painter.text_width(text, size)
    }
}

pub struct PaintCtx<'a>
{
    pub painter: &'a mut paint::Painter,
    pub state: interact::WidgetState,
    pub style: style::StyleSet
}

pub trait Lens<U, T>
{
    fn with<A, F: FnOnce(&T) -> A>(&self, data: &U, f: F) -> A;
    fn with_mut<A, F: FnOnce(&mut T) -> A>(&self, data: &mut U, f: F) -> A;
}

pub use gru_ui_derive::Lens;

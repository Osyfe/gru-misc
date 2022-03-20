use super::{Ui, Widget, EventCtx, LayoutCtx, PaintCtx, WidgetState, event::{EventPod, Event, MouseButton}, paint::{Vec2, Rect}, WidgetPod, WidgetPodS};
use std::{hash::Hash, rc::Rc, cell::RefCell};
use ahash::AHashMap;

pub struct ResponseState
{
    pub state: WidgetState,
    pub clicked: Option<MouseButton>
}

impl ResponseState
{
    pub(crate) fn new() -> Self
    {
        Self { state: WidgetState::Cold, clicked: None }
    }
}

pub struct Response<'a, T, W: Widget<T>, K: Hash + Eq>
{
    inner: WidgetPodS<T, W>,
    state: WidgetState,
    map: Rc<RefCell<AHashMap<K, ResponseState>>>,
    key: Option<K>,
    action: Option<Box<dyn FnMut() + 'a>>
}

impl<'a, T, W: Widget<T>, K: Hash + Eq> Widget<T> for Response<'a, T, W, K>
{
    #[inline]
    fn update(&mut self, data: &T) -> bool
    {
        self.inner.widget.update(data)
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        self.inner.widget.event(ctx, data, event);
        let mut update = false;
        match event.event
        {
            Event::PointerGone => self.state = WidgetState::Cold,
            Event::PointerMoved { pos, .. } =>
            {
                let hover = Rect::new_origin(self.inner.size).contains_linf(pos);
                if !hover && self.state != WidgetState::Cold
                {
                    self.state = WidgetState::Cold;
                    update = true;
                }
                if hover && self.state == WidgetState::Cold
                {
                    self.state = WidgetState::Hover;
                    update = true;
                }
            },
            Event::PointerClicked { pos, button, pressed } =>
            {
                let mut maybe_button = None;
                let hover = Rect::new_origin(self.inner.size).contains_linf(pos);
                if hover && pressed && !event.used
                {
                    self.state = WidgetState::Hot;
                    update = true;
                    event.used = true;
                }
                if hover && !pressed && self.state == WidgetState::Hot
                {
                    self.state = WidgetState::Hover;
                    update = true;
                    maybe_button = Some(button);
                    if let Some(action) = &mut self.action { action(); }
                    if let Some(key) = &self.key { self.map.borrow_mut().get_mut(key).unwrap().clicked = Some(button); }
                }
                if !pressed && self.inner.widget.response(data, maybe_button) { update = true; }
            },
            _ => {}
        }
        if update
        {
            ctx.request_update();
            if let Some(key) = &self.key { self.map.borrow_mut().get_mut(key).unwrap().state = self.state; }
        }
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, size: Rect) -> Vec2
    {
        self.inner.widget.layout(ctx, data, size)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        let old_state = ctx.state;
        ctx.state = self.state;
        self.inner.size = self.inner.widget.paint(ctx, data, size);
        ctx.state = old_state;
        self.inner.size
    }
}

impl<'a, T, W: Widget<T>, K: Hash + Eq> Response<'a, T, W, K>
{
    pub fn new<U>(widget: W, ui: &Ui<U, K>) -> Self
    {
        Self { inner: WidgetPodS::new(widget), state: WidgetState::Cold, map: ui.responses.clone(), key: None, action: None }
    }

    pub fn query<L: ?Sized + ToOwned<Owned = K>>(mut self, key: &L) -> Self
    {
        self.map.borrow_mut().insert(key.to_owned(), ResponseState::new());
        self.key = Some(key.to_owned());
        self
    }

    pub fn action(mut self, action: impl FnMut() + 'a) -> Self
    {
        self.action = Some(Box::new(action));
        self
    }
}

pub struct Watch<T: Clone + PartialEq, W: Widget<T>>
{
    inner: WidgetPod<T, W>,
    copy: Option<T>
}

impl<T: Clone + PartialEq, W: Widget<T>> Widget<T> for Watch<T, W>
{
    #[inline]
    fn update(&mut self, data: &T) -> bool
    {
        let update = match &self.copy
        {
            None => true,
            Some(copy) => data != copy
        };
        if update { self.copy = Some(data.clone()); }
        update
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        self.inner.widget.event(ctx, data, event)
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, size: Rect) -> Vec2
    {
        self.inner.widget.layout(ctx, data, size)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        self.inner.widget.paint(ctx, data, size)
    }

    #[inline]
    fn response(&mut self, data: &mut T, button: Option<MouseButton>) -> bool
    {
        self.inner.widget.response(data, button)
    }
}

impl<T: Clone + PartialEq, W: Widget<T>> Watch<T, W>
{
    pub fn new(widget: W) -> Self
    {
        Self { inner: WidgetPod::new(widget), copy: None }
    }
}

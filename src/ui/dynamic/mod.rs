use super::{Ui, Widget, EventCtx, LayoutCtx, PaintCtx, event::{EventPod, MouseButton}, interact::ResponseState, paint::{Vec2, Rect}, Register, WidgetPod};
use std::{rc::Rc, cell::RefCell, hash::Hash};
use ahash::AHashMap;

pub struct Watch<T: Clone + PartialEq, W: Widget<T>>
{
    inner: WidgetPod<T, W>,
    copy: Option<T>
}

impl<T: Clone + PartialEq, W: Widget<T>> Widget<T> for Watch<T, W>
{
    #[inline]
    fn update(&mut self, data: &mut T) -> bool
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

pub struct Transform<'a, U, T, W: Widget<T>>
{
    inner: W,
    data: T,
    transformer: Box<dyn FnMut(&U) -> T + 'a>
}

impl<'a, U, T, W: Widget<T>> Widget<U> for Transform<'a, U, T, W>
{
    #[inline]
    fn update(&mut self, _: &mut U) -> bool
    {
        false
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, _: &mut U, event: &mut EventPod)
    {
        self.inner.event(ctx, &mut self.data, event)
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &U, constraints: Rect) -> Vec2
    {
        self.data = (self.transformer)(data);
        self.inner.layout(ctx, &self.data, constraints)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, _: &U, size: Vec2) -> Vec2
    {
        self.inner.paint(ctx, &self.data, size)
    }
}

impl<'a, U, T, W: Widget<T>> Transform<'a, U, T, W>
{
    pub fn new(widget: W, init: T, transformer: impl FnMut(&U) -> T + 'a) -> Self
    {
        Self { inner: widget, data: init, transformer: Box::new(transformer) }
    }
}

pub struct Dynamic<T, W: Widget<T>, K: Hash + Eq, F: FnMut(Register<K>, &mut T) -> Option<Option<W>>>
{
    inner: WidgetPod<T, Option<W>>,
    map: Rc<RefCell<AHashMap<K, ResponseState>>>,
    generator: F
}

impl<T, W: Widget<T>, K: Hash + Eq, F: FnMut(Register<K>, &mut T) -> Option<Option<W>>> Widget<T> for Dynamic<T, W, K, F>
{
    #[inline]
    fn update(&mut self, data: &mut T) -> bool
    {
        let mut update = false;
        if let Some(new) = (self.generator)(Register(&self.map), data)
        {
            self.inner.widget = new;
            update = true;
        }
        if self.inner.widget.update(data) { update = true; }
        update
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        self.inner.widget.event(ctx, data, event)
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: Rect) -> Vec2
    {
        self.inner.widget.layout(ctx, data, constraints)
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

impl<T, W: Widget<T>, K: Hash + Eq, F: FnMut(Register<K>, &mut T) -> Option<Option<W>>> Dynamic<T, W, K, F>
{
    pub fn new<U>(ui: &Ui<U, K>, generator: F) -> Self
    {
        Self { inner: WidgetPod::new(None), map: ui.responses.clone(), generator }
    }
}

use super::{Widget, EventCtx, LayoutCtx, PaintCtx, event::{EventPod, MouseButton}, interact::ResponseState, paint::{Vec2, Rect}, Register, pods::{WidgetPod, WidgetPodS}};
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

pub enum DynamicContent<W>
{
    Keep,
    Show(W),
    Hide
}

pub struct Dynamic<T, W: Widget<T>, K: Hash + Eq, F: FnMut(Register<K>, &mut T) -> DynamicContent<W>>
{
    inner: WidgetPod<T, Option<W>>,
    map: Rc<RefCell<AHashMap<K, ResponseState>>>,
    generator: F
}

impl<T, W: Widget<T>, K: Hash + Eq, F: FnMut(Register<K>, &mut T) -> DynamicContent<W>> Widget<T> for Dynamic<T, W, K, F>
{
    #[inline]
    fn update(&mut self, data: &mut T) -> bool
    {
        let mut update = false;
        match (self.generator)(Register(&self.map), data)
        {
            DynamicContent::Keep => {},
            DynamicContent::Show(new) =>
            {
                update = true;
                self.inner.widget = Some(new);
            },
            DynamicContent::Hide =>
            {
                if !self.inner.widget.is_none() { update = true; }
                self.inner.widget = None;
            }
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

impl<T, W: Widget<T>, K: Hash + Eq, F: FnMut(Register<K>, &mut T) -> DynamicContent<W>> Dynamic<T, W, K, F>
{
    pub fn new(register: &Register<K>, generator: F) -> Self
    {
        Self { inner: WidgetPod::new(None), map: register.0.clone(), generator }
    }
}

pub fn dynamic_init<T, W: Widget<T>, K: Hash + Eq, F: FnMut(Register<K>, &mut T, bool) -> DynamicContent<W>>(register: &Register<K>, mut generator: F, init: &mut T) -> Dynamic<T, W, K, impl FnMut(Register<K>, &mut T) -> DynamicContent<W>>
{
    let map = register.0.clone();
    let register2 = Register(&map);
    let widget = match generator(register2, init, true)
    {
        DynamicContent::Show(widget) => Some(widget),
        _ => None
    };
    let generator = move |register: Register<'_, K>, data: &'_ mut T| generator(register, data, false);
    Dynamic { inner: WidgetPod::new(widget), map, generator }
}

pub struct Folder<T, WH: Widget<T>, WB: Widget<T>>
{
    head: WidgetPodS<T, WH>,
    body: WidgetPod<T, WB>,
    expanded: bool
}

impl<T, WH: Widget<T>, WB: Widget<T>> Widget<T> for Folder<T, WH, WB>
{
    #[inline]
    fn update(&mut self, data: &mut T) -> bool
    {
        if self.expanded { self.body.widget.update(data) }
        else { self.head.widget.update(data) }
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        if self.expanded
        {
            let head_height = Vec2(0.0, self.head.size.1);
            event.event.offset(-head_height);
            self.body.widget.event(ctx, data, event);
            event.event.offset(head_height);
        }
        else { self.head.widget.event(ctx, data, event); }
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: Rect) -> Vec2
    {
        if self.expanded
        {
            let head_height = Vec2(0.0, self.head.size.1);
            self.body.widget.layout(ctx, data, constraints - head_height) + head_height
        }
        else
        {
            self.head.size = self.head.widget.layout(ctx, data, constraints);
            self.head.size
        }
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        let size_head = self.head.widget.paint(ctx, data, self.head.size);
        if self.expanded
        {
            let head_height = Vec2(0.0, size_head.1);
            ctx.painter.add_offset(head_height);
            let size_body = self.body.widget.paint(ctx, data, size - head_height);
            ctx.painter.add_offset(-head_height);
            let mut size_total = size_body + head_height;
            size_total.1 = size_total.1.max(size_head.1);
            size_total
        } else { size_head }
    }

    #[inline]
    fn response(&mut self, _: &mut T, button: Option<MouseButton>) -> bool
    {
        match button
        {
            None => { let update = self.expanded; self.expanded = false; update },
            Some(MouseButton::Primary) => { self.expanded = !self.expanded; true },
            _ => false
        }
    }
}

impl<T, WH: Widget<T>, WB: Widget<T>> Folder<T, WH, WB>
{
    pub fn new(head: WH, body: WB) -> Self
    {
        Self { head: WidgetPodS::new(head), body: WidgetPod::new(body), expanded: false }
    }
}

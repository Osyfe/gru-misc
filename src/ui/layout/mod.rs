mod flex;
pub use flex::*;
mod split;
pub use split::*;

use super::{Widget, EventCtx, LayoutCtx, PaintCtx, event::EventPod, paint::{Vec2, Rect}};
use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq)]
pub enum LayoutAlign
{
    Front,
    Center,
    Back,
    Fill
}

type WidgetBox<'a, T> = WidgetPod<T, Box<dyn Widget<T> + 'a>>;

struct WidgetPod<T, W: Widget<T>>
{
    widget: W,
    _phantom: PhantomData<T>,
    pos: Vec2,
    size: Vec2
}

impl<T, W: Widget<T>> WidgetPod<T, W>
{
    fn new(widget: W) -> Self
    {
        Self { widget, _phantom: PhantomData, pos: Vec2(0.0, 0.0), size: Vec2(0.0, 0.0) }
    }
}

pub struct Empty<T>
{
    _phantom: PhantomData<T>
}

impl<T> Widget<T> for Empty<T>
{
    fn event(&mut self, _: &mut EventCtx, _: &mut T, _: &mut EventPod) { }

    #[inline]
    fn layout(&mut self, _: &mut LayoutCtx, _: &T, constraints: Rect) -> Vec2
    {
        constraints.min
    }

    #[inline]
    fn paint(&self, _: &mut PaintCtx, _: &T, _: Vec2) { }
}

impl<T> Empty<T>
{
    pub fn new() -> Self
    {
        Self { _phantom: PhantomData }
    }
}

pub struct Fix<T, W: Widget<T>>
{
    inner: W,
    _phantom: PhantomData<T>,
    size: Vec2
}

impl<T, W: Widget<T>> Widget<T> for Fix<T, W>
{
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        self.inner.event(ctx, data, event)
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, _: Rect) -> Vec2
    {
        self.inner.layout(ctx, data, Rect::new_origin(self.size))
    }

    #[inline]
    fn paint(&self, ctx: &mut PaintCtx, data: &T, size: Vec2)
    {
        self.inner.paint(ctx, data, size);
    }
}

impl<T, W: Widget<T>> Fix<T, W>
{
    pub fn new(widget: W, size: Vec2) -> Self
    {
        Self { inner: widget, _phantom: PhantomData, size }
    }
}

pub struct Align<T, W: Widget<T>>
{
    inner: W,
    _phantom: PhantomData<T>,
    width: LayoutAlign,
    height: LayoutAlign,
    pos: Vec2,
    size: Vec2
}

impl<T, W: Widget<T>> Widget<T> for Align<T, W>
{
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        self.inner.event(ctx, data, event)
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, mut constraints: Rect) -> Vec2
    {
        if self.width == LayoutAlign::Fill { constraints.min.0 = constraints.max.0; }
        if self.height == LayoutAlign::Fill { constraints.min.1 = constraints.max.1; }
        self.size = self.inner.layout(ctx, data, constraints);
        let margin = Vec2((constraints.max.0 - self.size.0).max(0.0), (constraints.max.1 - self.size.1).max(0.0));
        self.pos.0 = match self.width
        {
            LayoutAlign::Front | LayoutAlign::Fill => 0.0,
            LayoutAlign::Center => margin.0 / 2.0,
            LayoutAlign::Back => margin.0
        };
        self.pos.1 = match self.height
        {
            LayoutAlign::Front | LayoutAlign::Fill => 0.0,
            LayoutAlign::Center => margin.1 / 2.0,
            LayoutAlign::Back => margin.1
        };
        Vec2(self.size.0.max(constraints.max.0), self.size.1.max(constraints.max.1))
    }

    #[inline]
    fn paint(&self, ctx: &mut PaintCtx, data: &T, size: Vec2)
    {
        ctx.painter.add_offset(self.pos);
        self.inner.paint(ctx, data, size);
        ctx.painter.add_offset(-self.pos);
    }
}

impl<T, W: Widget<T>> Align<T, W>
{
    pub fn new(widget: W, width: LayoutAlign, height: LayoutAlign) -> Self
    {
        Self { inner: widget, _phantom: PhantomData, width, height, pos: Vec2(0.0, 0.0), size: Vec2(0.0, 0.0) }
    }
}

pub struct Padding<T, W: Widget<T>>
{
    inner: W,
    _phantom: PhantomData<T>,
    padding: Vec2,
    size: Vec2
}

impl<T, W: Widget<T>> Widget<T> for Padding<T, W>
{
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        self.inner.event(ctx, data, event)
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: Rect) -> Vec2
    {
        self.size = self.inner.layout(ctx, data, constraints - self.padding * 2.0);
        self.size + self.padding * 2.0
    }

    #[inline]
    fn paint(&self, ctx: &mut PaintCtx, data: &T, size: Vec2)
    {
        ctx.painter.add_offset(self.padding);
        self.inner.paint(ctx, data, size - self.padding * 2.0);
        ctx.painter.add_offset(-self.padding);
    }
}

impl<T, W: Widget<T>> Padding<T, W>
{
    pub fn new(widget: W, padding: Vec2) -> Self
    {
        Self { inner: widget, _phantom: PhantomData, padding, size: Vec2(0.0, 0.0) }
    }
}

mod flex;
pub use flex::*;
mod split;
pub use split::*;

use super::{Widget, EventCtx, LayoutCtx, PaintCtx, event::EventPod, paint::{Vec2, Rect}, WidgetPod, WidgetPodS, WidgetPodP, WidgetBoxP};
use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq)]
pub enum LayoutAlign
{
    Front,
    Center,
    Back,
    Fill
}

pub struct Empty<T>
{
    _phantom: PhantomData<T>
}

impl<T> Widget<T> for Empty<T>
{
    fn update(&mut self, _: &T) -> bool
    {
        false
    }

    fn event(&mut self, _: &mut EventCtx, _: &mut T, _: &mut EventPod) { }

    fn layout(&mut self, _: &mut LayoutCtx, _: &T, constraints: Rect) -> Vec2
    {
        constraints.min
    }

    fn paint(&mut self, _: &mut PaintCtx, _: &T, size: Vec2) -> Vec2
    {
        size
    }
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
    inner: WidgetPod<T, W>,
    should_size: Vec2,
}

impl<T, W: Widget<T>> Widget<T> for Fix<T, W>
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
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, _: Rect) -> Vec2
    {
        self.inner.widget.layout(ctx, data, Rect::new_origin(self.should_size))
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        self.inner.widget.paint(ctx, data, size)
    }
}

impl<T, W: Widget<T>> Fix<T, W>
{
    pub fn new(widget: W, size: Vec2) -> Self
    {
        Self { inner: WidgetPod::new(widget), should_size: size }
    }
}

pub struct Align<T, W: Widget<T>>
{
    inner: WidgetPodP<T, W>,
    width: LayoutAlign,
    height: LayoutAlign
}

impl<T, W: Widget<T>> Widget<T> for Align<T, W>
{
    #[inline]
    fn update(&mut self, data: &T) -> bool
    {
        self.inner.widget.update(data)
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        event.event.offset(-self.inner.pos);
        self.inner.widget.event(ctx, data, event);
        event.event.offset(self.inner.pos);
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, mut constraints: Rect) -> Vec2
    {
        if self.width == LayoutAlign::Fill { constraints.min.0 = constraints.max.0; }
        if self.height == LayoutAlign::Fill { constraints.min.1 = constraints.max.1; }
        self.inner.size = self.inner.widget.layout(ctx, data, constraints);
        let margin = Vec2((constraints.max.0 - self.inner.size.0).max(0.0), (constraints.max.1 - self.inner.size.1).max(0.0));
        self.inner.pos.0 = match self.width
        {
            LayoutAlign::Front | LayoutAlign::Fill => 0.0,
            LayoutAlign::Center => margin.0 / 2.0,
            LayoutAlign::Back => margin.0
        };
        self.inner.pos.1 = match self.height
        {
            LayoutAlign::Front | LayoutAlign::Fill => 0.0,
            LayoutAlign::Center => margin.1 / 2.0,
            LayoutAlign::Back => margin.1
        };
        Vec2(self.inner.size.0.max(constraints.max.0), self.inner.size.1.max(constraints.max.1))
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        ctx.painter.add_offset(self.inner.pos);
        let size = self.inner.widget.paint(ctx, data, size);
        ctx.painter.add_offset(-self.inner.pos);
        size
    }
}

impl<T, W: Widget<T>> Align<T, W>
{
    pub fn new(widget: W, width: LayoutAlign, height: LayoutAlign) -> Self
    {
        Self { inner: WidgetPodP::new(widget), width, height }
    }
}

pub struct Padding<T, W: Widget<T>>
{
    inner: WidgetPodS<T, W>,
    padding: Vec2
}

impl<T, W: Widget<T>> Widget<T> for Padding<T, W>
{
    #[inline]
    fn update(&mut self, data: &T) -> bool
    {
        self.inner.widget.update(data)
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        event.event.offset(-self.padding);
        self.inner.widget.event(ctx, data, event);
        event.event.offset(self.padding);
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: Rect) -> Vec2
    {
        self.inner.size = self.inner.widget.layout(ctx, data, constraints - self.padding * 2.0);
        self.inner.size + self.padding * 2.0
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        ctx.painter.add_offset(self.padding);
        let size = self.inner.widget.paint(ctx, data, size - self.padding * 2.0);
        ctx.painter.add_offset(-self.padding);
        size + self.padding * 2.0
    }
}

impl<T, W: Widget<T>> Padding<T, W>
{
    pub fn new(widget: W, padding: Vec2) -> Self
    {
        Self { inner: WidgetPodS::new(widget), padding }
    }
}

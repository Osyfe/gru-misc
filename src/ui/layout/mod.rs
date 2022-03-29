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
    Fill,
    FillPadding
}

pub struct Empty<T>
{
    _phantom: PhantomData<T>
}

impl<T> Widget<T> for Empty<T>
{
    fn update(&mut self, _: &mut T) -> bool
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
    width: Option<f32>,
    height: Option<f32>
}

impl<T, W: Widget<T>> Widget<T> for Fix<T, W>
{
    #[inline]
    fn update(&mut self, data: &mut T) -> bool
    {
        self.inner.widget.update(data)
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        self.inner.widget.event(ctx, data, event);
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, mut constraints: Rect) -> Vec2
    {
        if let Some(width) = self.width
        {
            constraints.min.0 = width;
            constraints.max.0 = width;
        }
        if let Some(height) = self.height
        {
            constraints.min.1 = height;
            constraints.max.1 = height;
        }
        self.inner.widget.layout(ctx, data, constraints)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        self.inner.widget.paint(ctx, data, size)
    }
}

impl<T, W: Widget<T>> Fix<T, W>
{
    pub fn new(widget: W, width: Option<f32>, height: Option<f32>) -> Self
    {
        Self { inner: WidgetPod::new(widget), width, height }
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
    fn update(&mut self, data: &mut T) -> bool
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
            LayoutAlign::Back => margin.0,
            LayoutAlign::FillPadding => margin.0 / 4.0
        };
        self.inner.pos.1 = match self.height
        {
            LayoutAlign::Front | LayoutAlign::Fill => 0.0,
            LayoutAlign::Center => margin.1 / 2.0,
            LayoutAlign::Back => margin.1,
            LayoutAlign::FillPadding => margin.1 / 4.0
        };
        Vec2(self.inner.size.0.max(constraints.max.0), self.inner.size.1.max(constraints.max.1))
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        ctx.painter.add_offset(self.inner.pos);
        let size = self.inner.widget.paint(ctx, data, size - self.inner.pos * 2.0);
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
    front: Vec2,
    back: Vec2
}

impl<T, W: Widget<T>> Widget<T> for Padding<T, W>
{
    #[inline]
    fn update(&mut self, data: &mut T) -> bool
    {
        self.inner.widget.update(data)
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        event.event.offset(-self.front);
        self.inner.widget.event(ctx, data, event);
        event.event.offset(self.front);
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: Rect) -> Vec2
    {
        self.inner.size = self.inner.widget.layout(ctx, data, constraints - self.front - self.back);
        self.inner.size + self.front + self.back
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        ctx.painter.add_offset(self.front);
        let size = self.inner.widget.paint(ctx, data, size - self.front - self.back);
        ctx.painter.add_offset(-self.front);
        size + self.front + self.back
    }
}

impl<T, W: Widget<T>> Padding<T, W>
{
    pub fn new(widget: W, front: Vec2, back: Vec2) -> Self
    {
        Self { inner: WidgetPodS::new(widget), front, back }
    }
}

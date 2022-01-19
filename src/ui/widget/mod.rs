use super::{EventCtx, LayoutCtx, PaintCtx, Lens, Widget, event::EventPod, layout::{self, LayoutAlign}, lens, paint::{TextSize, Vec2, Rect}};
use crate::text::Align;
use std::marker::PhantomData;
use std::borrow::Borrow;

impl<'a, T> Widget<T> for Box<dyn Widget<T> + 'a>
{
    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        self.as_mut().event(ctx, data, event);
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: Rect) -> Vec2
    {
        self.as_mut().layout(ctx, data, constraints)
    }

    #[inline]
    fn paint(&self, ctx: &mut PaintCtx, data: &T, size: Vec2)
    {
        self.as_ref().paint(ctx, data, size);
    }
}

pub trait WidgetExt<T>: Widget<T> + Sized
{
    fn boxed<'a>(self) -> Box<dyn Widget<T> + 'a> where Self: 'a { Box::new(self) }
    fn lens<U, L: Lens<U, T>>(self, lens: L) -> lens::LensWrap<U, T, Self, L> { lens::LensWrap::new(self, lens) }
    fn fix(self, size: Vec2) -> layout::Fix<T, Self> { layout::Fix::new(self, size) }
    fn align(self, width: LayoutAlign, height: LayoutAlign) -> layout::Align<T, Self> { layout::Align::new(self, width, height) }
    fn padding(self, padding: Vec2) -> layout::Padding<T, Self> { layout::Padding::new(self, padding) }
}

impl<T, W: Widget<T> + Sized> WidgetExt<T> for W {}

pub struct Square<T>
{
    _phantom: PhantomData<T>
}

impl<T> Square<T>
{
    pub fn new() -> Self
    {
        Self { _phantom: PhantomData }
    }
}

impl<T> Widget<T> for Square<T>
{
    fn event(&mut self, _: &mut EventCtx, _: &mut T, _: &mut EventPod) { }

    fn layout(&mut self, _: &mut LayoutCtx, _: &T, _: Rect) -> Vec2
    {
        Vec2(1.0, 1.0)
    }

    fn paint(&self, ctx: &mut PaintCtx, _: &T, size: Vec2)
    {
        ctx.painter.draw_rect(Rect::new_origin(size), (0.0, 0.0, 0.0, 1.0));
    }
}

pub struct Label<S: Borrow<str>, T>
{
    text: S,
    size: TextSize,
    align: Align,
    _phantom: PhantomData<T>
}

impl<S: Borrow<str>, T> Label<S, T>
{
    pub fn new(text: S, size: TextSize, align: Align) -> Self
    {
        if align == Align::Block { panic!("Label::new: Labels cannot have a block align."); }
        Self { text, size, align, _phantom: PhantomData }
    }
}

impl<S: Borrow<str>, T> Widget<T> for Label<S, T>
{
    fn event(&mut self, _: &mut EventCtx, _: &mut T, _: &mut EventPod) { }

    fn layout(&mut self, ctx: &mut LayoutCtx, _: &T, _: Rect) -> Vec2
    {
        let width = ctx.text_width(self.text.borrow(), self.size);
        let height = self.size.scale();
        Vec2(width, height)
    }

    fn paint(&self, ctx: &mut PaintCtx, _: &T, size: Vec2)
    {
        ctx.painter.draw_text(Rect::new_origin(size), self.text.borrow(), self.size, self.align, false, (0.0, 0.0, 0.0, 1.0));
    }
}

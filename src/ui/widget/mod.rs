use super::{Widget, EventCtx, LayoutCtx, PaintCtx, WidgetState, Lens, event::{Key, Event, EventPod, MouseButton}, layout::{self, LayoutAlign}, lens, interact, paint::{TextSize, Vec2, Rect}, WidgetPod};
use crate::text::Align;
use std::marker::PhantomData;
use std::borrow::Borrow;

impl<'a, T> Widget<T> for Box<dyn Widget<T> + 'a>
{
    #[inline]
    fn update(&mut self, data: &T) -> bool
    {
        self.as_mut().update(data)
    }

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
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        self.as_mut().paint(ctx, data, size)
    }

    #[inline]
    fn response(&mut self, data: &mut T, button: Option<MouseButton>) -> bool
    {
        self.as_mut().response(data, button)
    }
}

pub trait WidgetExt<T>: Widget<T> + Sized
{
    fn boxed<'a>(self) -> Box<dyn Widget<T> + 'a> where Self: 'a { Box::new(self) }
    fn lens<U, L: Lens<U, T>>(self, lens: L) -> lens::LensWrap<U, T, Self, L> { lens::LensWrap::new(self, lens) }
    fn fix(self, size: Vec2) -> layout::Fix<T, Self> { layout::Fix::new(self, size) }
    fn align(self, width: LayoutAlign, height: LayoutAlign) -> layout::Align<T, Self> { layout::Align::new(self, width, height) }
    fn padding(self, padding: Vec2) -> layout::Padding<T, Self> { layout::Padding::new(self, padding) }
    fn bg(self) -> Bg<T, Self> { Bg::new(self) }
    fn watch(self) -> interact::Watch<T, Self> where T: Clone + PartialEq { interact::Watch::new(self) }
    fn response<'a>(self, action: Option<Box<dyn FnMut() + 'a>>) -> interact::Response<'a, T, Self> where Self: 'a { interact::Response::new(self, action) }
}

impl<T, W: Widget<T> + Sized> WidgetExt<T> for W {}

pub struct Bg<T, W: Widget<T>>
{
    inner: WidgetPod<T, W>
}

impl<T, W: Widget<T>> Bg<T, W>
{
    pub fn new(widget: W) -> Self
    {
        Self { inner: WidgetPod::new(widget) }
    }
}

impl<T, W: Widget<T>> Widget<T> for Bg<T, W>
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
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: Rect) -> Vec2
    {
        self.inner.widget.layout(ctx, data, constraints)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        let color = match ctx.state
        {
            WidgetState::Cold => (0.3, 0.3, 0.3, 1.0),
            WidgetState::Hot => (0.5, 0.3, 0.3, 1.0),
            WidgetState::Hover => (0.3, 0.3, 0.5, 1.0)
        };
        ctx.painter.draw_rect(Rect::new_origin(size), color);
        self.inner.widget.paint(ctx, data, size);
        size
    }
}

pub struct Label<S: Borrow<str>, T>
{
    text: S,
    text_size: TextSize,
    size: Vec2,
    align: Align,
    _phantom: PhantomData<T>
}

impl<S: Borrow<str>, T> Label<S, T>
{
    pub fn new(text: S, size: TextSize, align: Align) -> Self
    {
        if align == Align::Block { panic!("Label::new: Labels cannot have a block align."); }
        Self { text, text_size: size, size: Vec2(0.0, 0.0), align, _phantom: PhantomData }
    }
}

impl<S: Borrow<str>, T> Widget<T> for Label<S, T>
{
    #[inline]
    fn update(&mut self, _: &T) -> bool
    {
        false
    }

    #[inline]
    fn event(&mut self, _: &mut EventCtx, _: &mut T, _: &mut EventPod) { }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, _: &T, _: Rect) -> Vec2
    {
        let width = ctx.text_width(self.text.borrow(), self.text_size);
        let height = self.text_size.scale();
        self.size = Vec2(width, height);
        self.size
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, _: &T, size: Vec2) -> Vec2
    {
        ctx.painter.draw_text(Rect::new_origin(size), self.text.borrow(), self.text_size, self.align, false, (0.0, 0.0, 0.0, 1.0));
        self.size
    }
}

pub struct Check;

impl Widget<bool> for Check
{
    #[inline]
    fn update(&mut self, _: &bool) -> bool
    {
        false
    }

    #[inline]
    fn event(&mut self, _: &mut EventCtx, _: &mut bool, _: &mut EventPod) { }

    #[inline]
    fn layout(&mut self, _: &mut LayoutCtx, _: &bool, _: Rect) -> Vec2
    {
        Vec2(1.0, 1.0)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, flag: &bool, _: Vec2)-> Vec2
    {
        let color = match ctx.state
        {
            WidgetState::Cold => (0.0, 0.0, 0.0, 1.0),
            WidgetState::Hot => (0.3, 0.0, 0.0, 1.0),
            WidgetState::Hover => (0.0, 0.0, 0.3, 1.0)
        };
        ctx.painter.draw_rect(Rect::new_origin(Vec2(1.0, 1.0)), color);
        ctx.painter.draw_rect(Rect::new_size(Vec2(0.2, 0.2), Vec2(0.6, 0.6)), (0.8, 0.8, 0.8, 1.0));
        if *flag { ctx.painter.draw_rect(Rect::new_size(Vec2(0.3, 0.3), Vec2(0.4, 0.4)), color); }
        Vec2(1.0, 1.0)
    }

    #[inline]
    fn response(&mut self, flag: &mut bool, button: Option<MouseButton>) -> bool
    {
        if button == Some(MouseButton::Primary) { *flag = !*flag; true } else { false }
    }
}

pub struct Edit
{
    active: bool,
    shift: bool
}

impl Widget<String> for Edit
{
    #[inline]
    fn update(&mut self, _: &String) -> bool
    {
        false
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut String, event: &mut EventPod)
    {
        if self.active && !event.used
        {
            if let Event::Key { key, pressed } = event.event
            {
                event.used = true;
                match key
                {
                    Key::LShift | Key::RShift => self.shift = pressed,
                    Key::Back => if pressed
                    {
                        data.pop();
                        ctx.request_update();
                    },
                    _ => {}
                }
                if pressed
                {
                    if let Some(ch) = key.ch(self.shift)
                    {
                        data.push(ch);
                        ctx.request_update();
                    }
                }
            }
        }
    }

    #[inline]
    fn layout(&mut self, _: &mut LayoutCtx, _: &String, size: Rect) -> Vec2
    {
        let height = TextSize::Normal.scale();
        Vec2(size.max.0, height)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, size: Vec2) -> Vec2
    {
        let rect = Rect::new_origin(size);
        let color = if self.active { (0.0, 0.3, 0.3, 1.0) } else { (0.0, 0.0, 0.3, 1.0) };
        ctx.painter.draw_rect(rect, color);
        ctx.painter.draw_text(Rect::new_origin(size), data, TextSize::Normal, Align::Left, false, (1.0, 1.0, 1.0, 1.0));
        size
    }

    #[inline]
    fn response(&mut self, _: &mut String, button: Option<MouseButton>) -> bool
    {
        self.active = button.is_some();
        true
    }
}

impl Edit
{
    pub fn new() -> Self
    {
        Self { active: true, shift: false }
    }
}

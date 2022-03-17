use super::{Widget, EventCtx, LayoutCtx, PaintCtx, WidgetState, Lens, event::{Key, Event, EventPod, MouseButton}, layout::{self, LayoutAlign}, lens, interact, paint::{TextSize, Vec2, Rect}, WidgetPod};
use crate::text::Align;
use std::{marker::PhantomData, borrow::Borrow};

pub trait WidgetExt<T>: Widget<T> + Sized
{
    fn boxed<'a>(self) -> Box<dyn Widget<T> + 'a> where Self: 'a { Box::new(self) }
    fn owning<U>(self, data: T) -> Owning<U, T, Self> { Owning::new(self, data) }
    fn lens<U, L: Lens<U, T>>(self, lens: L) -> lens::LensWrap<U, T, Self, L> { lens::LensWrap::new(self, lens) }
    fn fix(self, width: Option<f32>, height: Option<f32>) -> layout::Fix<T, Self> { layout::Fix::new(self, width, height) }
    fn align(self, width: LayoutAlign, height: LayoutAlign) -> layout::Align<T, Self> { layout::Align::new(self, width, height) }
    fn padding(self, front: Vec2, back: Vec2) -> layout::Padding<T, Self> { layout::Padding::new(self, front, back) }
    fn bg(self) -> Bg<T, Self> { Bg::new(self) }
    fn watch(self) -> interact::Watch<T, Self> where T: Clone + PartialEq { interact::Watch::new(self) }
    fn response<'a>(self, action: Option<Box<dyn FnMut() + 'a>>) -> interact::Response<'a, T, Self> where Self: 'a { interact::Response::new(self, action) }
}

impl<T, W: Widget<T> + Sized> WidgetExt<T> for W {}

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

pub struct Owning<U, T, W: Widget<T>>
{
    inner: WidgetPod<T, W>,
    data: T,
    _phantom: PhantomData<U>
}

impl<U, T, W: Widget<T>> Widget<U> for Owning<U, T, W>
{
    #[inline]
    fn update(&mut self, _: &U) -> bool
    {
        self.inner.widget.update(&self.data)
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, _: &mut U, event: &mut EventPod)
    {
        self.inner.widget.event(ctx, &mut self.data, event)
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, _: &U, constraints: Rect) -> Vec2
    {
        self.inner.widget.layout(ctx, &self.data, constraints)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, _: &U, size: Vec2) -> Vec2
    {
        self.inner.widget.paint(ctx, &self.data, size)
    }
}

impl<U, T, W: Widget<T>> Owning<U, T, W>
{
    pub fn new(widget: W, data: T) -> Self
    {
        Self { inner: WidgetPod::new(widget), data, _phantom: PhantomData }
    }
}

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

pub struct Label<T: Borrow<str>>
{
    text_size: TextSize,
    size: Vec2,
    align: Align,
    _phantom: PhantomData<T>
}

impl<T: Borrow<str>> Widget<T> for Label<T>
{
    #[inline]
    fn update(&mut self, _: &T) -> bool
    {
        false
    }

    #[inline]
    fn event(&mut self, _: &mut EventCtx, _: &mut T, _: &mut EventPod) { }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, _: Rect) -> Vec2
    {
        let width = ctx.text_width(data.borrow(), self.text_size);
        let height = self.text_size.scale();
        self.size = Vec2(width, height);
        self.size
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        ctx.painter.draw_text(Rect::new_origin(size), data.borrow(), self.text_size, self.align, false, (0.0, 0.0, 0.0, 1.0));
        self.size
    }
}

impl<T: Borrow<str>> Label<T>
{
    pub fn new(size: TextSize, align: Align) -> Self
    {
        if align == Align::Block { panic!("Label::new: Labels cannot have a block align."); }
        Self { text_size: size, size: Vec2(0.0, 0.0), align, _phantom: PhantomData }
    }
}

pub struct Check
{
    size: TextSize
}

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
        let size = self.size.scale();
        Vec2(size, size)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, flag: &bool, _: Vec2)-> Vec2
    {
        let size1 = self.size.scale();
        let (size2, size3) = (size1 * 0.2, size1 * 0.6);
        let (size4, size5) = (size1 * 0.3, size1 * 0.4);
        let color = match ctx.state
        {
            WidgetState::Cold => (0.0, 0.0, 0.0, 1.0),
            WidgetState::Hot => (0.3, 0.0, 0.0, 1.0),
            WidgetState::Hover => (0.0, 0.0, 0.3, 1.0)
        };
        ctx.painter.draw_rect(Rect::new_origin(Vec2(size1, size1)), color);
        ctx.painter.draw_rect(Rect::new_size(Vec2(size2, size2), Vec2(size3, size3)), (0.8, 0.8, 0.8, 1.0));
        if *flag { ctx.painter.draw_rect(Rect::new_size(Vec2(size4, size4), Vec2(size5, size5)), color); }
        Vec2(size1, size1)
    }

    #[inline]
    fn response(&mut self, flag: &mut bool, button: Option<MouseButton>) -> bool
    {
        if button == Some(MouseButton::Primary) { *flag = !*flag; true } else { false }
    }
}

impl Check
{
    pub fn new(size: TextSize) -> Self
    {
        Self { size }
    }
}

pub struct Edit
{
    size: TextSize,
    active: bool,
    filter: Box<dyn FnMut(char) -> bool>
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
            if let Event::Char(ch) = event.event
            {
                event.used = true;
                if (self.filter)(ch) { data.push(ch); }
                ctx.request_update();
            }
            if let Event::Key { key: Key::Back, pressed: true } = event.event
            {
                event.used;
                data.pop();
                ctx.request_update();
            }
        }
    }

    #[inline]
    fn layout(&mut self, _: &mut LayoutCtx, _: &String, size: Rect) -> Vec2
    {
        let height = self.size.scale();
        Vec2(size.max.0, height)
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, size: Vec2) -> Vec2
    {
        let rect = Rect::new_origin(size);
        let color = if self.active { (0.0, 0.3, 0.3, 1.0) } else { (0.0, 0.0, 0.3, 1.0) };
        ctx.painter.draw_rect(rect, color);
        ctx.painter.draw_text(Rect::new_origin(size), data, self.size, Align::Left, false, (1.0, 1.0, 1.0, 1.0));
        size
    }

    #[inline]
    fn response(&mut self, data: &mut String, button: Option<MouseButton>) -> bool
    {
        self.active = button.is_some();
        if let Some(MouseButton::Secondary) = button
        {
            use copypasta::{ClipboardContext, ClipboardProvider};
            let mut ctx = ClipboardContext::new().unwrap();
            for ch in ctx.get_contents().unwrap().chars().filter(|ch| (self.filter)(*ch)) { data.push(ch); }
        }
        true
    }
}

impl Edit
{
    pub fn new(size: TextSize, filter: Option<Box<dyn FnMut(char) -> bool>>) -> Self
    {
        let filter = filter.unwrap_or(Box::new(|_| true));
        Self { size, active: false, filter }
    }
}

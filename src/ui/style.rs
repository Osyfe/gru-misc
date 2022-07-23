use super::{paint::{Color, Vec2, Rect}, event::{EventPod, MouseButton}, interact::WidgetState, EventCtx, LayoutCtx, PaintCtx, Widget, pods::WidgetPod};

#[derive(Clone)]
pub struct ColorSet
{
    pub cold: Color,
    pub hot: Color,
    pub hover: Color
}

impl ColorSet
{
    pub const fn uniform(color: Color) -> Self
    {
        Self { cold: color, hot: color, hover: color }
    }

    pub fn get(&self, state: WidgetState) -> Color
    {
        match state
        {
            WidgetState::Cold => self.cold,
            WidgetState::Hot => self.hot,
            WidgetState::Hover => self.hover
        }
    }
}

#[derive(Clone)]
pub struct StyleSet
{
    pub bg: ColorSet,
    pub text: ColorSet,
    pub data: ColorSet,
    pub top: Color
}

impl Default for StyleSet
{
    fn default() -> Self
    {
        Self
        {
            bg: ColorSet
            {
                cold: Color::from_discrete_srgb(100, 100, 100, 255),
                hot: Color::from_discrete_srgb(200, 150, 100, 255),
                hover: Color::from_discrete_srgb(100, 150, 200, 255)
            },
            text: ColorSet
            {
                cold: Color::from_discrete_srgb(0, 0, 0, 255),
                hot: Color::from_discrete_srgb(0, 0, 0, 255),
                hover: Color::from_discrete_srgb(0, 0, 0, 255)
            },
            data: ColorSet
            {
                cold: Color::from_discrete_srgb(0, 0, 0, 255),
                hot: Color::from_discrete_srgb(250, 200, 200, 255),
                hover: Color::from_discrete_srgb(50, 150, 50, 255)
            },
            top: Color::from_discrete_srgb(150, 150, 150, 255)
        }
    }
}

pub struct Style<T, W: Widget<T>, F: Fn(&mut StyleSet)>
{
    inner: WidgetPod<T, W>,
    styler: F
}

impl<T, W: Widget<T>, F: Fn(&mut StyleSet)> Widget<T> for Style<T, W, F>
{
    #[inline]
    fn update(&mut self, data: &mut T) -> bool
    {
        self.inner.widget.update(data)
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
        let old_style = ctx.style.clone();
        (self.styler)(&mut ctx.style);
        let ret = self.inner.widget.paint(ctx, data, size);
        ctx.style = old_style;
        ret
    }

    #[inline]
    fn response(&mut self, data: &mut T, button: Option<MouseButton>) -> bool
    {
        self.inner.widget.response(data, button)
    }
}

impl<T, W: Widget<T>, F: Fn(&mut StyleSet)> Style<T, W, F>
{
    pub fn new(widget: W, styler: F) -> Self
    {
        Self { inner: WidgetPod::new(widget), styler }
    }
}

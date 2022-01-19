use super::*;

pub struct Split<'a, T, const ROW: bool, const N: usize>
{
    widgets: [WidgetBox<'a, T>; N],
    weights: [f32; N]
}

impl<'a, T, const ROW: bool, const N: usize> Widget<T> for Split<'a, T, ROW, N>
{
    fn event(&mut self, _: &mut EventCtx, _: &mut T, _: &mut EventPod)
    {

    }

    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: Rect) -> Vec2
    {
        let mut size = if ROW { Vec2(0.0, constraints.max.1) } else { Vec2(constraints.max.0, 0.0) };
        for (WidgetPod { widget, pos: w_pos, size: w_size, .. }, weight) in self.widgets.iter_mut().zip(self.weights.iter())
        {
            let max =
                if ROW { Vec2(constraints.max.0 * *weight, constraints.max.1) }
                else { Vec2(constraints.max.0, constraints.max.1 * *weight) };
            *w_size = widget.layout(ctx, data, Rect::new_origin(max));
            if ROW
            {
                *w_pos = Vec2(size.0, 0.0);
                size.0 += w_size.0.max(max.0);
                size.1 = w_size.1.max(size.1);
                
            } else
            {
                *w_pos = Vec2(0.0, size.1);
                size.0 = w_size.0.max(size.0);
                size.1 += w_size.1.max(max.1);
            }
        }
        size
    }

    fn paint(&self, ctx: &mut PaintCtx, data: &T, _: Vec2)
    {
        for WidgetPod { widget, pos: w_pos, size: w_size, .. } in &self.widgets
        {
            ctx.painter.add_offset(*w_pos);
            widget.paint(ctx, data, *w_size);
            ctx.painter.add_offset(-*w_pos);
        }
    }
}

impl<'a, T, const ROW: bool, const N: usize> Split<'a, T, ROW, N>
{
    fn new(widgets: [Box<dyn Widget<T> + 'a>; N], weights: Option<[f32; N]>) -> Self
    {
        let widgets = widgets.map(|widget| WidgetPod::new(widget));
        let weights = weights.unwrap_or([1.0; N]);
        let weight_norm: f32 = weights.iter().sum();
        let weights = weights.map(|weight| weight / weight_norm);
        Self { widgets, weights }
    }
}

impl<'a, T, const N: usize> Split<'a, T, true, N>
{
    pub fn row(widgets: [Box<dyn Widget<T> + 'a>; N], weights: Option<[f32; N]>) -> Self
    {
        Self::new(widgets, weights)
    }
}

impl<'a, T, const N: usize> Split<'a, T, false, N>
{
    pub fn column(widgets: [Box<dyn Widget<T> + 'a>; N], weights: Option<[f32; N]>) -> Self
    {
        Self::new(widgets, weights)
    }
}

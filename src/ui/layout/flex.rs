use super::*;

pub struct Flex<'a, T, const ROW: bool>
{
    widgets: Vec<WidgetBoxP<'a, T>>,
    padding: f32,
    primary_align: LayoutAlign,
    secondary_align: LayoutAlign
}

impl<'a, T, const ROW: bool> Widget<T> for Flex<'a, T, ROW>
{
    fn update(&mut self, data: &mut T) -> bool
    {
        self.widgets.iter_mut().map(|pod| pod.widget.update(data)).any(std::convert::identity)
    }

    fn event(&mut self, ctx: &mut EventCtx, data: &mut T, event: &mut EventPod)
    {
        for WidgetPodP { widget, pos: w_pos, .. } in &mut self.widgets
        {
            event.event.offset(-*w_pos);
            widget.event(ctx, data, event);
            event.event.offset(*w_pos);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, data: &T, constraints: Rect) -> Vec2
    {
        if self.widgets.len() == 0 { return constraints.min; }
        let mut size = Vec2(0.0, 0.0);
        for WidgetPodP { widget, size: w_size, .. } in &mut self.widgets
        {
            let min =
                if ROW { Vec2(0.0, constraints.min.1) }
                else { Vec2(constraints.min.0, 0.0) };
            let max =
                if ROW { Vec2(constraints.max.0 - size.0, constraints.max.1) }
                else { Vec2(constraints.max.0, constraints.max.1 - size.1) };
            *w_size = widget.layout(ctx, data, Rect { min, max });
            if ROW
            {
                size.0 += w_size.0;
                size.1 = size.1.max(w_size.1);
            } else
            {
                size.0 = size.0.max(w_size.0);
                size.1 += w_size.1
            }
        }
        let margin =
            if ROW { (constraints.min.0 - size.0 - self.padding * (self.widgets.len() as f32 - 1.0)).max(0.0) }
            else { (constraints.min.1 - size.1 - self.padding * (self.widgets.len() as f32 - 1.0)).max(0.0) };
        let (pad_front, pad_mid, pad_back) = match self.primary_align
        {
            LayoutAlign::Front => (0.0, 0.0, margin),
            LayoutAlign::Center => (margin / 2.0, 0.0, margin / 2.0),
            LayoutAlign::Back => (margin, 0.0, 0.0),
            LayoutAlign::Fill => if self.widgets.len() == 1 { (margin / 2.0, 0.0, margin / 2.0) } else { (0.0, margin / (self.widgets.len() as f32 - 1.0), 0.0) },
            LayoutAlign::FillPadding =>
            {
                let single_margin = margin / (self.widgets.len() as f32 + 1.0);
                (single_margin, single_margin, single_margin)
            }
        };
        if ROW
        {
            size.0 = pad_front.max(0.0);
            size.1 = size.1.max(constraints.min.1);
        } else
        {
            size.0 = size.0.max(constraints.min.0);
            size.1 = pad_front.max(0.0);
        }
        let secondary_size = if ROW { size.1 } else { size.0 };
        for WidgetPodP { pos: w_pos, size: w_size, .. } in &mut self.widgets
        {
            *w_pos = Vec2(0.0, 0.0);
            let (w_secondary_pos, w_secondary_size) = if ROW { (&mut w_pos.1, &mut w_size.1) } else { (&mut w_pos.0, &mut w_size.0) };
            match self.secondary_align
            {
                LayoutAlign::Front => {},
                LayoutAlign::Center => *w_secondary_pos += (secondary_size - *w_secondary_size) / 2.0,
                LayoutAlign::Back => *w_secondary_pos += secondary_size - *w_secondary_size,
                LayoutAlign::Fill => *w_secondary_size = secondary_size,
                LayoutAlign::FillPadding =>
                {
                    let margin = secondary_size - *w_secondary_size;
                    if margin < 0.0 { *w_secondary_pos += margin / 2.0; }
                    else
                    {
                        *w_secondary_pos += margin / 4.0;
                        *w_secondary_size = secondary_size - margin / 2.0;
                    }
                }
            };
            if ROW
            {
                w_pos.0 = size.0;
                size.0 += w_size.0 + (pad_mid + self.padding).max(0.0);
            } else
            {
                w_pos.1 = size.1;
                size.1 += w_size.1 + (pad_mid + self.padding).max(0.0);
            }
        }
        if ROW { size.0 = -self.padding + pad_back.max(0.0) }
        else { size.1 = -self.padding + pad_back.max(0.0); }
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, size: Vec2) -> Vec2
    {
        for WidgetPodP { widget, pos: w_pos, size: w_size, .. } in &mut self.widgets
        {
            ctx.painter.add_offset(*w_pos);
            widget.paint(ctx, data, *w_size);
            ctx.painter.add_offset(-*w_pos);
        }
        size
    }
}

impl<'a, T, const ROW: bool> Flex<'a, T, ROW>
{
    fn new(padding: f32, primary_align: LayoutAlign, secondary_align: LayoutAlign) -> Self
    {
        Self { widgets: Vec::new(), padding, primary_align, secondary_align }        
    }

    pub fn add_box(&mut self, widget: Box<dyn Widget<T> + 'a>) -> &mut Self
    {
        self.widgets.push(WidgetPodP::new(widget));
        self
    }

    pub fn with_box(mut self, widget: Box<dyn Widget<T> + 'a>) -> Self
    {
        self.widgets.push(WidgetPodP::new(widget));
        self
    }

    pub fn add<W: Widget<T> + 'a>(&mut self, widget: W) -> &mut Self
    {
        self.add_box(Box::new(widget))
    }

    pub fn with<W: Widget<T> + 'a>(self, widget: W) -> Self
    {
        self.with_box(Box::new(widget))
    }
}

impl<'a, T> Flex<'a, T, true>
{
    pub fn row(padding: f32, primary_align: LayoutAlign, secondary_align: LayoutAlign) -> Self
    {
        Self::new(padding, primary_align, secondary_align)
    }
}

impl<'a, T> Flex<'a, T, false>
{
    pub fn column(padding: f32, primary_align: LayoutAlign, secondary_align: LayoutAlign) -> Self
    {
        Self::new(padding, primary_align, secondary_align)
    }
}

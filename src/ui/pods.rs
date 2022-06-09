use super::*;

pub struct WidgetPod<T, W: Widget<T>>
{
    pub widget: W,
    _phantom: PhantomData<T>
}

impl<T, W: Widget<T>> WidgetPod<T, W>
{
    pub fn new(widget: W) -> Self
    {
        Self { widget, _phantom: PhantomData }
    }
}

pub struct WidgetPodS<T, W: Widget<T>>
{
    pub widget: W,
    pub size: paint::Vec2,
    _phantom: PhantomData<T>
}

impl<T, W: Widget<T>> WidgetPodS<T, W>
{
    pub fn new(widget: W) -> Self
    {
        Self { widget, _phantom: PhantomData, size: paint::Vec2(0.0, 0.0) }
    }
}

pub struct WidgetPodP<T, W: Widget<T>>
{
    pub widget: W,
    pub pos: paint::Vec2,
    pub size: paint::Vec2,
    _phantom: PhantomData<T>
}

impl<T, W: Widget<T>> WidgetPodP<T, W>
{
    pub fn new(widget: W) -> Self
    {
        Self { widget, _phantom: PhantomData, pos: paint::Vec2(0.0, 0.0), size: paint::Vec2(0.0, 0.0) }
    }
}

//type WidgetBox<'a, T> = WidgetPod<T, Box<dyn Widget<T> + 'a>>;
//type WidgetBoxS<'a, T> = WidgetPod<T, Box<dyn Widget<T> + 'a>>;
pub type WidgetBoxP<'a, T> = WidgetPodP<T, Box<dyn Widget<T> + 'a>>;

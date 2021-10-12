mod lens;

pub use lens::*;

use super::math::*;
use super::draw::Painter;

pub trait Widget<T>
{
    fn layout(&self, data: &T, bounds: Vec2) -> Vec2;
    fn draw(&self, data: &T, painter: &mut Painter);
}

pub trait WidgetExt<T>: Widget<T> + Sized
{
    fn lens<U, L: lens::Lens<U, T>>(self, lens: L) -> lens::LensWrap<U, T, Self, L>
    {
        lens::LensWrap::new(self, lens)
    }
}

impl<T, W: Widget<T> + Sized> WidgetExt<T> for W {}

pub trait WidgetExtUnit: Widget<()> + Sized
{
    fn lens_unit<U>(self) -> lens::LensWrap<U, (), Self, lens::LensUnit>
    {
        lens::LensWrap::new(self, lens::LensUnit)
    }
}

impl<W: Widget<()> + Sized> WidgetExtUnit for W {}

pub struct Square;

impl Widget<()> for Square
{
    fn layout(&self, _: &(), bounds: Vec2) -> Vec2
    {
        Vec2(100.0, 100.0)
    }

    fn draw(&self, _: &(), painter: &mut Painter)
    {
        painter.draw_rect(Rect::new_origin(Vec2(100.0, 100.0)));
    }
}

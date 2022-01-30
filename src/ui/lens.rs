use super::{Widget, EventCtx, LayoutCtx, PaintCtx, Lens, event::{EventPod, MouseButton}, paint::{Vec2, Rect}};
use std::marker::PhantomData;

pub trait LensExt<U, T>: Lens<U, T> + Sized
{
    #[inline]
    fn chain<S, L2: Lens<T, S>>(self, lens2: L2) -> LensChain<U, T, S, Self, L2>
    {
        LensChain::new(self, lens2)
    }
}

impl<U, T, L: Lens<U, T>> LensExt<U, T> for L {}

pub struct LensWrap<U, T, W: Widget<T>, L: Lens<U, T>>
{
    inner: W,
    lens: L,
    _phantom_u: PhantomData<U>,
    _phantom_t: PhantomData<T>
}

impl<U, T, W: Widget<T>, L: Lens<U, T>> LensWrap<U, T, W, L>
{
    #[inline]
    pub fn new(inner: W, lens: L) -> Self
    {
        Self { inner, lens, _phantom_u: PhantomData, _phantom_t: PhantomData }
    }
}

impl<U, T, W: Widget<T>, L: Lens<U, T>> Widget<U> for LensWrap<U, T, W, L>
{
    #[inline]
    fn update(&mut self, data: &U) -> bool
    {
        self.lens.with(data, |data| self.inner.update(data))
    }

    #[inline]
    fn event(&mut self, ctx: &mut EventCtx, data: &mut U, event: &mut EventPod)
    {
        self.lens.with_mut(data, |data| self.inner.event(ctx, data, event))
    }

    #[inline]
    fn layout(&mut self, ctx: &mut LayoutCtx, data: &U, constraints: Rect) -> Vec2
    {
        self.lens.with(data, |data| self.inner.layout(ctx, data, constraints))
    }

    #[inline]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &U, size: Vec2) -> Vec2
    {
        self.lens.with(data, |data| self.inner.paint(ctx, data, size))
    }

    #[inline]
    fn response(&mut self, data: &mut U, button: Option<MouseButton>) -> bool
    {
        self.lens.with_mut(data, |data| self.inner.response(data, button))
    }
}

pub struct LensChain<V, U, T, L1: Lens<V, U>, L2: Lens<U, T>>
{
    lens1: L1,
    lens2: L2,
    _phantom_v: PhantomData<V>,
    _phantom_u: PhantomData<U>,
    _phantom_t: PhantomData<T>
}

impl<V, U, T, L1: Lens<V, U>, L2: Lens<U, T>> LensChain<V, U, T, L1, L2>
{
    #[inline]
    pub fn new(lens1: L1, lens2: L2) -> Self
    {
        Self { lens1, lens2, _phantom_v: PhantomData, _phantom_u: PhantomData, _phantom_t: PhantomData }
    }
}

impl<V, U, T, L1: Lens<V, U>, L2: Lens<U, T>> Lens<V, T> for LensChain<V, U, T, L1, L2>
{
    #[inline]
    fn with<A, F: FnOnce(&T) -> A>(&self, data: &V, f: F) -> A
    {
        self.lens1.with(data, |data| self.lens2.with(data, |data| f(data)))
    }

    #[inline]
    fn with_mut<A, F: FnOnce(&mut T) -> A>(&mut self, data: &mut V, f: F) -> A
    {
        self.lens1.with_mut(data, |data| self.lens2.with_mut(data, |data| f(data)))
    }
}

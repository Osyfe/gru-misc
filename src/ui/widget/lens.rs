use super::{Widget, Vec2, super::draw::Painter};
use std::marker::PhantomData;

pub trait Lens<U, T>: Sized
{
    fn with<A, F: FnOnce(&T) -> A>(&self, data: &U, f: F) -> A;
    fn with_mut<A, F: FnOnce(&mut T) -> A>(&self, data: &mut U, f: F) -> A;
}

pub use gru_ui_derive::Lens;

pub struct LensUnit;

impl<U> Lens<U, ()> for LensUnit
{
    #[inline]
    fn with<A, F: FnOnce(&()) -> A>(&self, _: &U, f: F) -> A
    {
        f(&())
    }

    #[inline]
    fn with_mut<A, F: FnOnce(&mut ()) -> A>(&self, _: &mut U, f: F) -> A
    {
        f(&mut ())
    }
}

pub trait LensExt<U, T>: Lens<U, T>
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
    fn layout(&self, data: &U, bounds: Vec2) -> Vec2
    {
        self.lens.with(data, |data| self.inner.layout(data, bounds))
    }

    #[inline]
    fn draw(&self, data: &U, painter: &mut Painter)
    {
        self.lens.with(data, |data| self.inner.draw(data, painter));
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
    fn with_mut<A, F: FnOnce(&mut T) -> A>(&self, data: &mut V, f: F) -> A
    {
        self.lens1.with_mut(data, |data| self.lens2.with_mut(data, |data| f(data)))
    }
}

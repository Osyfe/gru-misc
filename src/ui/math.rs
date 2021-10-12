pub use crate::math::Vec2;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Clone, Copy)]
pub struct Rect
{
    pub min: Vec2,
    pub max: Vec2
}

impl Rect
{
    #[inline]
    pub fn new_origin(max: Vec2) -> Self
    {
        Self { min: Vec2(0.0, 0.0), max }
    }

    #[inline]
    pub fn new_size(min: Vec2, size: Vec2) -> Self
    {
        Self { min, max: min + size }
    }

    #[inline]
    pub fn new_size_origin(size: Vec2) -> Self
    {
        Self { min: Vec2(0.0, 0.0), max: size }
    }

    #[inline]
    pub fn size(self) -> Vec2
    {
        self.max - self.min
    }

    #[inline]
    pub fn contains_l1(self, pos: Vec2) -> bool
    {
        self.min.0 <= pos.0 && pos.0 <= self.max.0
     && self.min.1 <= pos.1 && pos.1 <= self.max.1
    }

    #[inline]
    pub fn contains_l2(self, pos: Vec2) -> bool
    {
        let norm = (pos - self.min).component_div(self.max - self.min) - Vec2(0.5, 0.5);
        norm.norm_sqr() <= 0.25
    }

    #[inline]
    pub fn intersect(self, other: Self) -> bool
    {
        self.min.0 <= other.max.0 && other.min.0 <= self.max.0
     && self.min.1 <= other.max.1 && other.min.1 <= self.max.1
    }
}

impl Add<Vec2> for Rect
{
    type Output = Self;
    #[inline]
    fn add(self, offset: Vec2) -> Self
    {
        Self { min: self.min + offset, max: self.max + offset }
    }
}

impl AddAssign<Vec2> for Rect
{
    #[inline]
    fn add_assign(&mut self, offset: Vec2)
    {
        self.min += offset;
        self.max += offset;
    }
}

impl Sub<Vec2> for Rect
{
    type Output = Self;
    #[inline]
    fn sub(self, offset: Vec2) -> Self
    {
        Self { min: self.min - offset, max: self.max - offset }
    }
}

impl SubAssign<Vec2> for Rect
{
    #[inline]
    fn sub_assign(&mut self, offset: Vec2)
    {
        self.min -= offset;
        self.max -= offset;
    }
}

impl Mul<f32> for Rect
{
    type Output = Self;
    #[inline]
    fn mul(self, scale: f32) -> Self
    {
        Self { min: self.min * scale, max: self.max * scale }
    }
}

impl MulAssign<f32> for Rect
{
    #[inline]
    fn mul_assign(&mut self, scale: f32)
    {
        self.min *= scale;
        self.max *= scale;
    }
}

impl Div<f32> for Rect
{
    type Output = Self;
    #[inline]
    fn div(self, scale: f32) -> Self
    {
        Self { min: self.min / scale, max: self.max / scale }
    }
}

impl DivAssign<f32> for Rect
{
    #[inline]
    fn div_assign(&mut self, scale: f32)
    {
        self.min /= scale;
        self.max /= scale;
    }
}

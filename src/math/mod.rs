use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt::{Display, Formatter, Result};

const DIV_CUT: f32 = 0.001;

macro_rules! impl_add_assign
{
    ($ty:ident, $ty2:ident) =>
    {
		impl AddAssign<$ty2> for $ty
		{
			#[inline]
		    fn add_assign(&mut self, other: $ty2)
		    {
		        *self = *self + other
		    }
		}
    }
}

macro_rules! impl_sub_assign
{
    ($ty:ident, $ty2:ident) =>
    {
		impl SubAssign<$ty2> for $ty
		{
			#[inline]
		    fn sub_assign(&mut self, other: $ty2)
		    {
		        *self = *self - other
		    }
		}
    }
}

macro_rules! impl_mul_assign
{
    ($ty:ident, $ty2:ident) =>
    {
		impl MulAssign<$ty2> for $ty
		{
			#[inline]
		    fn mul_assign(&mut self, other: $ty2)
		    {
		        *self = *self * other
		    }
		}
    }
}

macro_rules! impl_div_assign
{
    ($ty:ident, $ty2:ident) =>
    {
		impl DivAssign<$ty2> for $ty
		{
			#[inline]
		    fn div_assign(&mut self, other: $ty2)
		    {
		        *self = *self / other
		    }
		}
    }
}

mod vector;
mod matrix;
mod rotor;

pub use vector::*;
pub use matrix::*;
pub use rotor::*;

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
    pub fn size(self) -> Vec2
    {
        self.max - self.min
    }

    #[inline]
    pub fn width(self) -> f32
    {
        self.max.0 - self.min.0
    }

    #[inline]
    pub fn height(self) -> f32
    {
        self.max.1 - self.min.1
    }
	
	#[inline]
	pub fn center(self) -> Vec2
	{
		(self.max + self.min) * 0.5
	}
	
	#[inline]
	pub fn center_mul(self, scale: Vec2) -> Self
	{
		let center = self.center();
		let centered = self - center;
		centered.component_mul(scale) + center
	}
	
	#[inline]
	pub fn component_mul(self, scale: Vec2) -> Self
	{
		Self { min: self.min.component_mul(scale), max: self.max.component_mul(scale) }
	}
	
	#[inline]
	pub fn component_div(self, scale: Vec2) -> Self
	{
		Self { min: self.min.component_div(scale), max: self.max.component_div(scale) }
	}

    #[inline]
    pub fn contains_l2(self, pos: Vec2) -> bool
    {
        let norm = (pos - self.min).component_div(self.size()) - Vec2(0.5, 0.5);
        norm.norm_sqr() <= 0.25
    }

    #[inline]
    pub fn contains_linf(self, pos: Vec2) -> bool
    {
        self.min.0 <= pos.0 && pos.0 <= self.max.0
     && self.min.1 <= pos.1 && pos.1 <= self.max.1
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

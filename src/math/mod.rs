use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt::{Display, Formatter, Result};

const DIV_CUT: f32 = 0.001;

macro_rules! impl_add_assign
{
    ($ty:ident, $ty2:ident) =>
    {
		impl AddAssign<$ty2> for $ty
		{
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

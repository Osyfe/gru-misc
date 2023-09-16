#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use super::*;

macro_rules! impl_ops
{
	($ty:ident, $($i:tt),+) =>
	{
		impl Neg for $ty
		{
		    type Output = Self;
			#[inline]
		    fn neg(self) -> Self
		    {
		        Self($(-self.$i),+)
		    }
		}

		impl Add<Self> for $ty
		{
			type Output = Self;
			#[inline]
			fn add(self, other: Self) -> Self
			{
				Self($(self.$i + other.$i),+)
			}
		}

		impl Sub<Self> for $ty
		{
			type Output = Self;
			#[inline]
			fn sub(self, other: Self) -> Self
			{
				Self($(self.$i - other.$i),+)
			}
		}

		impl Mul<f32> for $ty
		{
			type Output = Self;
			#[inline]
			fn mul(self, other: f32) -> Self
			{
				Self($(self.$i * other),+)
			}
		}

		impl Div<f32> for $ty
		{
			type Output = Self;
			#[inline]
			fn div(self, other: f32) -> Self
			{
				Self($(self.$i / other),+)
			}
		}
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vec2(pub f32, pub f32);

impl_ops!(Vec2, 0, 1);
impl_add_assign!(Vec2, Self);
impl_sub_assign!(Vec2, Self);
impl_mul_assign!(Vec2, f32);
impl_div_assign!(Vec2, f32);

impl Display for Vec2
{
    fn fmt(&self, f: &mut Formatter) -> Result
    {
        write!(f, "({}\t{})", self.0, self.1)
    }
}

impl<T: Into<f32>> From<(T, T)> for Vec2
{
	#[inline]
	fn from(t: (T, T)) -> Vec2
	{
		Vec2(t.0.into(), t.1.into())
	}
}

impl<T: From<f32>> From<Vec2> for (T, T)
{
	#[inline]
	fn from(v: Vec2) -> (T, T)
	{
		(v.0.into(), v.1.into())
	}
}

impl Vec2
{
	#[inline]
	pub const fn zero() -> Self
	{
		Self(0.0, 0.0)
	}

	#[inline]
	pub const fn e_x() -> Self
	{
		Self(1.0, 0.0)
	}

	#[inline]
	pub const fn e_y() -> Self
	{
		Self(0.0, 1.0)
	}

	#[inline]
	pub fn norm_sqr(self) -> f32
	{
		self.0 * self.0 + self.1 * self.1
	}

	#[inline]
	pub fn norm(self) -> f32
	{
		(self.0 * self.0 + self.1 * self.1).sqrt()
	}

	#[inline]
	pub fn unit(self) -> Self
	{
		let norm = 1.0 / self.norm();
		Self(self.0 * norm, self.1 * norm)
	}

	#[inline]
	pub fn dot(self, other: Self) -> f32
	{
		self.0 * other.0 + self.1 * other.1
	}

	#[inline]
	pub fn component_mul(self, other: Self) -> Self
	{
		Self(self.0 * other.0, self.1 * other.1)
	}

	#[inline]
	pub fn component_div(self, other: Self) -> Self
	{
		Self(self.0 / other.0, self.1 / other.1)
	}

	#[inline]
	pub fn component_inverse(self) -> Self
	{
		Self(1.0 / self.0, 1.0 / self.1)
	}

	#[inline]
	pub fn component_abs(self) -> Self
	{
		Self(self.0.abs(), self.1.abs())
	}
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl_ops!(Vec3, 0, 1, 2);
impl_add_assign!(Vec3, Self);
impl_sub_assign!(Vec3, Self);
impl_mul_assign!(Vec3, f32);
impl_div_assign!(Vec3, f32);

impl Display for Vec3
{
    fn fmt(&self, f: &mut Formatter) -> Result
    {
        write!(f, "({}\t{}\t{})", self.0, self.1, self.2)
    }
}

impl<T: Into<f32>> From<(T, T, T)> for Vec3
{
	#[inline]
	fn from(t: (T, T, T)) -> Vec3
	{
		Vec3(t.0.into(), t.1.into(), t.2.into())
	}
}

impl<T: From<f32>> From<Vec3> for (T, T, T)
{
	#[inline]
	fn from(v: Vec3) -> (T, T, T)
	{
		(v.0.into(), v.1.into(), v.2.into())
	}
}

impl Vec3
{
	#[inline]
	pub const fn zero() -> Self
	{
		Self(0.0, 0.0, 0.0)
	}

	#[inline]
	pub const fn e_x() -> Self
	{
		Self(1.0, 0.0, 0.0)
	}

	#[inline]
	pub const fn e_y() -> Self
	{
		Self(0.0, 1.0, 0.0)
	}

	#[inline]
	pub const fn e_z() -> Self
	{
		Self(0.0, 0.0, 1.0)
	}

	#[inline]
	pub fn from_sphere(theta: f32, phi: f32) -> Self
	{
		let sin_theta = theta.sin();
		Self(sin_theta * phi.cos(), sin_theta * phi.sin(), theta.cos())
	}

	#[inline]
	pub fn norm_sqr(self) -> f32
	{
		self.0 * self.0 + self.1 * self.1 + self.2 * self.2
	}

	#[inline]
	pub fn norm(self) -> f32
	{
		(self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
	}

	#[inline]
	pub fn unit(self) -> Self
	{
		let norm = 1.0 / self.norm();
		Self(self.0 * norm, self.1 * norm, self.2 * norm)
	}

	#[inline]
	pub fn dot(self, other: Self) -> f32
	{
		self.0 * other.0 + self.1 * other.1 + self.2 * other.2
	}

	#[inline]
	pub fn cross(self, other: Self) -> Self
	{
		Self(self.1 * other.2 - self.2 * other.1, self.2 * other.0 - self.0 * other.2, self.0 * other.1 - self.1 * other.0)
	}

	#[inline]
	pub fn component_mul(self, other: Self) -> Self
	{
		Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
	}

	#[inline]
	pub fn component_div(self, other: Self) -> Self
	{
		Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
	}

	#[inline]
	pub fn component_inverse(self) -> Self
	{
		Self(1.0 / self.0, 1.0 / self.1, 1.0 / self.2)
	}

	#[inline]
	pub fn component_abs(self) -> Self
	{
		Self(self.0.abs(), self.1.abs() , self.2.abs())
	}

	#[inline]
	pub const fn with_w0(self) -> Vec4
	{
		Vec4(self.0, self.1, self.2, 0.0)
	}

	#[inline]
	pub const fn with_w1(self) -> Vec4
	{
		Vec4(self.0, self.1, self.2, 1.0)
	}
}


#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

impl_ops!(Vec4, 0, 1, 2, 3);
impl_add_assign!(Vec4, Self);
impl_sub_assign!(Vec4, Self);
impl_mul_assign!(Vec4, f32);
impl_div_assign!(Vec4, f32);

impl Display for Vec4
{
    fn fmt(&self, f: &mut Formatter) -> Result
    {
        write!(f, "({}\t{}\t{}\t{})", self.0, self.1, self.2, self.3)
    }
}

impl<T: Into<f32>> From<(T, T, T, T)> for Vec4
{
	#[inline]
	fn from(t: (T, T, T, T)) -> Vec4
	{
		Vec4(t.0.into(), t.1.into(), t.2.into(), t.3.into())
	}
}

impl<T: From<f32>> From<Vec4> for (T, T, T, T)
{
	#[inline]
	fn from(v: Vec4) -> (T, T, T, T)
	{
		(v.0.into(), v.1.into(), v.2.into(), v.3.into())
	}
}

impl Vec4
{
	#[inline]
	pub const fn zero() -> Self
	{
		Self(0.0, 0.0, 0.0, 0.0)
	}

	#[inline]
	pub fn norm_sqr(self) -> f32
	{
		self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3
	}

	#[inline]
	pub fn norm(self) -> f32
	{
		(self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3).sqrt()
	}

	#[inline]
	pub fn unit(self) -> Self
	{
		let norm = 1.0 / self.norm();
		Self(self.0 * norm, self.1 * norm, self.2 * norm, self.3 * norm)
	}

	#[inline]
	pub fn dot(self, other: Self) -> f32
	{
		self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3
	}

	#[inline]
	pub fn component_mul(self, other: Self) -> Self
	{
		Self(self.0 * other.0, self.1 * other.1, self.2 * other.2, self.3 * other.3)
	}

	#[inline]
	pub fn component_div(self, other: Self) -> Self
	{
		Self(self.0 / other.0, self.1 / other.1, self.2 / other.2, self.3 / other.3)
	}

	#[inline]
	pub fn component_inverse(self) -> Self
	{
		Self(1.0 / self.0, 1.0 / self.1, 1.0 / self.2, 1.0 / self.3)
	}

	#[inline]
	pub const fn without_w(self) -> Vec3
	{
		Vec3(self.0, self.1, self.2)
	}
    
    #[inline]
    pub fn divide_w(self) -> Vec3
    {
        Vec3(self.0 / self.3, self.1 / self.3, self.2 / self.3)
    }
}

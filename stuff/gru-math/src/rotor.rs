#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use super::*;

//     #####     COMPLEX     #####

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Complex
{
	pub re: f32,
	pub im: f32
}

impl Display for Complex
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        write!(f, "({} + {}i)", self.re, self.im)
    }
}

impl Neg for Complex
{
    type Output = Self;
    fn neg(self) -> Self
    {
        Self { re: -self.re, im: -self.im }
    }
}

impl Add<Self> for Complex
{
	type Output = Self;
	fn add(self, other: Self) -> Self
	{
		Self { re: self.re + other.re, im: self.im + other.im }
	}
}

impl Sub<Self> for Complex
{
	type Output = Self;
	fn sub(self, other: Self) -> Self
	{
		Self { re: self.re - other.re, im: self.im - other.im }
	}
}

impl Mul<Self> for Complex
{
	type Output = Self;
	fn mul(self, other: Self) -> Self
	{
		Self { re: self.re * other.re - self.im * other.im, im: self.re * other.im + self.im * other.re }
	}
}

impl Mul<f32> for Complex
{
	type Output = Self;
	fn mul(self, other: f32) -> Self
	{
		Self { re: self.re * other, im: self.im * other }
	}
}

impl Div<Self> for Complex
{
	type Output = Self;
	fn div(self, other: Self) -> Self
	{
		self * other.conjugate() / other.r2()
	}
}

impl Div<f32> for Complex
{
	type Output = Self;
	fn div(self, other: f32) -> Self
	{
		Self { re: self.re / other, im: self.im / other }
	}
}

impl_add_assign!(Complex, Self);
impl_sub_assign!(Complex, Self);
impl_mul_assign!(Complex, Self);
impl_mul_assign!(Complex, f32);
impl_div_assign!(Complex, Self);
impl_div_assign!(Complex, f32);

impl Complex
{
	pub const ZERO: Self = Self { re: 0.0, im: 0.0 };
	pub const ONE: Self = Self { re: 1.0, im: 0.0 };
	pub const I: Self = Self { re: 0.0, im: 1.0 };

	pub fn from_re(re: f32) -> Self
	{
		Self { re, im: 0.0 }
	}

	pub fn from_im(im: f32) -> Self
	{
		Self { re: 0.0, im }
	}

	pub fn conjugate(self) -> Self
	{
		Self { re: self.re, im: -self.im }
	}

	pub fn r(self) -> f32
	{
		self.r2().sqrt()
	}

	pub fn phi(self) -> f32
	{
		self.im.atan2(self.re)
	}

	pub fn r2(self) -> f32
	{
		self.re * self.re + self.im * self.im
	}

	pub fn exp(self) -> Self
	{
		Self { re: self.im.cos(), im: self.im.sin() } * self.re.exp()
	}

	pub fn transform(self, vec: Vec2) -> Vec2
	{
		Vec2(self.re * vec.0 - self.im * vec.1, self.im * vec.0 + self.re * vec.1)
	}

	pub fn to_mat2(self) -> Mat2
	{
		Mat2((self.re, self.im), (-self.im, self.re))
	}
}

//     #####     ROTOR     #####

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rotor
{
	s: f32, //scalar
	yz: f32,
	zx: f32,
	xy: f32
}

impl Neg for Rotor
{
    type Output = Self;
    fn neg(self) -> Self
    {
        Self { s: -self.s, yz: -self.yz, zx: -self.zx, xy: -self.xy }
    }
}

impl Mul<Self> for Rotor
{
	type Output = Self;
	fn mul(self, other: Self) -> Self
	{
		Self
		{
			s: self.s * other.s - self.zx * other.zx - self.xy * other.xy - self.yz * other.yz,
			yz: self.s * other.yz + self.yz * other.s - self.zx * other.xy + self.xy * other.zx,
			zx: self.s * other.zx + self.zx * other.s - self.xy * other.yz + self.yz * other.xy,
			xy: self.s * other.xy + self.xy * other.s - self.yz * other.zx + self.zx * other.yz
		}
	}
}

impl_mul_assign!(Rotor, Self);

impl Rotor
{
	pub fn from_unit_axis(axis: Vec3, phi: f32) -> Self
	{
		let phi_mod = -phi / 2.0;
		let cos = phi_mod.cos();
		let sin = phi_mod.sin();
		Self { s: cos, yz: sin * axis.0, zx: sin * axis.1, xy: sin * axis.2 }
	}

	pub fn from_axis(axis: Vec3) -> Self
	{
		let norm = axis.norm();
		let phi_mod = -norm / 2.0;
		let cos = phi_mod.cos();
		let sin_red = if norm > DIV_CUT { phi_mod.sin() / norm } else { -0.5 + norm * norm / 48.0 };
		Self { s: cos, yz: sin_red * axis.0, zx: sin_red * axis.1, xy: sin_red * axis.2 }
	}

	pub fn from_quaternion(x: f32, y: f32, z: f32, w: f32) -> Self
	{
		Self { s: w, yz: -x, zx: -y, xy: -z }
	}

	pub fn identity() -> Self
	{
		Self { s: 1.0, yz: 0.0, zx: 0.0, xy: 0.0 }
	}

	pub fn inverse(self) -> Self
	{
		Self { s: self.s, yz: -self.yz, zx: -self.zx, xy: -self.xy }
	}
	pub fn transform(self, vec: Vec3) -> Vec3
	{
		//This short version is slightly slower in release mode.
		//let erg = self * Rotor { s: 0.0, yz: vec.0, zx: vec.1, xy: vec.2 } * self.inverse();
		//Vec3(erg.yz, erg.zx, erg.xy)
		let x = self.s * vec.0 + self.xy * vec.1 - self.zx * vec.2;
		let y = self.s * vec.1 + self.yz * vec.2 - self.xy * vec.0;
		let z = self.s * vec.2 + self.zx * vec.0 - self.yz * vec.1;
		let i = self.xy * vec.2 + self.yz * vec.0 + self.zx * vec.1;
		Vec3
		(
			x * self.s + y * self.xy - z * self.zx + i * self.yz,
			y * self.s + z * self.yz - x * self.xy + i * self.zx,
			z * self.s + x * self.zx - y * self.yz + i * self.xy
		)
	}

	pub fn to_axis(self) -> Vec3
	{
		let phi_mod = self.s.max(-1.0).min(1.0).acos();
		let norm = -2.0 * phi_mod;
		let sin_red_inv = if phi_mod > DIV_CUT { norm / phi_mod.sin() } else { -2.0 - phi_mod * phi_mod / 3.0 };
		Vec3(self.yz, self.zx, self.xy) * sin_red_inv
	}

	pub fn to_mat3(self) -> Mat3
	{
		let s2 = self.s * self.s;
		let yz2 = self.yz * self.yz;
		let zx2 = self.zx * self.zx;
		let xy2 = self.xy * self.xy;
		let syz = 2.0 * self.s * self.yz;
		let szx = 2.0 * self.s * self.zx;
		let sxy = 2.0 * self.s * self.xy;
		let yzzx = 2.0 * self.yz * self.zx;
		let zxxy = 2.0 * self.zx * self.xy;
		let xyyz = 2.0 * self.xy * self.yz;
		Mat3
		(
			(s2 + yz2 - zx2 - xy2, -sxy + yzzx, szx + xyyz),
			(sxy + yzzx, s2 + zx2 - xy2 - yz2, -syz + zxxy),
			(-szx + xyyz, syz + zxxy, s2 + xy2 - yz2 - zx2)
		)
	}

	pub fn to_mat4(self) -> Mat4
	{
		self.to_mat3().to_mat4() //gets optimised in release mode
	}

	pub fn fix(self) -> Self
	{
		let norm = (self.s * self.s + self.yz * self.yz + self.zx * self.zx + self.xy * self.xy).sqrt();
		Self { s: self.s / norm, yz: self.yz / norm, zx: self.zx / norm, xy: self.xy / norm }
	}
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum SlerpFunction
{
	Exact { omega: f32, sin_omega: f32 },
	Taylor { omega2: f32 }
}

impl SlerpFunction
{
	fn new(omega: f32) -> Self
	{
		if omega > DIV_CUT { Self::Exact { omega, sin_omega: omega.sin() } }
		else { Self::Taylor { omega2: omega * omega } }
	}

	fn get(self, t: f32) -> f32
	{
		match self
		{
			Self::Exact { omega, sin_omega } => (t * omega).sin() / sin_omega,
			Self::Taylor { omega2 } => t + (t - t.powi(3)) / 6.0 * omega2
		}
	}
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Slerp
{
	function: SlerpFunction,
	r1: Rotor,
	r2: Rotor
}

impl Slerp
{
	pub fn new(r1: Rotor, mut r2: Rotor) -> Self
	{
		let mut cos_omega = r1.s * r2.s + r1.yz * r2.yz + r1.zx * r2.zx + r1.xy * r2.xy;
		if cos_omega < 0.0
		{
			r2 = -r2;
			cos_omega = -cos_omega;
		}
		let omega = cos_omega.max(-1.0).min(1.0).acos();
		let function = SlerpFunction::new(omega);
		Self { function, r1, r2 }
	}

	pub fn get(self, t: f32) -> Rotor
	{
		let f1 = self.function.get(1.0 - t);
		let f2 = self.function.get(t);
		Rotor
		{
			s: f1 * self.r1.s + f2 * self.r2.s,
			yz: f1 * self.r1.yz + f2 * self.r2.yz,
			zx: f1 * self.r1.zx + f2 * self.r2.zx,
			xy: f1 * self.r1.xy + f2 * self.r2.xy
		}
	}
}

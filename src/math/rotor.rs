#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};

use super::*;

//     #####     COMPLEX     #####

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct Complex
{
	pub re: f32,
	pub im: f32
}

impl Display for Complex
{
    fn fmt(&self, f: &mut Formatter) -> Result
    {
        write!(f, "({} + {}i)", self.re, self.im)
    }
}

impl Neg for Complex
{
    type Output = Self;
	#[inline]
    fn neg(self) -> Self
    {
        Self { re: -self.re, im: -self.im }
    }
}

impl Add<Self> for Complex
{
	type Output = Self;
	#[inline]
	fn add(self, other: Self) -> Self
	{
		Self { re: self.re + other.re, im: self.im + other.im }
	}
}

impl Sub<Self> for Complex
{
	type Output = Self;
	#[inline]
	fn sub(self, other: Self) -> Self
	{
		Self { re: self.re - other.re, im: self.im - other.im }
	}
}

impl Mul<Self> for Complex
{
	type Output = Self;
	#[inline]
	fn mul(self, other: Self) -> Self
	{
		Self { re: self.re * other.re - self.im * other.im, im: self.re * other.im + self.im * other.re }
	}
}

impl Mul<f32> for Complex
{
	type Output = Self;
	#[inline]
	fn mul(self, other: f32) -> Self
	{
		Self { re: self.re * other, im: self.im * other }
	}
}

impl Div<Self> for Complex
{
	type Output = Self;
	#[inline]
	fn div(self, other: Self) -> Self
	{
		self * other.conjugate() / other.r2()
	}
}

impl Div<f32> for Complex
{
	type Output = Self;
	#[inline]
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

	#[inline]
	pub const fn from_re(re: f32) -> Self
	{
		Self { re, im: 0.0 }
	}

	#[inline]
	pub const fn from_im(im: f32) -> Self
	{
		Self { re: 0.0, im }
	}

	#[inline]
	pub const fn conjugate(self) -> Self
	{
		Self { re: self.re, im: -self.im }
	}

	#[inline]
	pub fn r(self) -> f32
	{
		self.r2().sqrt()
	}

    #[inline]
	pub const fn r2(self) -> f32
	{
		self.re * self.re + self.im * self.im
	}

	#[inline]
	pub fn phi(self) -> f32
	{
		self.im.atan2(self.re)
	}

	#[inline]
	pub fn exp(self) -> Self
	{
        let (sin, cos) = self.im.sin_cos();
		Self { re: cos, im: sin } * self.re.exp()
	}

	#[inline]
	pub const fn transform(self, vec: Vec2) -> Vec2
	{
		Vec2(self.re * vec.0 - self.im * vec.1, self.im * vec.0 + self.re * vec.1)
	}

	#[inline]
	pub const fn to_mat2(self) -> Mat2
	{
		Mat2(Vec2(self.re, self.im), Vec2(-self.im, self.re))
	}
}

//     #####     ROTOR     #####

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct Rotor
{
	s: f32, //scalar
	yz: f32,
	zx: f32,
	xy: f32
}

impl Default for Rotor
{
	fn default() -> Self
	{
		Self::identity()
	}
}

impl Neg for Rotor
{
    type Output = Self;
	#[inline]
    fn neg(self) -> Self
    {
        Self { s: -self.s, yz: -self.yz, zx: -self.zx, xy: -self.xy }
    }
}

impl Mul<Self> for Rotor
{
	type Output = Self;
	#[inline]
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
	#[inline]
	pub fn from_unit_axis(axis: Vec3, phi: f32) -> Self
	{
		let phi_mod = -phi / 2.0;
		let cos = phi_mod.cos();
		let sin = phi_mod.sin();
		Self { s: cos, yz: sin * axis.0, zx: sin * axis.1, xy: sin * axis.2 }
	}

	#[inline]
	pub fn from_axis(axis: Vec3) -> Self
	{
		let norm = axis.norm();
		let phi_mod = -norm / 2.0;
		let cos = phi_mod.cos();
		let sin_red = if norm > DIV_CUT { phi_mod.sin() / norm } else { -0.5 + norm * norm / 48.0 };
		Self { s: cos, yz: sin_red * axis.0, zx: sin_red * axis.1, xy: sin_red * axis.2 }
	}
	
	#[inline]
	pub fn from_plane(v1: Vec3, v2: Vec3) -> Self
	{
		let normal = v1.cross(v2);
		let dot = v1.dot(v2);
		let phi = dot.max(-1.0).min(1.0).acos();
		let phi_mod = -phi / 2.0;
		let cos = phi_mod.cos();
		let sin_red = -(1.0 / (2.0 * (1.0 + dot))).sqrt(); //diverges for v1 ~ -v2 => NaN
		Self { s: cos, yz: sin_red * normal.0, zx: sin_red * normal.1, xy: sin_red * normal.2 }
	}

	#[inline]
	pub fn from_coords((x1, y1): (Vec3, Vec3), (x2, y2): (Vec3, Vec3)) -> Self
	{
		let (dot_x, dot_y) = (x2.dot(x1), y2.dot(y1));
		if dot_x <= 0.0 && dot_y <= 0.0
		{
			let delta_x = (x2 - x1).unit();
			let axis = delta_x.cross(y2 - y1).unit();
			let normal = delta_x.cross(axis);
			let phi = -2.0 * f32::atan2(x1.dot(delta_x), x1.dot(normal));
			Self::from_unit_axis(axis, phi)
		} else if dot_x >= 0.0
		{
			let rot1 = Self::from_plane(x1, x2);
			let y1 = rot1.transform(y1);
			let z2 = x2.cross(y2);
			let phi = -f32::atan2(y1.dot(z2), y1.dot(y2));
			let rot2 = Self::from_unit_axis(x2, phi);
			rot2 * rot1
		} else //dot_y > 0
		{
			let rot1 = Self::from_plane(y1, y2);
			let x1 = rot1.transform(x1);
			let z2 = x2.cross(y2);
			let phi = -f32::atan2(-x1.dot(z2), x1.dot(x2));
			let rot2 = Self::from_unit_axis(y2, phi);
			rot2 * rot1
		}
	}

	#[inline]
	pub fn from_euler(psi: f32, theta: f32, phi: f32) -> Self
	{
		Self::from_unit_axis(Vec3::e_z(), phi) * Self::from_unit_axis(Vec3::e_y(), theta) * Self::from_unit_axis(Vec3::e_z(), psi)
	}

	#[inline]
	pub const fn from_quaternion(x: f32, y: f32, z: f32, w: f32) -> Self
	{
		Self { s: w, yz: -x, zx: -y, xy: -z }
	}

	#[inline]
	pub const fn identity() -> Self
	{
		Self { s: 1.0, yz: 0.0, zx: 0.0, xy: 0.0 }
	}

	#[inline]
	pub fn inverse(self) -> Self
	{
		Self { s: self.s, yz: -self.yz, zx: -self.zx, xy: -self.xy }
	}

	#[inline]
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

	#[inline]
	pub fn to_axis(self) -> Vec3
	{
		let phi_mod = self.s.max(-1.0).min(1.0).acos();
		let sin_red_inv = if phi_mod > DIV_CUT { -2.0 * phi_mod / phi_mod.sin() } else { -2.0 - phi_mod * phi_mod / 3.0 };
		Vec3(self.yz, self.zx, self.xy) * sin_red_inv
	}

	#[inline]
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
			Vec3(s2 + yz2 - zx2 - xy2, -sxy + yzzx, szx + xyyz),
			Vec3(sxy + yzzx, s2 + zx2 - xy2 - yz2, -syz + zxxy),
			Vec3(-szx + xyyz, syz + zxxy, s2 + xy2 - yz2 - zx2)
		)
	}

	#[inline]
	pub fn to_mat4(self) -> Mat4
	{
		self.to_mat3().to_mat4() //gets optimised in release mode
	}
    
    #[inline]
    pub fn to_components(self) -> (f32, [f32; 3])
    {
        (self.s, [self.yz, self.zx, self.xy])
    }

	#[inline]
	pub fn fix(self) -> Self
	{
		let norm = 1.0 / (self.s * self.s + self.yz * self.yz + self.zx * self.zx + self.xy * self.xy).sqrt();
		Self { s: self.s * norm, yz: self.yz * norm, zx: self.zx * norm, xy: self.xy * norm }
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
	#[inline]
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

	#[inline]
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

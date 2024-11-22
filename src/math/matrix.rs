#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use super::*;

//All matrices are column major (=> ready for shaders)!

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Mat2(pub Vec2, pub Vec2);
static_assertions::const_assert_eq!(std::mem::size_of::<Mat2>(), 16);

impl Display for Mat2
{
    fn fmt(&self, f: &mut Formatter) -> Result
    {
        write!(f, "({}\t{}\n {}\t{})", self.0.0, self.1.0, self.0.1, self.1.1)
    }
}

impl Mul<Self> for Mat2
{
	type Output = Self;
	#[inline]
	fn mul(self, other: Self) -> Self
	{
		Self
		(
			Vec2
			(
				self.0.0 * other.0.0 + self.1.0 * other.0.1,
				self.0.1 * other.0.0 + self.1.1 * other.0.1
			), Vec2
			(
				self.0.0 * other.1.0 + self.1.0 * other.1.1,
				self.0.1 * other.1.0 + self.1.1 * other.1.1
			)
		)
	}
}

impl Mul<Vec2> for Mat2
{
	type Output = Vec2;
	#[inline]
	fn mul(self, other: Vec2) -> Vec2
	{
		Vec2(self.0.0 * other.0 + self.1.0 * other.1, self.0.1 * other.0 + self.1.1 * other.1)
	}
}

impl_mul_assign!(Mat2, Self);

impl Mat2
{
	#[inline]
	pub const fn identity() -> Self
	{
		Self(Vec2(1.0, 0.0), Vec2(0.0, 1.0))
	}

	#[inline]
	pub fn rotation(phi: f32) -> Self
	{
		let cos = phi.cos();
		let sin = phi.sin();
		Self(Vec2(cos, sin), Vec2(-sin, cos))
	}

	#[inline]
	pub const fn scale(Vec2(sx, sy): Vec2) -> Self
	{
		Self(Vec2(sx, 0.0), Vec2(0.0, sy))
	}

	#[inline]
	pub const fn scale_xy(s: f32) -> Self { Self::scale(Vec2(s, s)) }
	#[inline]
	pub const fn scale_x(s: f32) -> Self { Self::scale(Vec2(s, 1.0)) }
	#[inline]
	pub const fn scale_y(s: f32) -> Self { Self::scale(Vec2(1.0, s)) }

	#[inline]
	pub const fn transpose(self) -> Self
	{
		Self(Vec2(self.0.0, self.1.0), Vec2(self.0.1, self.1.1))
	}

    #[inline]
    pub fn det(self) -> f32
    {
        self.0.0 * self.1.1 - self.0.1 * self.1.0
    }

	#[inline]
	pub const fn to_array(&self) -> [f32; 4]
	{
		[self.0.0, self.0.1, self.1.0, self.1.1]
	}
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Mat3(pub Vec3, pub Vec3, pub Vec3);
static_assertions::const_assert_eq!(std::mem::size_of::<Mat3>(), 36);

impl Display for Mat3
{
    fn fmt(&self, f: &mut Formatter) -> Result
    {
        write!
        (
        	f,
        	"({}\t{}\t{}\n {}\t{}\t{}\n {}\t{}\t{})",
        	self.0.0, self.1.0, self.2.0,
        	self.0.1, self.1.1, self.2.1,
        	self.0.2, self.1.2, self.2.2
        )
    }
}

impl Mul<Self> for Mat3
{
	type Output = Self;
	#[inline]
	fn mul(self, other: Self) -> Self
	{
		Self
		(
			Vec3
			(
				self.0.0 * other.0.0 + self.1.0 * other.0.1 + self.2.0 * other.0.2,
				self.0.1 * other.0.0 + self.1.1 * other.0.1 + self.2.1 * other.0.2,
				self.0.2 * other.0.0 + self.1.2 * other.0.1 + self.2.2 * other.0.2
			), Vec3
			(
				self.0.0 * other.1.0 + self.1.0 * other.1.1 + self.2.0 * other.1.2,
				self.0.1 * other.1.0 + self.1.1 * other.1.1 + self.2.1 * other.1.2,
				self.0.2 * other.1.0 + self.1.2 * other.1.1 + self.2.2 * other.1.2
			), Vec3
			(
				self.0.0 * other.2.0 + self.1.0 * other.2.1 + self.2.0 * other.2.2,
				self.0.1 * other.2.0 + self.1.1 * other.2.1 + self.2.1 * other.2.2,
				self.0.2 * other.2.0 + self.1.2 * other.2.1 + self.2.2 * other.2.2
			)
		)
	}
}

impl Mul<Vec3> for Mat3
{
	type Output = Vec3;
	#[inline]
	fn mul(self, other: Vec3) -> Vec3
	{
		Vec3
		(
			self.0.0 * other.0 + self.1.0 * other.1 + self.2.0 * other.2,
			self.0.1 * other.0 + self.1.1 * other.1 + self.2.1 * other.2,
			self.0.2 * other.0 + self.1.2 * other.1 + self.2.2 * other.2
		)
	}
}

impl_mul_assign!(Mat3, Self);

impl Mat3
{
	#[inline]
	pub const fn identity() -> Self
	{
		Self
		(
			Vec3(1.0, 0.0, 0.0),
			Vec3(0.0, 1.0, 0.0),
			Vec3(0.0, 0.0, 1.0)
		)
	}

	#[inline]
	pub fn rotation(Vec3(ax, ay, az): Vec3, phi: f32) -> Self
	{
		let cos = phi.cos();
		let one_cos = 1.0 - cos;
		let sin = phi.sin();
		Self
		(
			Vec3(ax*ax*one_cos + cos, ay*ax*one_cos + az*sin, az*ax*one_cos - ay*sin),
			Vec3(ax*ay*one_cos - az*sin, ay*ay*one_cos + cos, az*ay*one_cos + ax*sin),
			Vec3(ax*az*one_cos + ay*sin, ay*az*one_cos - ax*sin, az*az*one_cos + cos)
		)
	}

	#[inline]
	pub fn rotation_x(phi: f32) -> Self { Self::rotation(Vec3(1.0, 0.0, 0.0), phi) }
	#[inline]
	pub fn rotation_y(phi: f32) -> Self { Self::rotation(Vec3(0.0, 1.0, 0.0), phi) }
	#[inline]
	pub fn rotation_z(phi: f32) -> Self { Self::rotation(Vec3(0.0, 0.0, 1.0), phi) }
    
    #[inline]
    pub fn rotation_euler_xyz(psi: f32, theta: f32, phi: f32) -> Self
    {
        Self::rotation_z(phi) * Self::rotation_x(theta) * Self::rotation_z(psi)
    }

	#[inline]
	pub const fn scale(Vec3(sx, sy, sz): Vec3) -> Self
	{
		Self
		(
			Vec3(sx, 0.0, 0.0),
			Vec3(0.0, sy, 0.0),
			Vec3(0.0, 0.0, sz)
		)
	}

	#[inline]
	pub const fn scale_xyz(s: f32) -> Self { Self::scale(Vec3(s, s, s)) }
	#[inline]
	pub const fn scale_x(s: f32) -> Self { Self::scale(Vec3(s, 1.0, 1.0)) }
	#[inline]
	pub const fn scale_y(s: f32) -> Self { Self::scale(Vec3(1.0, s, 1.0)) }
	#[inline]
	pub const fn scale_z(s: f32) -> Self { Self::scale(Vec3(1.0, 1.0, s)) }

	#[inline]
	pub const fn transpose(self) -> Self
	{
		Self
		(
			Vec3(self.0.0, self.1.0, self.2.0),
        	Vec3(self.0.1, self.1.1, self.2.1),
        	Vec3(self.0.2, self.1.2, self.2.2)
        )
	}
    
    #[inline]
    pub fn inverse(self) -> Self
    {
        let norm = 1.0 / self.det();
        Self
        (
            Vec3(self.1.1 * self.2.2 - self.2.1 * self.1.2, self.2.1 * self.0.2 - self.0.1 * self.2.2, self.0.1 * self.1.2 - self.1.1 * self.0.2) * norm,
            Vec3(self.2.0 * self.1.2 - self.1.0 * self.2.2, self.0.0 * self.2.2 - self.2.0 * self.0.2, self.1.0 * self.0.2 - self.0.0 * self.1.2) * norm,
            Vec3(self.1.0 * self.2.1 - self.2.0 * self.1.1, self.2.0 * self.0.1 - self.0.0 * self.2.1, self.0.0 * self.1.1 - self.1.0 * self.0.1) * norm
        )
    }

    #[inline]
    pub fn det(self) -> f32
    {
        self.0.0 * self.1.1 * self.2.2
      + self.1.0 * self.2.1 * self.0.2
      + self.2.0 * self.0.1 * self.1.2
      - self.0.2 * self.1.1 * self.2.0
      - self.0.1 * self.1.0 * self.2.2
      - self.0.0 * self.1.2 * self.2.1
    }

	#[inline]
	pub const fn to_mat4(self) -> Mat4
	{
		Mat4
		(
			Vec4(self.0.0, self.0.1, self.0.2, 0.0),
        	Vec4(self.1.0, self.1.1, self.1.2, 0.0),
        	Vec4(self.2.0, self.2.1, self.2.2, 0.0),
        	Vec4(0.0, 0.0, 0.0, 1.0)
		)
	}

	#[inline]
	pub const fn to_array(&self) -> [f32; 9]
	{
		[self.0.0, self.0.1, self.0.2, self.1.0, self.1.1, self.1.2, self.2.0, self.2.1, self.2.2]
	}
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Mat4(pub Vec4, pub Vec4, pub Vec4, pub Vec4);
static_assertions::const_assert_eq!(std::mem::size_of::<Mat4>(), 64);

impl Display for Mat4
{
    fn fmt(&self, f: &mut Formatter) -> Result
    {
        write!
        (
        	f,
        	"({}\t{}\t{}\t{}\n {}\t{}\t{}\t{}\n {}\t{}\t{}\t{}\n {}\t{}\t{}\t{})",
        	self.0.0, self.1.0, self.2.0, self.3.0,
        	self.0.1, self.1.1, self.2.1, self.3.1,
        	self.0.2, self.1.2, self.2.2, self.3.2,
        	self.0.3, self.1.3, self.2.3, self.3.3
        )
    }
}

impl Mul<Self> for Mat4
{
	type Output = Self;
	#[inline]
	fn mul(self, other: Self) -> Self
	{
		Self
		(
			Vec4
			(
				self.0.0 * other.0.0 + self.1.0 * other.0.1 + self.2.0 * other.0.2 + self.3.0 * other.0.3,
				self.0.1 * other.0.0 + self.1.1 * other.0.1 + self.2.1 * other.0.2 + self.3.1 * other.0.3,
				self.0.2 * other.0.0 + self.1.2 * other.0.1 + self.2.2 * other.0.2 + self.3.2 * other.0.3,
				self.0.3 * other.0.0 + self.1.3 * other.0.1 + self.2.3 * other.0.2 + self.3.3 * other.0.3
			), Vec4
			(
				self.0.0 * other.1.0 + self.1.0 * other.1.1 + self.2.0 * other.1.2 + self.3.0 * other.1.3,
				self.0.1 * other.1.0 + self.1.1 * other.1.1 + self.2.1 * other.1.2 + self.3.1 * other.1.3,
				self.0.2 * other.1.0 + self.1.2 * other.1.1 + self.2.2 * other.1.2 + self.3.2 * other.1.3,
				self.0.3 * other.1.0 + self.1.3 * other.1.1 + self.2.3 * other.1.2 + self.3.3 * other.1.3
			), Vec4
			(
				self.0.0 * other.2.0 + self.1.0 * other.2.1 + self.2.0 * other.2.2 + self.3.0 * other.2.3,
				self.0.1 * other.2.0 + self.1.1 * other.2.1 + self.2.1 * other.2.2 + self.3.1 * other.2.3,
				self.0.2 * other.2.0 + self.1.2 * other.2.1 + self.2.2 * other.2.2 + self.3.2 * other.2.3,
				self.0.3 * other.2.0 + self.1.3 * other.2.1 + self.2.3 * other.2.2 + self.3.3 * other.2.3
			), Vec4
			(
				self.0.0 * other.3.0 + self.1.0 * other.3.1 + self.2.0 * other.3.2 + self.3.0 * other.3.3,
				self.0.1 * other.3.0 + self.1.1 * other.3.1 + self.2.1 * other.3.2 + self.3.1 * other.3.3,
				self.0.2 * other.3.0 + self.1.2 * other.3.1 + self.2.2 * other.3.2 + self.3.2 * other.3.3,
				self.0.3 * other.3.0 + self.1.3 * other.3.1 + self.2.3 * other.3.2 + self.3.3 * other.3.3
			)
		)
	}
}

impl Mul<Vec4> for Mat4
{
	type Output = Vec4;
	#[inline]
	fn mul(self, other: Vec4) -> Vec4
	{
		Vec4
		(
			self.0.0 * other.0 + self.1.0 * other.1 + self.2.0 * other.2 + self.3.0 * other.3,
			self.0.1 * other.0 + self.1.1 * other.1 + self.2.1 * other.2 + self.3.1 * other.3,
			self.0.2 * other.0 + self.1.2 * other.1 + self.2.2 * other.2 + self.3.2 * other.3,
			self.0.3 * other.0 + self.1.3 * other.1 + self.2.3 * other.2 + self.3.3 * other.3
		)
	}
}

impl_mul_assign!(Mat4, Self);

impl Mat4
{
	#[inline]
	pub const fn identity() -> Self
	{
		Self
		(
			Vec4(1.0, 0.0, 0.0, 0.0),
			Vec4(0.0, 1.0, 0.0, 0.0),
			Vec4(0.0, 0.0, 1.0, 0.0),
			Vec4(0.0, 0.0, 0.0, 1.0)
		)
	}
	
    //z_lin = z_ndc * z_near / (z_ndc * (z_near - z_far) + z_far)
    //z_ndc = z_lin * z_far / (z_lin * (z_far - z_near) + z_near)
    
	#[inline]
	pub fn perspective_opengl(aspect: f32, fovy: f32, z_near: f32, z_far: f32) -> Self
	{
		let a = 1.0 / (fovy / 2.0).tan();
		let b = 1.0 / (z_far - z_near);
		Self
		(
			Vec4(a / aspect, 0.0, 0.0, 0.0),
			Vec4(0.0, a, 0.0, 0.0),
			Vec4(0.0, 0.0, -(z_near + z_far) * b, -1.0),
			Vec4(0.0, 0.0, -2.0 * z_near * z_far * b, 0.0)
		)
	}

	#[inline]
	pub fn perspective_opengl_inverse(aspect: f32, fovy: f32, z_near: f32, z_far: f32) -> Self
	{
		let a_inv = (fovy / 2.0).tan();
		let b_inv = z_far - z_near;
		let z_nf2 = 2.0 * z_near * z_far;
		Self
		(
			Vec4(a_inv * aspect, 0.0, 0.0, 0.0),
			Vec4(0.0, a_inv, 0.0, 0.0),
			Vec4(0.0, 0.0, 0.0, -b_inv / z_nf2),
			Vec4(0.0, 0.0, -1.0, (z_near + z_far) / z_nf2)
		)
	}

	#[inline]
	pub fn perspective_vulkan(aspect: f32, fovy: f32, z_near: f32, z_far: f32) -> Self
	{
		let a = 1.0 / (fovy / 2.0).tan();
		let b = z_far / (z_far - z_near);
		Self
		(
			Vec4(a / aspect, 0.0, 0.0, 0.0),
			Vec4(0.0, a, 0.0, 0.0),
			Vec4(0.0, 0.0, b, 1.0),
			Vec4(0.0, 0.0, -z_near * b, 0.0)
		)
	}
    
    #[inline]
	pub fn perspective_vulkan_inverse(aspect: f32, fovy: f32, z_near: f32, z_far: f32) -> Self
	{
		let a_inv = (fovy / 2.0).tan();
		let b_inv = (z_far - z_near) / z_far;
		Self
		(
			Vec4(a_inv * aspect, 0.0, 0.0, 0.0),
			Vec4(0.0, a_inv, 0.0, 0.0),
			Vec4(0.0, 0.0, 0.0, -b_inv / z_near),
			Vec4(0.0, 0.0, 1.0, 1.0 / z_near)
		)
	}

    #[inline]
    pub fn perspective_wgpu(aspect: f32, fovy: f32, z_near: f32, z_far: f32) -> Self
    {
        let a = 1.0 / (fovy / 2.0).tan();
		let b = z_far / (z_far - z_near);
		Self
		(
			Vec4(a / aspect, 0.0, 0.0, 0.0),
			Vec4(0.0, a, 0.0, 0.0),
			Vec4(0.0, 0.0, -b, -1.0),
			Vec4(0.0, 0.0, -z_near * b, 0.0)
		)
    }

    #[inline]
	pub fn perspective_wgpu_inverse(aspect: f32, fovy: f32, z_near: f32, z_far: f32) -> Self
	{
		let a_inv = (fovy / 2.0).tan();
		let b_inv = (z_far - z_near) / z_far;
		Self
		(
			Vec4(a_inv * aspect, 0.0, 0.0, 0.0),
			Vec4(0.0, a_inv, 0.0, 0.0),
			Vec4(0.0, 0.0, 0.0, -b_inv / z_near),
			Vec4(0.0, 0.0, -1.0, 1.0 / z_near)
		)
	}

	#[inline]
	pub const fn translation(Vec3(dx, dy, dz): Vec3) -> Self
	{
		Self
		(
			Vec4(1.0, 0.0, 0.0, 0.0),
			Vec4(0.0, 1.0, 0.0, 0.0),
			Vec4(0.0, 0.0, 1.0, 0.0),
			Vec4(dx, dy, dz, 1.0)
		)
	}

	#[inline]
	pub const fn translation_x(dx: f32) -> Self { Self::translation(Vec3(dx, 0.0, 0.0)) }
	#[inline]
	pub const fn translation_y(dy: f32) -> Self { Self::translation(Vec3(0.0, dy, 0.0)) }
	#[inline]
	pub const fn translation_z(dz: f32) -> Self { Self::translation(Vec3(0.0, 0.0, dz)) }

	#[inline]
	pub fn rotation(Vec3(ax, ay, az): Vec3, phi: f32) -> Self
	{
		let cos = phi.cos();
		let one_cos = 1.0 - cos;
		let sin = phi.sin();
		Self
		(
			Vec4(ax*ax*one_cos + cos, ay*ax*one_cos + az*sin, az*ax*one_cos - ay*sin, 0.0),
			Vec4(ax*ay*one_cos - az*sin, ay*ay*one_cos + cos, az*ay*one_cos + ax*sin, 0.0),
			Vec4(ax*az*one_cos + ay*sin, ay*az*one_cos - ax*sin, az*az*one_cos + cos, 0.0),
			Vec4(0.0, 0.0, 0.0, 1.0)
		)
	}

	#[inline]
	pub fn rotation_x(phi: f32) -> Self { Self::rotation(Vec3(1.0, 0.0, 0.0), phi) }
	#[inline]
	pub fn rotation_y(phi: f32) -> Self { Self::rotation(Vec3(0.0, 1.0, 0.0), phi) }
	#[inline]
	pub fn rotation_z(phi: f32) -> Self { Self::rotation(Vec3(0.0, 0.0, 1.0), phi) }
    
    #[inline]
    pub fn rotation_euler_xyz(phi: f32, theta: f32, psi: f32) -> Self
    {
        Self::rotation_z(phi) * Self::rotation_x(theta) * Self::rotation_z(psi)
    }

	#[inline]
	pub const fn scale(Vec3(sx, sy, sz): Vec3) -> Self
	{
		Self
		(
			Vec4(sx, 0.0, 0.0, 0.0),
			Vec4(0.0, sy, 0.0, 0.0),
			Vec4(0.0, 0.0, sz, 0.0),
			Vec4(0.0, 0.0, 0.0, 1.0)
		)
	}

	#[inline]
	pub const fn scale_xyz(s: f32) -> Self { Self::scale(Vec3(s, s, s)) }
	#[inline]
	pub const fn scale_x(s: f32) -> Self { Self::scale(Vec3(s, 1.0, 1.0)) }
	#[inline]
	pub const fn scale_y(s: f32) -> Self { Self::scale(Vec3(1.0, s, 1.0)) }
	#[inline]
	pub const fn scale_z(s: f32) -> Self { Self::scale(Vec3(1.0, 1.0, s)) }

	#[inline]
	pub const fn transpose(self) -> Self
	{
		Self
		(
			Vec4(self.0.0, self.1.0, self.2.0, self.3.0),
        	Vec4(self.0.1, self.1.1, self.2.1, self.3.1),
        	Vec4(self.0.2, self.1.2, self.2.2, self.3.2),
        	Vec4(self.0.3, self.1.3, self.2.3, self.3.3)
        )
	}

	#[inline]
	pub fn transform(self, v: Vec3) -> Vec3
	{
		(self * v.with_w1()).without_w()
	}

	#[inline]
	pub const fn to_array(&self) -> [f32; 16]
	{
		[self.0.0, self.0.1, self.0.2, self.0.3, self.1.0, self.1.1, self.1.2, self.1.3, self.2.0, self.2.1, self.2.2, self.2.3, self.3.0, self.3.1, self.3.2, self.3.3]
	}
}

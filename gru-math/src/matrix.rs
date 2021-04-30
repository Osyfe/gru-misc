use super::*;

//All matrices are column major (=> ready for shaders)!

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Mat2(pub (f32, f32), pub (f32, f32));

impl Display for Mat2
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        write!(f, "({}\t{}\n {}\t{})", self.0.0, self.1.0, self.0.1, self.1.1)
    }
}

impl Mul<Self> for Mat2
{
	type Output = Self;
	fn mul(self, other: Self) -> Self
	{
		Self
		(
			(
				self.0.0 * other.0.0 + self.1.0 * other.0.1,
				self.0.1 * other.0.0 + self.1.1 * other.0.1
			),
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
	fn mul(self, other: Vec2) -> Vec2
	{
		Vec2(self.0.0 * other.0 + self.1.0 * other.1, self.0.1 * other.0 + self.1.1 * other.1)
	}
}

impl_mul_assign!(Mat2, Self);

impl Mat2
{
	pub fn identity() -> Self
	{
		Self((1.0, 0.0), (0.0, 1.0))
	}

	pub fn rotation(phi: f32) -> Self
	{
		let cos = phi.cos();
		let sin = phi.sin();
		Self((cos, sin), (-sin, cos))
	}

	pub fn scale(Vec2(sx, sy): Vec2) -> Self
	{
		Self((sx, 0.0), (0.0, sy))
	}

	pub fn scale_xyz(s: f32) -> Self { Self::scale(Vec2(s, s)) }
	pub fn scale_x(s: f32) -> Self { Self::scale(Vec2(s, 1.0)) }
	pub fn scale_y(s: f32) -> Self { Self::scale(Vec2(1.0, s)) }

	pub fn transpose(self) -> Self
	{
		Self((self.0.0, self.1.0), (self.0.1, self.1.1))
	}

	pub fn to_array(&self) -> [f32; 4]
	{
		[self.0.0, self.0.1, self.1.0, self.1.1]
	}
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Mat3(pub (f32, f32, f32), pub (f32, f32, f32), pub (f32, f32, f32));

impl Display for Mat3
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
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
	fn mul(self, other: Self) -> Self
	{
		Self
		(
			(
				self.0.0 * other.0.0 + self.1.0 * other.0.1 + self.2.0 * other.0.2,
				self.0.1 * other.0.0 + self.1.1 * other.0.1 + self.2.1 * other.0.2,
				self.0.2 * other.0.0 + self.1.2 * other.0.1 + self.2.2 * other.0.2
			),
			(
				self.0.0 * other.1.0 + self.1.0 * other.1.1 + self.2.0 * other.1.2,
				self.0.1 * other.1.0 + self.1.1 * other.1.1 + self.2.1 * other.1.2,
				self.0.2 * other.1.0 + self.1.2 * other.1.1 + self.2.2 * other.1.2
			),
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
	pub fn identity() -> Self
	{
		Self
		(
			(1.0, 0.0, 0.0),
			(0.0, 1.0, 0.0),
			(0.0, 0.0, 1.0)
		)
	}

	pub fn rotation(Vec3(ax, ay, az): Vec3, phi: f32) -> Self
	{
		let cos = phi.cos();
		let one_cos = 1.0 - cos;
		let sin = phi.sin();
		Self
		(
			(ax*ax*one_cos + cos, ay*ax*one_cos + az*sin, az*ax*one_cos - ay*sin),
			(ax*ay*one_cos - az*sin, ay*ay*one_cos + cos, az*ay*one_cos + ax*sin),
			(ax*az*one_cos + ay*sin, ay*az*one_cos - ax*sin, az*az*one_cos + cos)
		)
	}

	pub fn rotation_x(phi: f32) -> Self { Self::rotation(Vec3(1.0, 0.0, 0.0), phi) }
	pub fn rotation_y(phi: f32) -> Self { Self::rotation(Vec3(0.0, 1.0, 0.0), phi) }
	pub fn rotation_z(phi: f32) -> Self { Self::rotation(Vec3(0.0, 0.0, 1.0), phi) }

	pub fn scale(Vec3(sx, sy, sz): Vec3) -> Self
	{
		Self
		(
			(sx, 0.0, 0.0),
			(0.0, sy, 0.0),
			(0.0, 0.0, sz)
		)
	}

	pub fn scale_xyz(s: f32) -> Self { Self::scale(Vec3(s, s, s)) }
	pub fn scale_x(s: f32) -> Self { Self::scale(Vec3(s, 1.0, 1.0)) }
	pub fn scale_y(s: f32) -> Self { Self::scale(Vec3(1.0, s, 1.0)) }
	pub fn scale_z(s: f32) -> Self { Self::scale(Vec3(1.0, 1.0, s)) }

	pub fn transpose(self) -> Self
	{
		Self
		(
			(self.0.0, self.1.0, self.2.0),
        	(self.0.1, self.1.1, self.2.1),
        	(self.0.2, self.1.2, self.2.2)
        )
	}

	pub fn to_mat4(self) -> Mat4
	{
		Mat4
		(
			(self.0.0, self.0.1, self.0.2, 0.0),
        	(self.1.0, self.1.1, self.1.2, 0.0),
        	(self.2.0, self.2.1, self.2.2, 0.0),
        	(0.0, 0.0, 0.0, 1.0)
		)
	}

	pub fn to_array(&self) -> [f32; 9]
	{
		[self.0.0, self.0.1, self.0.2, self.1.0, self.1.1, self.1.2, self.2.0, self.2.1, self.2.2]
	}
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Mat4(pub (f32, f32, f32, f32), pub (f32, f32, f32, f32), pub (f32, f32, f32, f32), pub (f32, f32, f32, f32));

impl Display for Mat4
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
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
	fn mul(self, other: Self) -> Self
	{
		Self
		(
			(
				self.0.0 * other.0.0 + self.1.0 * other.0.1 + self.2.0 * other.0.2 + self.3.0 * other.0.3,
				self.0.1 * other.0.0 + self.1.1 * other.0.1 + self.2.1 * other.0.2 + self.3.1 * other.0.3,
				self.0.2 * other.0.0 + self.1.2 * other.0.1 + self.2.2 * other.0.2 + self.3.2 * other.0.3,
				self.0.3 * other.0.0 + self.1.3 * other.0.1 + self.2.3 * other.0.2 + self.3.3 * other.0.3
			),
			(
				self.0.0 * other.1.0 + self.1.0 * other.1.1 + self.2.0 * other.1.2 + self.3.0 * other.1.3,
				self.0.1 * other.1.0 + self.1.1 * other.1.1 + self.2.1 * other.1.2 + self.3.1 * other.1.3,
				self.0.2 * other.1.0 + self.1.2 * other.1.1 + self.2.2 * other.1.2 + self.3.2 * other.1.3,
				self.0.3 * other.1.0 + self.1.3 * other.1.1 + self.2.3 * other.1.2 + self.3.3 * other.1.3
				
			),
			(
				self.0.0 * other.2.0 + self.1.0 * other.2.1 + self.2.0 * other.2.2 + self.3.0 * other.2.3,
				self.0.1 * other.2.0 + self.1.1 * other.2.1 + self.2.1 * other.2.2 + self.3.1 * other.2.3,
				self.0.2 * other.2.0 + self.1.2 * other.2.1 + self.2.2 * other.2.2 + self.3.2 * other.2.3,
				self.0.3 * other.2.0 + self.1.3 * other.2.1 + self.2.3 * other.2.2 + self.3.3 * other.2.3
				
			),
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
	pub fn identity() -> Self
	{
		Self
		(
			(1.0, 0.0, 0.0, 0.0),
			(0.0, 1.0, 0.0, 0.0),
			(0.0, 0.0, 1.0, 0.0),
			(0.0, 0.0, 0.0, 1.0)
		)
	}

	//z_lin = z_sample * z_near / (z_sample * (z_near - z_far) + z_far)
	pub fn perspective(aspect: f32, fovy: f32, z_near: f32, z_far: f32) -> Self
	{
		let a = 1.0 / (fovy / 2.0).tan();
		let b = z_far / (z_far - z_near);
		Self
		(
			(a / aspect, 0.0, 0.0, 0.0),
			(0.0, a, 0.0, 0.0),
			(0.0, 0.0, b, 1.0),
			(0.0, 0.0, -z_near * b, 0.0)
		)
	}

	pub fn translation(dx: f32, dy: f32, dz: f32) -> Self
	{
		Self
		(
			(1.0, 0.0, 0.0, 0.0),
			(0.0, 1.0, 0.0, 0.0),
			(0.0, 0.0, 1.0, 0.0),
			(dx, dy, dz, 1.0)
		)
	}

	pub fn translation_x(dx: f32) -> Self { Self::translation(dx, 0.0, 0.0) }
	pub fn translation_y(dy: f32) -> Self { Self::translation(0.0, dy, 0.0) }
	pub fn translation_z(dz: f32) -> Self { Self::translation(0.0, 0.0, dz) }

	pub fn rotation(Vec3(ax, ay, az): Vec3, phi: f32) -> Self
	{
		let cos = phi.cos();
		let one_cos = 1.0 - cos;
		let sin = phi.sin();
		Self
		(
			(ax*ax*one_cos + cos, ay*ax*one_cos + az*sin, az*ax*one_cos - ay*sin, 0.0),
			(ax*ay*one_cos - az*sin, ay*ay*one_cos + cos, az*ay*one_cos + ax*sin, 0.0),
			(ax*az*one_cos + ay*sin, ay*az*one_cos - ax*sin, az*az*one_cos + cos, 0.0),
			(0.0, 0.0, 0.0, 1.0)
		)
	}

	pub fn rotation_x(phi: f32) -> Self { Self::rotation(Vec3(1.0, 0.0, 0.0), phi) }
	pub fn rotation_y(phi: f32) -> Self { Self::rotation(Vec3(0.0, 1.0, 0.0), phi) }
	pub fn rotation_z(phi: f32) -> Self { Self::rotation(Vec3(0.0, 0.0, 1.0), phi) }

	pub fn scale(Vec3(sx, sy, sz): Vec3) -> Self
	{
		Self
		(
			(sx, 0.0, 0.0, 0.0),
			(0.0, sy, 0.0, 0.0),
			(0.0, 0.0, sz, 0.0),
			(0.0, 0.0, 0.0, 1.0)
		)
	}

	pub fn scale_xyz(s: f32) -> Self { Self::scale(Vec3(s, s, s)) }
	pub fn scale_x(s: f32) -> Self { Self::scale(Vec3(s, 1.0, 1.0)) }
	pub fn scale_y(s: f32) -> Self { Self::scale(Vec3(1.0, s, 1.0)) }
	pub fn scale_z(s: f32) -> Self { Self::scale(Vec3(1.0, 1.0, s)) }

	pub fn transpose(self) -> Self
	{
		Self
		(
			(self.0.0, self.1.0, self.2.0, self.3.0),
        	(self.0.1, self.1.1, self.2.1, self.3.1),
        	(self.0.2, self.1.2, self.2.2, self.3.2),
        	(self.0.3, self.1.3, self.2.3, self.3.3)
        )
	}

	pub fn transform(self, v: Vec3) -> Vec3
	{
		(self * v.with_w1()).without_w()
	}

	pub fn to_array(&self) -> [f32; 16]
	{
		[self.0.0, self.0.1, self.0.2, self.0.3, self.1.0, self.1.1, self.1.2, self.1.3, self.2.0, self.2.1, self.2.2, self.2.3, self.3.0, self.3.1, self.3.2, self.3.3]
	}
}

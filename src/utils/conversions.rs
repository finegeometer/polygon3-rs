use super::{Line, Point};


impl std::convert::TryFrom<[i32; 3]> for Line {
	type Error = LineMinIntError;
	/// Given [a,b,c], return the line (a*x+b*y+c).cmp(0)
	fn try_from(arr: [i32; 3]) -> Result<Self, LineMinIntError> {
		if arr[0] == std::i32::MIN || arr[1] == std::i32::MIN || arr[2] == std::i32::MIN {
			return Err(LineMinIntError);
		}
		Ok(Self(arr))
	}
}

#[derive(Debug)]
/// This error occurs if you try to make a line with a, b, or c equal to -2_147_483_648 (0x8000_0000), because that number breaks operations like negation.
pub struct LineMinIntError;

impl std::fmt::Display for LineMinIntError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Tried to create a line with coordinate -2_147_483_648 (0x8000_0000). This is not allowed, because it breaks operations like negation.")
	}
}
impl std::error::Error for LineMinIntError {}




impl Into<[i32; 3]> for Line {
	/// Given the line (a*x+b*y+c).cmp(0), return [a,b,c].
	fn into(self) -> [i32; 3] {
		self.0
	}
}


impl std::convert::TryFrom<[i64; 3]> for Point {
	type Error = PointMinIntError;
	/// Create a point from homogeneous coordinates. The point will have the same sign as the z coordinate.
	fn try_from(arr: [i64; 3]) -> Result<Self, PointMinIntError> {
		if arr[0] == std::i64::MIN || arr[1] == std::i64::MIN || arr[2] == std::i64::MIN {
			return Err(PointMinIntError);
		}
		Ok(Self(arr))
	}
}

#[derive(Debug)]
/// This error occurs if you try to make a point with x, y, or z equal to -9_223_372_036_854_775_808 (0x8000_0000_0000_0000), because that number breaks operations like negation.
pub struct PointMinIntError;

impl std::fmt::Display for PointMinIntError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Tried to create a point with coordinate -9_223_372_036_854_775_808 (0x8000_0000_0000_0000). This is not allowed, because it breaks operations like negation.")
	}
}
impl std::error::Error for PointMinIntError {}



impl Into<[i64; 3]> for Point {
	/// Return a point's homogeneous coordinates. The z coordinate will have the same sign as the point.
	fn into(self) -> [i64; 3] {
		self.0
	}
}








impl Line {
	/// Convert from homogeneous coordinates. Inexact, because of floating point input.
	pub fn try_from_f64_array([mut a, mut b, mut c]: [f64; 3]) -> Option<Self> {
		let m = a.abs().max(b.abs()).max(c.abs());

		a /= m;
		b /= m;
		c /= m;

		a *= f64::from(std::i32::MAX);
		b *= f64::from(std::i32::MAX);
		c *= f64::from(std::i32::MAX);

		if a.is_finite() && b.is_finite() && c.is_finite() {
			Some(Self([a as i32, b as i32, c as i32]))
		} else {
			None
		}
	}

	/// Convert to homogeneous coordinates. Exact, despite conversion to floating point.
	pub fn to_f64_array(self) -> [f64; 3] {
		let [a, b, c] = self.0;
		[f64::from(a), f64::from(b), f64::from(c)]
	}
}

/// ```
/// assert_eq!((0x7FFF_FFFF_FFFF_FC00_i64 as f64) as i128, 0x7FFF_FFFF_FFFF_FC00);
/// ```
const MAX_F64_THAT_FITS_IN_I64: f64 = 0x7FFF_FFFF_FFFF_FC00_i64 as f64;

impl Point {
	/// Convert from homogeneous coordinates. Inexact, because of floating point input.
	pub fn try_from_f64_array([mut a, mut b, mut c]: [f64; 3]) -> Option<Self> {
		let m = a.abs().max(b.abs()).max(c.abs());

		a /= m;
		b /= m;
		c /= m;

		a *= MAX_F64_THAT_FITS_IN_I64;
		b *= MAX_F64_THAT_FITS_IN_I64;
		c *= MAX_F64_THAT_FITS_IN_I64;

		if a.is_finite() && b.is_finite() && c.is_finite() {
			Some(Self([a as i64, b as i64, c as i64]))
		} else {
			None
		}
	}

	/// Convert to homogeneous coordinates. Inexact, because of loss of precision in conversion to floating point.
	pub fn to_f64_array(self) -> [f64; 3] {
		let [a, b, c] = self.0;
		[a as f64, b as f64, c as f64]
	}
}

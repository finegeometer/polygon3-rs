mod conversions;

use std::cmp::Ordering;

pub use conversions::{LineMinIntError, PointMinIntError};

// Throughout this module, i32s may not be 0x8000_0000, and i64s may not be 0x8000_0000_0000_0000.

/// A line, with a distinguished positive and negative side.
/// Can be the "line at infinity", in which case all (positive) points are on the same side of the line.
#[derive(Debug, Copy, Clone)]
pub struct Line([i32; 3]);

impl std::ops::Neg for Line {
    type Output = Self;
    /// Reverse the positive and negative sides of the line.
    fn neg(self) -> Self {
        let [a, b, c] = self.0;
        Self([-a, -b, -c])
    }
}

impl Line {
    /// Calculate the intersection of two lines. The resulting point will be positive if the counterclockwise angle from the first to the second is less than 180 degrees.
    /// The lines should not be equal. If they are, the result is the degenerate point [0 : 0 : 0].
    pub fn intersect(self, other: Self) -> Point {
        let [x1, y1, z1] = self.0;
        let [x2, y2, z2] = other.0;
        let x1 = i64::from(x1);
        let y1 = i64::from(y1);
        let z1 = i64::from(z1);
        let x2 = i64::from(x2);
        let y2 = i64::from(y2);
        let z2 = i64::from(z2);
        Point([y1 * z2 - y2 * z1, z1 * x2 - z2 * x1, x1 * y2 - x2 * y1])
    }

    pub(crate) fn slope(self) -> impl Ord {
        let [x, y, _] = self.0;
        match y.cmp(&0) {
            Ordering::Greater => (1, Ratio::new(-x, y)),
            Ordering::Less => (3, Ratio::new(-x, y)),
            Ordering::Equal => match x.cmp(&0) {
                Ordering::Greater => (0, Ratio::ZERO),
                Ordering::Less => (2, Ratio::ZERO),
                Ordering::Equal => panic!("Asked for slope of line at infinity."),
            },
        }
    }

    /// If l1.slope() == l2.slope(), then l1 contains l2 iff l1.distance() <= l2.distance().
    pub(crate) fn distance(self) -> impl Ord {
        let [x, y, z] = self.0;
        Ratio(z, x.abs().max(y.abs()))
    }

    pub(crate) fn is_infinity(self) -> Option<Ordering> {
        if self.0[0] == 0 && self.0[1] == 0 {
            Some(self.0[2].cmp(&0))
        } else {
            None
        }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        match (self.0, other.0) {
            ([0, 0, 0], [0, 0, 0]) => true,
            ([0, 0, 0], _) | (_, [0, 0, 0]) => false,
            (_, _) => self.slope() == other.slope() && self.distance() == other.distance(),
        }
    }
}

impl Eq for Line {}

struct Ratio(i32, i32);

impl Ratio {
    const ZERO: Self = Self(0, 1);

    fn new(a: i32, b: i32) -> Self {
        match b.cmp(&0) {
            Ordering::Greater => Self(a, b),
            Ordering::Less => Self(-a, -b),
            Ordering::Equal => panic!("Divided by zero"),
        }
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = i64::from(self.0);
        let b = i64::from(self.1);
        let c = i64::from(other.0);
        let d = i64::from(other.1);
        (a * d).cmp(&(b * c))
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Ratio {}

impl PartialEq for Ratio {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Debug, Copy, Clone)]
/// A signed point. Negative points behave oppositely when tested against lines.
/// Can also be point at infinity. Opposite points at infinity are not identified.
pub struct Point([i64; 3]);

impl Point {
    /// Test a point against a line.
    /// ```
    /// # use polygon3::{Point, Line};
    /// # use core::convert::TryInto;
    ///
    /// // (1, 1)
    /// let point: Point = [1,1,1].try_into().unwrap();
    /// // x + y + 1 == 0
    /// let line: Line = [1,1,1].try_into().unwrap();
    ///
    /// assert_eq!(point.cmp_line(line), std::cmp::Ordering::Greater);
    ///
    /// // Note that reversing the sign of a point makes it behave oppositely.
    /// assert_eq!((-point).cmp_line(line), std::cmp::Ordering::Less);
    ///
    /// ```
    pub fn cmp_line(self, l: Line) -> Ordering {
        let [x1, y1, z1] = self.0;
        let [x2, y2, z2] = l.0;
        let x1 = i128::from(x1);
        let y1 = i128::from(y1);
        let z1 = i128::from(z1);
        let x2 = i128::from(x2);
        let y2 = i128::from(y2);
        let z2 = i128::from(z2);
        (x1 * x2 + y1 * y2 + z1 * z2).cmp(&0)
    }
}

impl std::ops::Neg for Point {
    type Output = Self;
    /// Invert the sign of a point. Takes points at infinity to their opposite.
    fn neg(self) -> Self {
        let [a, b, c] = self.0;
        Self([-a, -b, -c])
    }
}

impl Point {
    /// Ordering::Greater if positive.
    /// Ordering::Less if negative.
    /// Ordering::Equal if point at infinity.
    pub fn sign(self) -> Ordering {
        self.0[2].cmp(&0)
    }

    pub(crate) fn x_coord(self) -> impl Ord {
        struct Ratio(i64, i64);

        impl Ratio {
            fn new(a: i64, b: i64) -> Self {
                match b.cmp(&0) {
                    Ordering::Greater => Self(a, b),
                    Ordering::Less => Self(-a, -b),
                    Ordering::Equal => panic!("Divided by zero"),
                }
            }
        }

        impl Ord for Ratio {
            fn cmp(&self, other: &Self) -> Ordering {
                let a = i128::from(self.0);
                let b = i128::from(self.1);
                let c = i128::from(other.0);
                let d = i128::from(other.1);
                (a * d).cmp(&(b * c))
            }
        }

        impl PartialOrd for Ratio {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Eq for Ratio {}

        impl PartialEq for Ratio {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == Ordering::Equal
            }
        }

        let [x, _, z] = self.0;

        Ratio::new(x, z)
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct UnorientedLine(pub Line);

impl UnorientedLine {
    pub fn intersect(self, other: Self) -> Point {
        let p = self.0.intersect(other.0);
        if p.0[2] < 0 {
            -p
        } else {
            p
        }
    }

    pub fn angle_from_horizontal(self) -> impl Ord {
        let [x, y, _] = (self.0).0;

        if x == 0 {
            (0, Ratio::ZERO)
        } else {
            (1, Ratio::new(y, x))
        }
    }
}

impl PartialEq for UnorientedLine {
    fn eq(&self, other: &Self) -> bool {
        self.intersect(*other).0 == [0, 0, 0]
    }
}

pub fn pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    slice.iter().zip(slice.iter().cycle().skip(1))
}

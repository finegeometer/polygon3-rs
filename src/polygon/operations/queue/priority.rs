use crate::utils::Point;
use std::cmp::Ordering;

/// Reversed lexicographic ordering of points.
/// Larger points are hit first by the sweep line.
pub fn priority(point: Point) -> impl Ord {
    let [x, y, z]: [i64; 3] = point.into();
    std::cmp::Reverse((Ratio::new(y, z), Ratio::new(x, z)))
}

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

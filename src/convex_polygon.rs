use std::cmp::Ordering;
use crate::utils::{Line, Point};

#[derive(Debug, Clone)]
/// A convex region of the plane. Can be infinite or finite, but cannot be empty.
pub struct ConvexPolygon(Vec<Line>);


/// Given three lines, is the middle unnecessary to define the closed region?
/// ```text
/// \     /
///  \   /   -> FALSE
/// --\-/--
///
///  \   /
///   \ /
///    X    -> TRUE
/// --/-\--
/// ```
fn is_middle_line_removable(l1: Line, l2: Line, l3: Line) -> bool {
	let p = l1.intersect(l3);
	p.sign() == Ordering::Greater && p.cmp_line(l2) != Ordering::Less
}


impl ConvexPolygon {

	/// Does the polygon contain the point?
	/// Return `Greater` if yes, `Less` if no, and `Equal` if on the boundary.
	pub fn contains(&self, pt: Point) -> Ordering {
		self.0.iter().map(|l| pt.cmp_line(*l)).min().unwrap_or(Ordering::Greater)
	}

	/// Given a collection of half-planes,
	/// calculate the convex polygon that is their intersection.
	///
	///
	/// # Correctness
	/// This function has been fuzzed for a week through `ConvexPolygon::test`, and reported no errors.
	///
	/// ```text
	///                         american fuzzy lop 2.52b (fuzz)
	/// 
	/// ┌─ process timing ─────────────────────────────────────┬─ overall results ─────┐
	/// │        run time : 7 days, 7 hrs, 16 min, 2 sec       │  cycles done : 3      │
	/// │   last new path : 0 days, 4 hrs, 50 min, 54 sec      │  total paths : 437    │
	/// │ last uniq crash : none seen yet                      │ uniq crashes : 0      │
	/// │  last uniq hang : none seen yet                      │   uniq hangs : 0      │
	/// ├─ cycle progress ────────────────────┬─ map coverage ─┴───────────────────────┤
	/// │  now processing : 379* (86.73%)     │    map density : 0.86% / 1.28%         │
	/// │ paths timed out : 0 (0.00%)         │ count coverage : 4.18 bits/tuple       │
	/// ├─ stage progress ────────────────────┼─ findings in depth ────────────────────┤
	/// │  now trying : arith 16/8            │ favored paths : 41 (9.38%)             │
	/// │ stage execs : 2.18M/3.23M (67.55%)  │  new edges on : 62 (14.19%)            │
	/// │ total execs : 213M                  │ total crashes : 0 (0 unique)           │
	/// │  exec speed : 536.8/sec             │  total tmouts : 508 (8 unique)         │
	/// ├─ fuzzing strategy yields ───────────┴───────────────┬─ path geometry ────────┤
	/// │   bit flips : 24/7.72M, 3/7.72M, 2/7.72M            │    levels : 12         │
	/// │  byte flips : 0/965k, 0/963k, 0/963k                │   pending : 219        │
	/// │ arithmetics : 1/53.9M, 0/18.3M, 0/744k              │  pend fav : 0          │
	/// │  known ints : 2/5.52M, 5/24.0M, 9/40.2M             │ own finds : 436        │
	/// │  dictionary : 0/0, 0/0, 2/41.1M                     │  imported : n/a        │
	/// │       havoc : 388/1.34M, 0/0                        │ stability : 99.76%     │
	/// │        trim : 1.21%/67.6k, 0.17%                    ├────────────────────────┘
	/// └─────────────────────────────────────────────────────┘          [cpu000: 25%]
	/// ```
	pub fn from_boundaries(boundaries: impl IntoIterator<Item=Line>) -> Option<Self> {

		// Remove lines at infinity
		let mut boundaries: Vec<_> = boundaries.into_iter().filter(|l| l.is_infinity() != Some(Ordering::Greater)).collect();

		if boundaries.iter().any(|l| l.is_infinity().is_some()) {
			return None;
		}

		boundaries.sort_unstable_by_key(|l| (l.slope(), l.distance()));
		boundaries.dedup_by_key(|l| l.slope());


		// Now we eliminate redundant edges.

		let mut boundaries = boundaries.into_iter();

		use std::collections::VecDeque;

		let mut out = VecDeque::new();

		let here;

		if let Some(x) = boundaries.next() {
			here = x;
		} else {
			return Some(Self(Vec::new()));
		}

		let mut boundaries = boundaries.chain(std::iter::once(here));
		let mut l2: Line = boundaries.next().unwrap();

		for l3 in boundaries {
			let mut push = true;
			while is_middle_line_removable(*out.back().unwrap_or(&here), l2, l3) {
				if let Some(x) = out.pop_back() {
					l2 = x;
				} else {
					push = false;
					break;
				}
			}
			if push {
				out.push_back(l2);
			}
			l2 = l3;
		}

		let mut here = here;

		while let (Some(&back), Some(&front)) = (out.back(), out.front()) {
			if !is_middle_line_removable(back, here, front) {
				break;
			}

			here = out.pop_front().unwrap();

			while let Some(&front) = out.front() {
				if !is_middle_line_removable(back, here, front) {
					break;
				}

				here = out.pop_front().unwrap();
			}

			out.push_front(here);
			here = out.pop_back().unwrap();
		}

		out.push_back(here);

		let out: Vec<Line> = out.into();

		match out.len() {
			2 => {
				if opposite_overlapping(out[0], out[1]) {return None;}
			}
			3 => {
				let p = out[0].intersect(out[1]);
				match p.sign() {
					Ordering::Greater => {
						if p.cmp_line(out[2]) != Ordering::Greater {
							return None;
						}
					}
					Ordering::Equal => {
						if (out[1].intersect(out[2])).cmp_line(out[0]) != Ordering::Greater {
							return None;
						}
					}
					Ordering::Less => {}
				}
			}
			4 => {
				if opposite_overlapping(out[0], out[2]) || opposite_overlapping(out[1], out[3]) {return None;}
			}
			_ => {}
		}

		Some(Self(out))

	}
}

impl Into<Vec<Line>> for ConvexPolygon {
	/// Get the edges of a convex polygon, in clockwise order.
	fn into(self) -> Vec<Line> {
		self.0
	}
}

fn opposite_overlapping(l1: Line, l2: Line) -> bool {
	let l2 = -l2;	
	l1.slope() == l2.slope() && l1.distance() <= l2.distance()
}



/// Tests
impl ConvexPolygon {
	/// Assert that every vertex of the polygon is strictly inside the region formed by the non-adjacent edges.
	///
	/// If the library is bug-free, this always passes.
	pub fn assert_valid(&self) {
		let edges = &self.0;
		let n = edges.len();
		for i in 0..n {
			let j = (i + 1) % n;
			let vertex = edges[i].intersect(edges[j]);
			
			assert_eq!(
				edges.iter().enumerate().filter_map(|(k, edge)| {
					if k == i || k == j {
						None
					} else {
						Some(vertex.cmp_line(*edge))
					}
				}).min().unwrap_or(Ordering::Greater),
				Ordering::Greater
			)
		}
	}

	/// Create a polygon from a collection of boundaries.
	/// Assert that `point` is contained in the polygon if and only if it is on the positive side of all of the boundaries.
	/// Also assert that the polygon is valid (see `assert_valid`)
	///
	/// If the library is bug-free, this always passes.
	pub fn test(point: Point, boundaries: Vec<Line>) {

		let boundaries_contain_point = boundaries.iter().map(|l| point.cmp_line(*l)).min().unwrap_or(Ordering::Greater);

		#[cfg(test)]
		println!("{:?}", boundaries);

		if let Some(poly) = ConvexPolygon::from_boundaries(boundaries.iter().copied()) {

			#[cfg(test)]
			println!("{:?}", poly);

			poly.assert_valid();
			assert_eq!(boundaries_contain_point, poly.contains(point));
		} else {
			assert_ne!(boundaries_contain_point, Ordering::Greater);
		}

	}
}



#[test]
fn test_square() {
	use std::convert::TryInto;
	let boundaries = vec![
		[1, 0, 1].try_into().unwrap(),
		[-1, 0, 1].try_into().unwrap(),
		[0, 1, 1].try_into().unwrap(),
		[0, -1, 1].try_into().unwrap(),
	];

	let point = [0, 0, 1].try_into().unwrap();
	ConvexPolygon::test(point, boundaries.clone());

	let point = [2, 0, 1].try_into().unwrap();
	ConvexPolygon::test(point, boundaries);
}


// [Line([2139062143, 2139062110, 2139068031]), Line([2139039784, 679444351, 2139062143]), Line([2139062143, 2139062110, 2139068031]), Line([2139062143, 2136899455, 2139034231]), Line([2139062110, -129, -8912640])]
// ConvexPolygon([])






use crate::utils::{Line, UnorientedLine};
use crate::convex_polygon::ConvexPolygon;
use std::cmp::Ordering;

mod operations;
mod contains;

/// A bounded region of the plane whose boundary is made of line segments.
/// May have multiple disconnected components, and may have holes.
#[derive(Debug, Clone)]
pub struct Polygon(Vec<Vec<UnorientedLine>>);

impl std::convert::TryFrom<ConvexPolygon> for Polygon {
	type Error = InfiniteRegionError;
	/// Convert a convex region of the plane into a polygon, failing if it is infinite.
	/// 
	/// ## Correctness
	/// This function has been fuzzed for half a day, and reported no errors.
	///
	/// ```text
	///                         american fuzzy lop 2.52b (fuzz)
	/// 
	/// ┌─ process timing ─────────────────────────────────────┬─ overall results ─────┐
	/// │        run time : 0 days, 14 hrs, 20 min, 6 sec      │  cycles done : 4      │
	/// │   last new path : 0 days, 1 hrs, 35 min, 18 sec      │  total paths : 387    │
	/// │ last uniq crash : none seen yet                      │ uniq crashes : 0      │
	/// │  last uniq hang : none seen yet                      │   uniq hangs : 0      │
	/// ├─ cycle progress ────────────────────┬─ map coverage ─┴───────────────────────┤
	/// │  now processing : 354* (91.47%)     │    map density : 0.76% / 1.30%         │
	/// │ paths timed out : 0 (0.00%)         │ count coverage : 3.96 bits/tuple       │
	/// ├─ stage progress ────────────────────┼─ findings in depth ────────────────────┤
	/// │  now trying : auto extras (over)    │ favored paths : 40 (10.34%)            │
	/// │ stage execs : 176k/2.03M (8.69%)    │  new edges on : 68 (17.57%)            │
	/// │ total execs : 41.4M                 │ total crashes : 0 (0 unique)           │
	/// │  exec speed : 452.1/sec             │  total tmouts : 8 (2 unique)           │
	/// ├─ fuzzing strategy yields ───────────┴───────────────┬─ path geometry ────────┤
	/// │   bit flips : 20/1.66M, 1/1.66M, 2/1.66M            │    levels : 11         │
	/// │  byte flips : 0/207k, 0/206k, 0/206k                │   pending : 207        │
	/// │ arithmetics : 4/11.5M, 0/2.59M, 0/489k              │  pend fav : 0          │
	/// │  known ints : 2/1.37M, 1/5.53M, 3/8.88M             │ own finds : 386        │
	/// │  dictionary : 0/0, 0/0, 2/4.24M                     │  imported : n/a        │
	/// │       havoc : 351/991k, 0/0                         │ stability : 99.76%     │
	/// │        trim : 0.14%/31.0k, 0.71%                    ├────────────────────────┘
	/// └─────────────────────────────────────────────────────┘          [cpu000: 28%]
	/// ```
	fn try_from(poly: ConvexPolygon) -> Result<Self, InfiniteRegionError> {

		let edges: Vec<Line> = poly.into();

		if edges.is_empty() {
			return Err(InfiniteRegionError);
		}

		for i in 0..edges.len() {
			let j = (i+1) % edges.len();
			if let Ordering::Greater = edges[i].intersect(edges[j]).sign() {
				// Empty if block
			} else {
				return Err(InfiniteRegionError);
			}
		}
		// I hope this is compiled out
		let edges: Vec<UnorientedLine> = edges.into_iter().map(UnorientedLine).collect();
		Ok(Self(vec![edges]))
	}
}

#[derive(Debug)]
pub struct InfiniteRegionError;

impl std::fmt::Display for InfiniteRegionError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Tried to convert an infinite ConvexPolygon to a Polygon.")
	}
}
impl std::error::Error for InfiniteRegionError {}


impl Default for Polygon {
	fn default() -> Self {
		Self(Vec::new())
	}
}



impl Polygon {
	pub fn try_from_edges(polys: Vec<Vec<Line>>) -> Option<Self> {
		let mut out = Vec::with_capacity(polys.len());
		for edges in polys {
			if edges.len() < 3 {
				return None;
			}
			for i in 0..edges.len() {
				let j = (i+1) % edges.len();
				let [_,_,z]: [i64; 3] = edges[i].intersect(edges[j]).into();
				if z == 0 {
					return None;
				}
			}

			out.push(edges.into_iter().map(UnorientedLine).collect());
		}
		Some(Self(out))
	}
}
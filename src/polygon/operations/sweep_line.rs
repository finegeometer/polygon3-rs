mod chain_end_connector;
use chain_end_connector::*;

use crate::utils::{Point, UnorientedLine};
use bit_vec::BitVec;
use std::cmp::Ordering;

// Sweep line:
// 	Line y + εx = c
// 	c increasing over time
// 	ε > 0, but infinitesimal

// We track the edges that are crossing the sweep-line, and track which regions between are in which polygons.


pub(super) struct SweepLine {
	edges: Vec<Edge>,
	regions: Vec<BitVec>,
	inside: fn(&BitVec) -> bool,
	pub out: Vec<Vec<UnorientedLine>>,
	num_polys: usize,
}

struct Edge {
	line: UnorientedLine,
	polys: BitVec,
	out_chain_end: Option<chain_end::ChainEnd<UnorientedLine>>,
}

impl SweepLine {
	pub fn new(num_polys: usize, inside: fn(&BitVec) -> bool) -> Self {
		Self {
			edges: Vec::new(),
			regions: vec![BitVec::from_elem(num_polys, false)],
			inside,
			out: Vec::new(),
			num_polys,
		}
	}

	fn section_reversed(&mut self, range: std::ops::Range<usize>) -> SweepLineSection {
		let mut relevant_edges: Vec<Edge> = self.edges.splice(range.clone(), std::iter::empty()).rev().collect();
		let mut end_connector = ChainEndConnector::new();

		for e in relevant_edges.iter_mut() {
			if let Some(e) = e.out_chain_end.take() {
				end_connector.end(&mut self.out, e);
			}
		}


		SweepLineSection {
			sweep_line: self,
			range,
			relevant_edges,
			end_connector,
		}
	}

	pub fn relevant_section_reversed(&mut self, pt: Point) -> SweepLineSection {
		let search_fn = |e: &Edge| {
			let mut line = e.line.0;
			let [x,_,_]: [i32; 3] = line.into();
			// Make the line point toward negative x.
			match x.cmp(&0) {
				Ordering::Greater => {line = -line;}
				Ordering::Equal => {return Ordering::Equal;}
				_ => {}
			}
			pt.cmp_line(line)
		};

		let range = match self.edges.binary_search_by(search_fn) {
			Err(n) => n..n,
			Ok(n) => {

				let mut start = n;
				loop {
					if let Some(prev) = start.checked_sub(1) {
						if search_fn(&self.edges[prev]) == Ordering::Equal {
							start = prev;
							continue;
						}
					}
					break;
				}

				let mut end = n;
				loop {
					end += 1;
					if let Some(x) = self.edges.get(end) {
						if search_fn(x) == Ordering::Equal {
							continue;
						}
					}
					break;
				}

				start..end
			}
		};

		self.section_reversed(range)
	}
}

pub(crate) struct SweepLineSection<'r> {
	sweep_line: &'r mut SweepLine,
	range: std::ops::Range<usize>,
	// Note: None of the relevant edges will have output chains attached.
	relevant_edges: Vec<Edge>,
	end_connector: ChainEndConnector,
}


impl<'r> Drop for SweepLineSection<'r> {
	fn drop(&mut self) {
		let Self {sweep_line, range, relevant_edges, end_connector} = self;
		let mut relevant_edges: Vec<Edge> = std::mem::replace(relevant_edges, Vec::new());

		let mut region: BitVec = sweep_line.regions[range.start].clone();
		let mut regions: Vec<BitVec> = Vec::new();
		for edge in &mut relevant_edges {
			regions.push(region.clone());

			let bool1 = (sweep_line.inside)(&region);
			region.xor(&edge.polys);
			let bool2 = (sweep_line.inside)(&region);

			if bool1 ^ bool2 {
				let (e1, e2) = chain_end::ChainEnd::new(std::iter::once(edge.line));
				end_connector.end(&mut sweep_line.out, e1);
				edge.out_chain_end = Some(e2);
			}
		}
		sweep_line.regions.splice(range.clone(), regions);

		sweep_line.edges.splice(range.start .. range.start, relevant_edges);
	}
}


impl<'r> SweepLineSection<'r> {
	pub fn insert(&mut self, line: UnorientedLine, poly_idx: usize) {

		// \ < | < / < -
		let search_fn = |l: UnorientedLine| {
			std::cmp::Reverse(l.angle_from_horizontal())
		};

		match self.relevant_edges.binary_search_by_key(&search_fn(line), |e| search_fn(e.line)) {
			Err(n) => {
				let mut polys = BitVec::from_elem(self.sweep_line.num_polys, false);
				polys.set(poly_idx, true);
				self.relevant_edges.insert(n, Edge {
					line,
					polys,
					out_chain_end: None,
				});
			}
			Ok(n) => {
				let tmp: bool = self.relevant_edges[n].polys[poly_idx];
				self.relevant_edges[n].polys.set(poly_idx, !tmp);

				if self.relevant_edges[n].polys.none() {
					self.relevant_edges.remove(n);
				}
			}
		}
	}

	pub fn boundary_intersections(&mut self) -> impl Iterator<Item = Point> {
		let n = self.range.start;

		let edge_00 = n.checked_sub(1).map(|m| {
			&self.sweep_line.edges[m]
		});
		let edge_11 = self.sweep_line.edges.get(n);

		let out = if let (Some(edge_01), Some(edge_10)) = (self.relevant_edges.first(), self.relevant_edges.last()) {
			( edge_00.map(|edge_00| edge_00.line.intersect(edge_01.line))
			, edge_11.map(|edge_11| edge_11.line.intersect(edge_10.line))
			)
		} else if let (Some(edge_00), Some(edge_11)) = (edge_00, edge_11) {
			(Some(edge_00.line.intersect(edge_11.line)), None)
		} else {
			(None, None)
		};

		out.0.into_iter().chain(out.1)
	}
}




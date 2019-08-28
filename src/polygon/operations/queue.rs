mod priority;

use std::cmp::Ordering;
use crate::utils::{Point, UnorientedLine};
use std::collections::BinaryHeap;
use priority::priority;

#[derive(Debug, Copy, Clone)]
pub(super) struct Event {
	point: Point, // sign positive
	edges: Option<[(UnorientedLine, usize);2]>,
}


impl Event {
	// Point may not be at infinity.
	pub fn new_intersection(mut point: Point) -> Self {
		if point.sign() == Ordering::Less {point = -point};
		Self {
			point,
			edges: None,
		}
	}
	// Point may not be at infinity.
	pub fn new_vertex(mut point: Point, edges: [(UnorientedLine, usize); 2]) -> Self {
		if point.sign() == Ordering::Less {point = -point};
		Self {
			point,
			edges: Some(edges),
		}
	}
}

impl PartialEq for Event {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other) == Ordering::Equal
	}
}
impl Eq for Event {}
impl PartialOrd for Event {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl Ord for Event {
	fn cmp(&self, other: &Self) -> Ordering {
		priority(self.point).cmp(&priority(other.point))
	}
}




/// A Queue of events
#[derive(Debug)]
pub(super) struct Q {
	queue: BinaryHeap<Event>,
	last_evt: Option<Event>,
}

impl Q {
	pub fn new() -> Self {
		Self {
			queue: BinaryHeap::new(),
			last_evt: None,
		}
	}

	/// Push an event, if it is not already in our past.
	pub fn push(&mut self, event: Event) {
		match self.last_evt {
			// The ordering's reversed: max is first
			Some(e) if e <= event => {}
			_ => {
				self.queue.push(event);
			}
		}
	}

	/// Get everything that happens at the next interesting point on the sweep-line.
	/// Return the point, and put the line endings in `line_endings`.
	pub fn next_event(&mut self, line_endings: &mut Vec<(UnorientedLine, usize)>) -> Option<Point> {
		line_endings.clear();
		let event = *self.queue.peek()?;
		// Get everything of the same priority.
		while let Some(&e) = self.queue.peek() {
			if e != event {
				break;
			}
			self.queue.pop();

			if let Some([e1, e2]) = e.edges {
				line_endings.push(e1);
				line_endings.push(e2);
			}
		}

		self.last_evt = Some(event);
		Some(event.point)
	}
}

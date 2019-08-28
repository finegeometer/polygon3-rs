use crate::utils::{Point, Line};
use std::cmp::Ordering;
use super::Polygon;

impl Polygon {

	/// Does a polygon contain a given point?
	/// Return Ordering::Greater if yes.
	/// Return Ordering::Less if no.
	/// Return Ordering::Equal if on the boundary.
	pub fn contains(&self, point: Point) -> Ordering {

		// Points at infinity.
		if point.sign() == Ordering::Equal {
			return Ordering::Less;
		}

		let mut inside = false;

		for poly in &self.0 {


			let n = poly.len();
			for i in 0..n {
				let j = (i + 1) % n;
				let k = (i + 2) % n;

				let edge: Line = {
					let [_,y,_]: [i32; 3] = poly[j].0.into();

					if y < 0 {
						-poly[j].0
					} else {
						poly[j].0
					}
				};

				let pt_1 = poly[i].intersect(poly[j]);
				let pt_2 = poly[j].intersect(poly[k]);

				match point.cmp_line(edge) {
					Ordering::Less => {
						let x1 = pt_1.x_coord();
						let x2 = point.x_coord();
						let x3 = pt_2.x_coord();

						if (x1 < x2) ^ (x3 < x2) {
							inside ^= true;
						}
					}
					Ordering::Equal => {


						match point.cmp_line(poly[i].0) {
							Ordering::Equal => {return Ordering::Equal;}
							o if o != pt_2.cmp_line(poly[i].0) => {continue;}
							_ => {}
						}
						match point.cmp_line(poly[k].0) {
							Ordering::Equal => {return Ordering::Equal;}
							o if o != pt_1.cmp_line(poly[k].0) => {continue;}
							_ => {}
						}
						return Ordering::Equal;
					}
					Ordering::Greater => {}
				}

			}
		}

		if inside {
			Ordering::Greater
		} else {
			Ordering::Less
		}
	}
}


#[test]
fn test_square() {
	use std::convert::TryInto;
	use crate::ConvexPolygon;

	let boundaries = vec![
		[1, 0, 1].try_into().unwrap(),
		[-1, 0, 1].try_into().unwrap(),
		[0, 1, 1].try_into().unwrap(),
		[0, -1, 1].try_into().unwrap(),
	];
	let square: Polygon = ConvexPolygon::from_boundaries(boundaries.into_iter()).unwrap().try_into().unwrap();


	for x in -2 ..= 2 {
		for y in -2 ..= 2 {
			let point: Point = [x, y, 1].try_into().unwrap();

			println!("{:?}", (x,y));
			assert_eq!(square.contains(point), 1.cmp(&x.abs().max(y.abs())));
		}
	}

}

#[test]
fn test_diamond() {
	use std::convert::TryInto;
	use crate::ConvexPolygon;

	let boundaries = vec![
		[1, 1, 1].try_into().unwrap(),
		[-1, 1, 1].try_into().unwrap(),
		[1, -1, 1].try_into().unwrap(),
		[-1, -1, 1].try_into().unwrap(),
	];
	let diamond: Polygon = ConvexPolygon::from_boundaries(boundaries.into_iter()).unwrap().try_into().unwrap();


	for x in -2 ..= 2 {
		for y in -2 ..= 2 {
			let point: Point = [x, y, 1].try_into().unwrap();

			println!("{:?}", (x,y));
			assert_eq!(diamond.contains(point), 1.cmp(&(x.abs() + y.abs())));
		}
	}

}

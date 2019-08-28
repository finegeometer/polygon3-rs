#[macro_use]
extern crate afl;

use std::convert::{TryFrom, TryInto};
use polygon3::{Point, Line, ConvexPolygon, Polygon};
use std::cmp::Ordering;


fn parse_i32(data: &mut &[u8]) -> Option<i32> {
	if data.len() < 4 {
		return None;
	}
	let arr: [u8; 4] = data[0..4].try_into().ok()?;
	*data = &data[4..];
	Some(i32::from_be_bytes(arr))
}

fn parse_i64(data: &mut &[u8]) -> Option<i64> {
	if data.len() < 8 {
		return None;
	}
	let arr: [u8; 8] = data[0..8].try_into().ok()?;
	*data = &data[8..];
	Some(i64::from_be_bytes(arr))
}

fn parse_line(data: &mut &[u8]) -> Option<Line> {
	let a = parse_i32(data)?;
	let b = parse_i32(data)?;
	let c = parse_i32(data)?;
	[a,b,c].try_into().ok()
}

fn parse_point(data: &mut &[u8]) -> Option<Point> {
	let a = parse_i64(data)?;
	let b = parse_i64(data)?;
	let c = parse_i64(data)?;
	[a,b,c].try_into().ok()
}

fn parse_edge_vec(data: &mut &[u8]) -> Option<Vec<Line>> {
	let n = data.get(0)? % 8;
	*data = &data[1..];
	let mut out = Vec::new();
	for _ in 0 .. n+3 {
		out.push(parse_line(data)?);
	}
	Some(out)
}

fn parse_polygon(data: &mut &[u8]) -> Option<Polygon> {
	let n = data.get(0)? % 4;
	*data = &data[1..];
	let mut out = Vec::new();
	for _ in 0..n {
		out.push(parse_edge_vec(data)?);
	}
	Some(Polygon::try_from_edges(out)?)
}

fn parse_polygons(data: &mut &[u8]) -> Option<Vec<Polygon>> {
	let n = data.get(0)? % 4;
	*data = &data[1..];
	let mut out = Vec::new();
	for _ in 0..n {
		out.push(parse_polygon(data)?);
	}
	Some(out)
}


fn fuzz_convex_polygon_from_boundaries(mut data: &[u8]) {
	let point;
	if let Some(x) = parse_point(&mut data) {
		match x.sign() {
			Ordering::Greater => {point = x;}
			Ordering::Equal => {return;}
			Ordering::Less => {point = -x;}
		}
	} else {
		return;
	}

	let mut boundaries = Vec::new();
	while let Some(x) = parse_line(&mut data) {
		boundaries.push(x);
	}

	ConvexPolygon::test(point, boundaries);
}


fn fuzz_polygon_containment(mut data: &[u8]) {
	let point;
	if let Some(x) = parse_point(&mut data) {
		match x.sign() {
			Ordering::Greater => {point = x;}
			Ordering::Equal => {return;}
			Ordering::Less => {point = -x;}
		}
	} else {
		return;
	}

	let mut boundaries = Vec::new();
	while let Some(x) = parse_line(&mut data) {
		boundaries.push(x);
	}


	if let Some(convex_polygon) = ConvexPolygon::from_boundaries(boundaries.into_iter()) {

		// println!("{:?}", convex_polygon);

		let answer1 = convex_polygon.contains(point);

		if let Ok(polygon) = Polygon::try_from(convex_polygon) {

			// println!("{:?}", polygon);

			let answer2 = polygon.contains(point);
			assert_eq!(answer1, answer2);
		}
	}
}

fn fuzz_polygon_difference(mut data: &[u8]) {
	let point;
	if let Some(x) = parse_point(&mut data) {
		match x.sign() {
			Ordering::Greater => {point = x;}
			Ordering::Equal => {return;}
			Ordering::Less => {point = -x;}
		}
	} else {
		return;
	}

	if let Some(polygon) = parse_polygon(&mut data) {
		if let Some(polygons) = parse_polygons(&mut data) {
			polygon.test_difference(point, polygons)
		}
	}

}



fn main() {
    fuzz!(|data: &[u8]| {
        // fuzz_convex_polygon_from_boundaries(data);
        // fuzz_polygon_containment(data);
        fuzz_polygon_difference(data);
    });
}

#[test]
fn get_crash() {
    fuzz_polygon_difference(include_bytes!("../out/crashes/<insert file here>"));
}

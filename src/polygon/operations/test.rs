use super::Polygon;
use crate::utils::*;
use crate::*;
use std::convert::TryInto;

fn square() -> Polygon {
    let boundaries = vec![
        [1, 0, 5].try_into().unwrap(),
        [-1, 0, 5].try_into().unwrap(),
        [0, 1, 5].try_into().unwrap(),
        [0, -1, 5].try_into().unwrap(),
    ];
    ConvexPolygon::from_boundaries(boundaries.into_iter())
        .unwrap()
        .try_into()
        .unwrap()
}

fn diamond() -> Polygon {
    let boundaries = vec![
        [1, 1, 7].try_into().unwrap(),
        [-1, 1, 7].try_into().unwrap(),
        [1, -1, 7].try_into().unwrap(),
        [-1, -1, 7].try_into().unwrap(),
    ];
    ConvexPolygon::from_boundaries(boundaries.into_iter())
        .unwrap()
        .try_into()
        .unwrap()
}

fn bowtie() -> Polygon {
    Polygon(vec![vec![
        UnorientedLine([0, 1, 7].try_into().unwrap()),
        UnorientedLine([1, 1, 0].try_into().unwrap()),
        UnorientedLine([0, -1, 7].try_into().unwrap()),
        UnorientedLine([1, -1, 0].try_into().unwrap()),
    ]])
}

#[test]
fn empty_union() {
    assert!(Polygon::union(std::iter::empty()).0.is_empty());
}

#[test]
fn square_minus_nothing() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            square().test_difference(point, Vec::new());
        }
    }
}

#[test]
fn bowtie_minus_nothing() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            bowtie().test_difference(point, Vec::new());
        }
    }
}

#[test]
fn square_minus_self() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            println!("{:?}", (x, y));
            square().test_difference(point, vec![square()]);
        }
    }
}

#[test]
fn bowtie_minus_square() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            println!("{:?}", (x, y));
            bowtie().test_difference(point, vec![square()]);
        }
    }
}

#[test]
fn square_minus_bowtie() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            println!("{:?}", (x, y));
            bowtie().test_difference(point, vec![square()]);
        }
    }
}

#[test]
fn bowtie_minus_self() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            println!("{:?}", (x, y));
            bowtie().test_difference(point, vec![bowtie()]);
        }
    }
}

#[test]
fn diamond_minus_bowtie() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            println!("{:?}", (x, y));
            diamond().test_difference(point, vec![bowtie()]);
        }
    }
}

#[test]
fn bowtie_minus_diamond() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            println!("{:?}", (x, y));
            bowtie().test_difference(point, vec![diamond()]);
        }
    }
}

#[test]
fn diamond_minus_square() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            println!("{:?}", (x, y));
            diamond().test_difference(point, vec![square()]);
        }
    }
}

#[test]
fn square_minus_diamond() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            println!("{:?}", (x, y));
            square().test_difference(point, vec![diamond()]);
        }
    }
}

#[test]
fn bowtie_minus_square_and_diamond() {
    for x in -10..=10 {
        for y in -10..=10 {
            let point: Point = [x, y, 1].try_into().unwrap();

            println!("{:?}", (x, y));
            bowtie().test_difference(point, vec![square(), diamond()]);
        }
    }
}

#![forbid(unsafe_code)]

//! Polygon boolean operations, focused on correctness.
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!


mod utils;
mod convex_polygon;
mod polygon;

pub use utils::{Line, Point, LineMinIntError, PointMinIntError};
pub use convex_polygon::ConvexPolygon;
pub use polygon::Polygon;


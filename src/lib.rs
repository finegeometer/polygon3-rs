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

mod convex_polygon;
mod polygon;
mod utils;

pub use convex_polygon::ConvexPolygon;
pub use polygon::Polygon;
pub use utils::{Line, LineMinIntError, Point, PointMinIntError};

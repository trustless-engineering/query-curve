//! # QueryCurve
//!
//! A library for querying custom curves defined by Bezier curves, which can be created at
//! https://querycurve.com/

pub mod base62;
pub mod constants;
pub mod query_curve;
pub mod types;
pub mod utils;

// Re-export functions for easier access
pub use query_curve::{get_encoded_curve_query_function, query_curve, query_encoded_curve};

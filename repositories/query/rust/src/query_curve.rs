use crate::types::{BezierSegment, EncodedScaledBezierChain, ScaledBezierChain};
use crate::utils::decode;

/// Finds the y value for a given x value along a cubic Bezier curve.
///
/// # Arguments
///
/// * `curve` - The cubic Bezier curve, outputted from `query-curve/builder`. First 4 elements are (scaleFactorX, scaleFactorY, offsetX, offsetY), rest are the points and handles.
/// * `scaled_x` - The x value for which we will find the associated y value along the curve. This should reflect the value in the scaled & offset coordinate space.
///
/// # Returns
///
/// The y value for the given x value along the curve.
pub fn query_curve(curve: &ScaledBezierChain, scaled_x: f64) -> Option<f64> {
    if curve.len() < 8 {
        return None;
    }

    let scale_x = curve[0];
    let scale_y = curve[1];
    let offset_x = curve[2];
    let offset_y = curve[3];

    if scale_x == 0.0 || scale_y == 0.0 {
        panic!("Scale factors cannot be 0");
    }

    let x = (scaled_x / scale_x) - offset_x;

    let first = 4;
    let last = curve.len() - 1;

    if x < curve[first] || x > curve[last - 1] {
        return None;
    }

    // If any point exactly matches the x value, return the scaled y value. This
    // is not just an optimization, but also reduces floating point errors. It's
    // also the case that values queried at key points, such as first and last,
    // are important to be exact.
    // TODO - This could be optimized by using a binary search
    // TODO - this could potentially be optimized by combining it with
    // the loop being used to find the segmentStartIndex, and simply don't break
    // out when the segment is found to allow the loop to continue to check for exact matches.
    for i in (first..curve.len()).step_by(6) {
        if (curve[i] - x).abs() < 1e-15 {
            return Some(to_external_coordinate(curve[i + 1], scale_y, offset_y));
        }
    }

    let mut segment_start_index = None;
    for i in (first..curve.len() - 7).step_by(6) {
        let start_x = curve[i];
        let end_x = curve[i + 6];
        if x >= start_x && x <= end_x {
            segment_start_index = Some(i);
            break;
        }
    }

    let segment_start_index = segment_start_index?;
    let segment_slice = &curve[segment_start_index..segment_start_index + 8];
    let segment: BezierSegment = segment_slice.try_into().unwrap();

    for attempts in 0..10 {
        let tweak = 0.0001 * (attempts as f64);
        let adjusted_x = if x >= 1.0 { x - tweak } else { x + tweak };
        let mut t = get_t_at_x(&segment, adjusted_x);
        if t.is_none() {
            t = get_t_at_x_alternative(&segment, adjusted_x, 1e-6, 100);
        }
        if let Some(t_value) = t {
            let point = get_point_on_curve_at_t(&segment, t_value);
            let y = if point[1].abs() < 1e-15 {
                0.0
            } else {
                point[1]
            }; // If the y value is very close to 0, return 0
            return Some(to_external_coordinate(y, scale_y, offset_y));
        }
    }

    None
}

/// Converts a value from the scaled & offset coordinate space to the external coordinate space.
///
/// # Arguments
///
/// * `value` - The value to convert.
/// * `scale_y` - The y scale factor.
/// * `offset_y` - The y offset.
///
/// # Returns
///
/// The value in the external coordinate space.
fn to_external_coordinate(value: f64, scale_y: f64, offset_y: f64) -> f64 {
    // Add y offset (3 index) and multiply by y scale factor (1 index)
    let scaled = (value + offset_y) * scale_y;
    if scaled == -0.0 {
        0.0
    } else {
        scaled
    }
}

/// Returns the point on a cubic Bezier curve at a given t value.
///
/// # Arguments
///
/// * `segment` - The cubic Bezier segment to evaluate.
/// * `t` - The t value at which to evaluate the curve.
///
/// # Returns
///
/// The cartesian coordinates of the point on the curve at the given t value.
fn get_point_on_curve_at_t(segment: &BezierSegment, t: f64) -> [f64; 2] {
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let t2 = t * t;

    let a = mt2 * mt;
    let b = mt2 * t * 3.0;
    let c = mt * t2 * 3.0;
    let d = t * t2;

    let x = a * segment[0] + b * segment[2] + c * segment[4] + d * segment[6];
    let y = a * segment[1] + b * segment[3] + c * segment[5] + d * segment[7];

    [x, y]
}

/// Returns the derivative of a cubic Bezier curve at a given t value.
///
/// # Arguments
///
/// * `segment` - The cubic Bezier segment to evaluate.
/// * `t` - The t value at which to evaluate the derivative.
///
/// # Returns
///
/// The derivative of the curve at the given t value.
fn get_derivative_at_t(segment: &BezierSegment, t: f64) -> [f64; 2] {
    let mt = 1.0 - t;

    let a = -3.0 * mt * mt;
    let b = 3.0 * mt * (mt - 2.0 * t);
    let c = 3.0 * t * (2.0 * mt - t);
    let d = 3.0 * t * t;

    let x = a * segment[0] + b * segment[2] + c * segment[4] + d * segment[6];
    let y = a * segment[1] + b * segment[3] + c * segment[5] + d * segment[7];

    [x, y]
}

/// Returns the t value for a given x value on a cubic Bezier curve.
///
/// # Arguments
///
/// * `segment` - The cubic Bezier segment to evaluate.
/// * `x` - The x value for which we will find the associated t value along the curve.
///
/// # Returns
///
/// The t value for the given x value along the curve.
fn get_t_at_x(segment: &BezierSegment, x: f64) -> Option<f64> {
    let mut t = 0.5;
    let mut iteration_count = 0;

    loop {
        let point = get_point_on_curve_at_t(segment, t);
        let derivative = get_derivative_at_t(segment, t);

        let x_at_t = point[0];
        let x_derivative_at_t = derivative[0];
        let x_difference = x - x_at_t;

        if x_derivative_at_t.abs() > 1e-6 {
            t += x_difference / x_derivative_at_t;
        }

        t = t.clamp(0.0, 1.0);

        iteration_count += 1;

        if x_difference.abs() <= 1e-6 {
            return Some(t);
        }

        if iteration_count > 15 {
            return None;
        }
    }
}

/// Returns the t value for a given x value on a cubic Bezier curve.
/// An alternative method to get_t_at_x. It uses bisecting instead of Newton-Raphson.
/// It is slower but more reliable. It should be used as a fallback when Newton-Raphson fails to converge.
/// Reference implementation in chromium: https://chromium.googlesource.com/chromium/src/+/master/ui/gfx/geometry/cubic_bezier.cc
///
/// # Arguments
///
/// * `segment` - The cubic Bezier segment to evaluate.
/// * `x` - The x value for which we will find the associated t value along the curve.
/// * `tolerance` - The tolerance for the x value.
/// * `max_iterations` - The maximum number of iterations to perform.
///
/// # Returns
///
/// The t value for the given x value along the curve.
fn get_t_at_x_alternative(
    segment: &BezierSegment,
    x: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Option<f64> {
    let mut a = 0.0;
    let mut b = 1.0;
    let mut t;

    for _ in 0..max_iterations {
        t = (a + b) / 2.0;
        let x_at_t = get_point_on_curve_at_t(segment, t)[0];

        if (x_at_t - x).abs() <= tolerance {
            return Some(t);
        }

        let x_at_a = get_point_on_curve_at_t(segment, a)[0];

        if (x_at_t > x) != (x_at_a > x) {
            b = t;
        } else {
            a = t;
        }
    }

    None
}

/// Finds the y value for a given x value along a cubic Bezier curve.
///
/// # Arguments
///
/// * `encoded_chain` - The encoded cubic Bezier curve, outputted from `query-curve/builder`. First 4 elements are (scaleFactorX, scaleFactorY, offsetX, offsetY), rest are the points and handles.
/// * `scaled_x` - The x value for which we will find the associated y value along the curve. This should reflect the value in the scaled & offset coordinate space.
///
/// # Returns
///
/// The y value for the given x value along the curve.
pub fn query_encoded_curve(encoded_chain: &EncodedScaledBezierChain, scaled_x: f64) -> Option<f64> {
    match decode(encoded_chain) {
        Ok(chain) => query_curve(&chain, scaled_x),
        Err(_) => None,
    }
}

/// Returns a function that can be used to query the y value for a given x value along a cubic Bezier curve.
/// It is more efficient than `query_encoded_curve`, as it doesn't need to decode the chain every time. A given
/// curve is kept as a reference in the closure, however, so this is meant for repeated queries on the same curve.
///
/// # Arguments
///
/// * `encoded_chain` - The encoded cubic Bezier curve, outputted from `query-curve/builder`. First 4 elements are (scaleFactorX, scaleFactorY, offsetX, offsetY), rest are the points and handles.
///
/// # Returns
///
/// A function that can be used to query the y value for a given x value along a cubic Bezier curve.
pub fn get_encoded_curve_query_function(
  encoded_chain: &EncodedScaledBezierChain,
) -> Option<impl Fn(f64) -> Option<f64> + '_> {
    match decode(encoded_chain) {
        Ok(decoded_chain) => Some(move |scaled_x: f64| query_curve(&decoded_chain, scaled_x)),
        Err(_) => None,
    }
}

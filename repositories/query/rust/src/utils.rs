use crate::base62::from_base62;
use crate::constants::ENCODING_SCALE_FACTOR;
use crate::types::ScaledBezierChain;
use num_traits::ToPrimitive;
use regex::Regex;

pub fn decode(chain: &str) -> Result<ScaledBezierChain, String> {
    let re = Regex::new(r"--?[0-9A-Za-z]+").unwrap();
    let chain_with_leading_dash = format!("-{}", chain);
    let matches: Vec<&str> = re
        .find_iter(&chain_with_leading_dash)
        .map(|mat| mat.as_str())
        .collect();

    let mut result = Vec::new();

    for link in matches {
        let is_negative = link.starts_with("--");
        let number_str = link.trim_start_matches('-');
        let transformed = from_base62(number_str)?;
        let transformed_f64 = transformed
            .to_f64()
            .ok_or("Failed to convert BigUint to f64")?;
        let value = if is_negative {
            -transformed_f64
        } else {
            transformed_f64
        } / ENCODING_SCALE_FACTOR;
        result.push(value);
    }

    Ok(result)
}

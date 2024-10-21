[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ralusek/query-curve/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/query-curve.svg)](https://crates.io/crates/query-curve)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/ralusek/query-curve/blob/master/LICENSE)
[![Documentation](https://docs.rs/query-curve/badge.svg)](https://docs.rs/query-curve)
[![License](https://img.shields.io/crates/l/query-curve.svg)](https://github.com/ralusek/query-curve/blob/master/LICENSE)

# QueryCurve
This tool allows you to invoke queries against a curve you've laid out at [https://querycurve.com](https://querycurve.com/)

Once you have a curve in the shape you'd like:
![Example curve from QueryCurve.com](https://querycurve.com/example_d.png)

You'll get a resulting encoded curve that'll look like this:
`2BLnMW-2BLnMW--KyjA--KyjA-0-KyjA-CaR6-XZAG-KyjA-TN1E-KyjA-KyjA-KyjA-CaR6-TN1E-8OI4-fxSK-KyjA`

## Time to query!

### Installation
Add query-curve to your Cargo.toml dependencies:
```toml
[dependencies]
query-curve = "0.1.0"
```

Then run:
```bash
cargo build
```

### Usage

```rust
use query_curve::query_encoded_curve;

fn main() {
    let curve = "5SNUPI-8nlt2n2-0-0-0-fxSK-3yGp-fn3A-TzAp-e6zY-bau8-PAsC-dGxk-LXPh-f3xT-9cbF-fxSK-0";
    let x_value = 0.0;
    let result = query_encoded_curve(&curve.to_string(), x_value);

    match result {
        Some(y) => println!("At x = {}, y = {}", x_value, y),
        None => println!("Failed to find y for x = {}", x_value),
    }
}
```

#### Querying with a dynamically loaded curve
If you are pulling your curve from a db or otherwise need it to be dynamic:
```rust
use query_curve::query_encoded_curve;

fn main() {
    // Assume this was loaded from a database
    let dynamically_loaded_curve = "fxSK-fxSK-0-0-0-0-KyjA-0-KyjA-fxSK-fxSK-fxSK";
    let my_x_value = 0.35;

    // Gets the corresponding y value along the curve for a given x
    let result = query_encoded_curve(&dynamically_loaded_curve.to_string(), my_x_value);

    match result {
        Some(y) => println!("At x = {}, y = {}", my_x_value, y),
        None => println!("Failed to find y for x = {}", my_x_value),
    }
}
```
Note: While decoding the curve is fast, repeatedly querying against the same curve can be optimized by preloading the curve.
If you anticipate multiple queries against the same curve, consider using:

#### Querying with a preloaded or reused curve
If the curve you're using will be used to facilitate multiple queries, this alternative for querying will
bypass the need to decode the curve on every query.

```rust
use query_curve::get_encoded_curve_query_function;

fn main() {
    let fixed_curve = "fxSK-fxSK-0-0-0-0-KyjA-0-KyjA-fxSK-fxSK-fxSK";
    // Returns a function with a reference to the decoded curve
    let query_my_curve = get_encoded_curve_query_function(&fixed_curve.to_string()).unwrap();

    let x_values = [0.0, 0.5, 0.37];

    for &x in &x_values {
        match query_my_curve(x) {
            Some(y) => println!("At x = {}, y = {}", x, y),
            None => println!("Failed to find y for x = {}", x),
        }
    }
}
```

## Features

- Efficient Querying: Quickly find the y value for any given x on your custom Bezier curve.
- Preloading Curves: Optimize performance by preloading and reusing decoded curves for multiple queries.
- Integration with QueryCurve.com: Seamlessly use curves designed with QueryCurve.com.


## Documentation
Full documentation is available at docs.rs/query-curve.

## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

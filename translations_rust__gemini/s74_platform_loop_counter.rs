use std::io::{self, Read};
use std::os::raw::c_long;

fn safe_mul_clamped(a: c_long, b: c_long) -> c_long {
    let product = (a as f64) * (b as f64);
    if product > (c_long::MAX as f64) {
        return c_long::MAX;
    }
    if product < (c_long::MIN as f64) {
        return c_long::MIN;
    }
    a.wrapping_mul(b)
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut tokens = input.split_whitespace();
    let a: c_long = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => std::process::exit(1),
    };
    let b: c_long = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => std::process::exit(1),
    };

    println!("{}", safe_mul_clamped(a, b));
}

use std::io::{self, Read};
use std::os::raw::c_long;

fn safe_add_clamped(a: c_long, b: c_long) -> c_long {
    let sum = (a as f64) + (b as f64);
    if sum > (c_long::MAX as f64) {
        return c_long::MAX;
    }
    if sum < (c_long::MIN as f64) {
        return c_long::MIN;
    }
    a.wrapping_add(b)
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut words = input.split_whitespace();
    let a: c_long = match words.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => std::process::exit(1),
    };
    let b: c_long = match words.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => std::process::exit(1),
    };

    println!("{}", safe_add_clamped(a, b));
}

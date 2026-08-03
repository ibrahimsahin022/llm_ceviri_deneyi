use std::io::{self, Read};

const LONG_MAX: i32 = i32::MAX;
const LONG_MIN: i32 = i32::MIN;

fn safe_mul_clamped(a: i32, b: i32) -> i32 {
    let product = a as f64 * b as f64;
    if product > LONG_MAX as f64 {
        return LONG_MAX;
    }
    if product < LONG_MIN as f64 {
        return LONG_MIN;
    }
    a * b
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let b: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    println!("{}", safe_mul_clamped(a, b));
}

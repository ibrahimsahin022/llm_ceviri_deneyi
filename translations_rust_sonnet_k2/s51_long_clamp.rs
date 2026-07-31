use std::io::{self, Read};

fn safe_add_clamped(a: i32, b: i32) -> i32 {
    let sum: f64 = a as f64 + b as f64;
    if sum > i32::MAX as f64 {
        return i32::MAX;
    }
    if sum < i32::MIN as f64 {
        return i32::MIN;
    }
    a + b
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let a: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let b: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    println!("{}", safe_add_clamped(a, b));
}

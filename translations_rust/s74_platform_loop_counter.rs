use std::io::{self, Read};

fn safe_mul_clamped(a: i64, b: i64) -> i64 {
    let product = a as f64 * b as f64;
    if product > i64::MAX as f64 {
        return i64::MAX;
    }
    if product < i64::MIN as f64 {
        return i64::MIN;
    }
    a * b
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    println!("{}", safe_mul_clamped(a, b));
}

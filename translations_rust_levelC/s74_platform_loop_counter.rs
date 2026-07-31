use std::io::{self, Read};

fn safe_mul_clamped(a: i32, b: i32) -> i32 {
    let product = a as f64 * b as f64;
    if product > i32::MAX as f64 {
        return i32::MAX;
    }
    if product < i32::MIN as f64 {
        return i32::MIN;
    }
    a * b
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    println!("{}", safe_mul_clamped(a, b));
}

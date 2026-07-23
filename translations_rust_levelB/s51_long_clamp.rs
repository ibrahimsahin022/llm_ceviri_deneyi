use std::io::{self, Read};

fn safe_add_clamped(a: i64, b: i64) -> i64 {
    let sum = a as f64 + b as f64;
    if sum > i64::MAX as f64 {
        return i64::MAX;
    }
    if sum < i64::MIN as f64 {
        return i64::MIN;
    }
    a + b
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    println!("{}", safe_add_clamped(a, b));
}

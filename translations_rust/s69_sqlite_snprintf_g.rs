use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a: f64 = it.next().unwrap().parse().unwrap();
    let b: f64 = it.next().unwrap().parse().unwrap();
    let ratio = a / b;
    println!("{}", ratio);
}

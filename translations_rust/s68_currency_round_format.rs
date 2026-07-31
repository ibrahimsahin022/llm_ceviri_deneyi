use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let price: f64 = it.next().unwrap().parse().unwrap();
    let qty: i32 = it.next().unwrap().parse().unwrap();
    let total = price * qty as f64;
    println!("{}", total);
}

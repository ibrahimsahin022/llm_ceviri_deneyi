use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let x: f32 = input.trim().parse().unwrap();
    println!("{}", x.to_bits());
}

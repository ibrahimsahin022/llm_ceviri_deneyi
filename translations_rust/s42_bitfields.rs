use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let raw: u32 = input.trim().parse().unwrap();
    let a = raw & 0x1;
    let b = raw & 0x7;
    let c = raw & 0xF;
    println!("{} {} {}", a, b, c);
}

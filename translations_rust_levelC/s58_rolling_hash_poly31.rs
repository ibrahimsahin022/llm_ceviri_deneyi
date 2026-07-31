use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut h: u32 = 0;
    for b in line.bytes() {
        h = h.wrapping_mul(31).wrapping_add(b as u32);
    }
    println!("{}", h);
}

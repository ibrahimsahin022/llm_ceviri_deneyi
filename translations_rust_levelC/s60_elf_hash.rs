use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut h: u32 = 0;
    for b in line.bytes() {
        h = (h << 4) + b as u32;
        let g = h & 0xF0000000;
        if g != 0 {
            h ^= g >> 24;
        }
        h &= !g;
    }
    println!("{}", h);
}

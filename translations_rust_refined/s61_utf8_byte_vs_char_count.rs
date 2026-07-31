use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut bytes = 0;
    let mut chars = 0;
    for b in line.bytes() {
        bytes += 1;
        if (b & 0xC0) != 0x80 {
            chars += 1;
        }
    }
    println!("{} {}", bytes, chars);
}

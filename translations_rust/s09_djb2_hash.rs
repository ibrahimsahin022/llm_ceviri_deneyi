use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut hash: u32 = 5381;
    for b in line.bytes() {
        hash = hash * 33 + b as u32;
    }
    println!("{}", hash);
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    for (idx, tok) in line.split(' ').filter(|s| !s.is_empty()).enumerate() {
        println!("{}: {}", idx, tok);
    }
}

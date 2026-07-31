use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let reversed: String = line.chars().rev().collect();
    if line == reversed {
        println!("EVET");
    } else {
        println!("HAYIR");
    }
}

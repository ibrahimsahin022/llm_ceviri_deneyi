use std::io::{self, Read};

const COLOR_NAMES: [&str; 5] = ["red", "green", "blue", "yellow", "black"];

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let word = input.split_whitespace().next().unwrap_or("");
    let idx = COLOR_NAMES.iter().position(|&s| s == word);
    match idx {
        Some(i) => println!("{}", i),
        None => println!("-1"),
    }
}

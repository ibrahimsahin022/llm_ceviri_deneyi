use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut alpha = 0;
    let mut digit = 0;
    for ch in line.chars() {
        if ch.is_alphabetic() {
            alpha += 1;
        }
        if ch.is_ascii_digit() {
            digit += 1;
        }
    }
    println!("{} {}", alpha, digit);
}

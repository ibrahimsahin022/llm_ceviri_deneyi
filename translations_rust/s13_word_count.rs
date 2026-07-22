use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut chars = 0;
    let mut words = 0;
    let mut inword = false;
    for c in input.chars() {
        chars += 1;
        if c == ' ' || c == '\n' || c == '\t' || c == '\r' {
            inword = false;
        } else if !inword {
            words += 1;
            inword = true;
        }
    }
    println!("{} {}", words, chars);
}

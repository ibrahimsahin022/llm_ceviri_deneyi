use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut alpha = 0;
    let mut digit = 0;
    for b in line.bytes() {
        let c = b as i8;
        if c >= 0 {
            let a = c as u8;
            if a.is_ascii_alphabetic() {
                alpha += 1;
            }
            if a.is_ascii_digit() {
                digit += 1;
            }
        }
    }
    println!("{} {}", alpha, digit);
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let k: i32 = lines.next().unwrap().trim().parse().unwrap();
    let k = ((k % 26) + 26) % 26;
    let text = lines.next().unwrap_or("");
    let mut out = String::new();
    for c in text.chars() {
        if c >= 'a' && c <= 'z' {
            let shifted = (c as u8 - b'a' + k as u8) % 26 + b'a';
            out.push(shifted as char);
        } else if c >= 'A' && c <= 'Z' {
            let shifted = (c as u8 - b'A' + k as u8) % 26 + b'A';
            out.push(shifted as char);
        } else {
            out.push(c);
        }
    }
    println!("{}", out);
}

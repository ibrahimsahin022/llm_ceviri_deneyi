use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let buf = input.split_whitespace().next().unwrap_or("");
    let mut v: u64 = 0;
    for c in buf.chars() {
        if !c.is_ascii_digit() {
            break;
        }
        v = v * 10 + (c as u64 - '0' as u64);
    }
    println!("{}", v);
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let buf = input.split_whitespace().next().unwrap_or("");
    let mut v: u32 = 0;
    for c in buf.chars() {
        if !c.is_ascii_digit() {
            break;
        }
        v = v.wrapping_mul(10).wrapping_add(c as u32 - '0' as u32);
    }
    println!("{}", v);
}

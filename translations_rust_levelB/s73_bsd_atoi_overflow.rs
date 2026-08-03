use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let buf = match input.split_whitespace().next() {
        Some(s) => s,
        None => return,
    };
    let mut v: u32 = 0;
    for b in buf.bytes() {
        if b < b'0' || b > b'9' {
            break;
        }
        v = v.wrapping_mul(10).wrapping_add((b - b'0') as u32);
    }
    println!("{}", v);
}

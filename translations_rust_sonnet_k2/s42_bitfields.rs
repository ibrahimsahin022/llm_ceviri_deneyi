use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let raw: u32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let a = raw & 0x1;
    let b = raw & 0x7;
    let c = raw & 0xF;

    println!("{} {} {}", a, b, c);
}

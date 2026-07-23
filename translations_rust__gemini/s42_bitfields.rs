use std::io::{self, Read};

struct Flags {
    a: u32,
    b: u32,
    c: u32,
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_whitespace();
    let token = match iter.next() {
        Some(t) => t,
        None => return,
    };

    let raw = match token.parse::<i64>() {
        Ok(v) => v as u32,
        Err(_) => return,
    };

    let f = Flags {
        a: raw & 0x1,
        b: raw & 0x7,
        c: raw & 0xF,
    };

    println!("{} {} {}", f.a, f.b, f.c);
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut f: u64 = 1;
    let mut i: i32 = 2;
    while i <= n {
        f = f.wrapping_mul(i as u64);
        i += 1;
    }

    println!("{}", f);
}

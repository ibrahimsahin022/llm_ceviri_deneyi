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

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let t = a.wrapping_add(b);
        a = b;
        b = t;
    }

    println!("{}", a);
}

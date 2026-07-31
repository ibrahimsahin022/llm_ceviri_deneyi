use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let mut n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let vals = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let syms = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];

    let mut out = String::new();
    for i in 0..13 {
        while n >= vals[i] {
            out.push_str(syms[i]);
            n -= vals[i];
        }
    }
    out.push('\n');

    let stdout = io::stdout();
    let mut o = stdout.lock();
    let _ = o.write_all(out.as_bytes());
}

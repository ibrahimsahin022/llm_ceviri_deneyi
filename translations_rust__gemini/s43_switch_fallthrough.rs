use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();
    let level: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let bonus = match level {
        4 => 8 + 4 + 2 + 1,
        3 => 4 + 2 + 1,
        2 => 2 + 1,
        1 => 1,
        _ => 0,
    };

    println!("{}", bonus);
}

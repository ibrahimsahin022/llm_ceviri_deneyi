use std::io::{self, Read};
use std::process;

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }

    let bracket: i32 = match input.split_whitespace().next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => process::exit(1),
    };

    let tax = match bracket {
        4 => 800 + 400 + 200 + 100,
        3 => 400 + 200 + 100,
        2 => 200 + 100,
        1 => 100,
        _ => 0,
    };

    println!("{}", tax);
}

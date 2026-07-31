use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let level: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut bonus: i32 = 0;
    match level {
        4 => {
            bonus += 8;
            bonus += 4;
            bonus += 2;
            bonus += 1;
        }
        3 => {
            bonus += 4;
            bonus += 2;
            bonus += 1;
        }
        2 => {
            bonus += 2;
            bonus += 1;
        }
        1 => {
            bonus += 1;
        }
        _ => {
            bonus = 0;
        }
    }

    println!("{}", bonus);
}

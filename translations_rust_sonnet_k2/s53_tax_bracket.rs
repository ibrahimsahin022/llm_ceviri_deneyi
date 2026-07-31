use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let bracket: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut tax: i32 = 0;
    match bracket {
        4 => {
            tax += 800;
            tax += 400;
            tax += 200;
            tax += 100;
        }
        3 => {
            tax += 400;
            tax += 200;
            tax += 100;
        }
        2 => {
            tax += 200;
            tax += 100;
        }
        1 => {
            tax += 100;
        }
        _ => {
            tax = 0;
        }
    }

    println!("{}", tax);
}

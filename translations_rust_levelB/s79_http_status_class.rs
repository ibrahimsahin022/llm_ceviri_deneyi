use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let code: i32 = match input.split_whitespace().next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let tier = code / 100;
    let mut score = 0;
    match tier {
        2..=5 => {
            if tier >= 5 {
                score += 8;
            }
            if tier >= 4 {
                score += 4;
            }
            if tier >= 3 {
                score += 2;
            }
            score += 1;
        }
        _ => {
            score = 0;
        }
    }
    println!("{}", score);
}

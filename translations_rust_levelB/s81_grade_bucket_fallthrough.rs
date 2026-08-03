use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grade: i32 = match input.split_whitespace().next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let tier = grade / 10;
    let mut badges = 0;
    match tier {
        6..=10 => {
            if tier >= 9 {
                badges += 1;
            }
            if tier >= 8 {
                badges += 1;
            }
            if tier >= 7 {
                badges += 1;
            }
            badges += 1;
        }
        _ => {
            badges = 0;
        }
    }
    println!("{}", badges);
}

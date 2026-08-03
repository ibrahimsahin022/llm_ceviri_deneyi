use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let target: i32 = match input.split_whitespace().next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let mut steps = 0;
    match target {
        0..=3 => {
            if target >= 3 {
                steps += 1;
            }
            if target >= 2 {
                steps += 1;
            }
            if target >= 1 {
                steps += 1;
            }
            steps += 1;
        }
        _ => {
            steps = -1;
        }
    }
    println!("{}", steps);
}

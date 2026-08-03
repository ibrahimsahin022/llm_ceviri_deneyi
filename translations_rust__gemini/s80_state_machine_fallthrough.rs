use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        if let Some(word) = input.split_whitespace().next() {
            if let Ok(target) = word.parse::<i32>() {
                let steps = match target {
                    3 => 4,
                    2 => 3,
                    1 => 2,
                    0 => 1,
                    _ => -1,
                };
                println!("{}", steps);
            }
        }
    }
}

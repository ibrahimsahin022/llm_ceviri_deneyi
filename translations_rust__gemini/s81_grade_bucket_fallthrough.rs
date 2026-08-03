use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut words = input.split_whitespace();
    if let Some(word) = words.next() {
        if let Ok(grade) = word.parse::<i32>() {
            let tier = grade / 10;
            let badges = match tier {
                9 | 10 => 4,
                8 => 3,
                7 => 2,
                6 => 1,
                _ => 0,
            };
            println!("{}", badges);
        }
    }
}

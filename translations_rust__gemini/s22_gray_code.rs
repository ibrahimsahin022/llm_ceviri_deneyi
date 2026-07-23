use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        if let Some(token) = input.split_whitespace().next() {
            if let Ok(n) = token.parse::<u32>() {
                println!("{}", n ^ (n >> 1));
            }
        }
    }
}

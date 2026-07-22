use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        if let Some(token) = input.split_whitespace().next() {
            if let Ok(n) = token.parse::<i32>() {
                let mut f: u64 = 1;
                for i in 2..=n {
                    f *= i as u64;
                }
                println!("{}", f);
            }
        }
    }
}

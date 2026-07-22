use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    if io::stdin().read_to_string(&mut buffer).is_ok() {
        if let Some(token) = buffer.split_whitespace().next() {
            if let Ok(n) = token.parse::<i32>() {
                let mut a: u64 = 0;
                let mut b: u64 = 1;
                for _ in 0..n {
                    let t = a.wrapping_add(b);
                    a = b;
                    b = t;
                }
                println!("{}", a);
            }
        }
    }
}

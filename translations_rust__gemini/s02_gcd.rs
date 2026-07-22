use std::io::{self, Read};

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    if a < 0 {
        -a
    } else {
        a
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        let mut iter = input.split_whitespace();
        if let (Some(a_str), Some(b_str)) = (iter.next(), iter.next()) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                println!("{}", gcd(a, b));
            }
        }
    }
}

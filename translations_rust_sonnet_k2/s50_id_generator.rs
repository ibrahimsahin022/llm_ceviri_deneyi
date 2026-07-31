use std::io::{self, Read};

const BASLANGIC_ID: i32 = 1000;

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut counter = BASLANGIC_ID;
    for _ in 0..n {
        println!("{}", counter);
        counter += 1;
    }
}

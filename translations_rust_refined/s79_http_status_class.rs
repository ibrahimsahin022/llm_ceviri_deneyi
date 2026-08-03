use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let code: i32 = input.trim().parse().unwrap();
    let tier = code / 100;
    let score = match tier {
        5 => 15,
        4 => 7,
        3 => 3,
        2 => 1,
        _ => 0,
    };
    println!("{}", score);
}

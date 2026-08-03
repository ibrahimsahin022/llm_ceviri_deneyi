use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grade: i32 = input.trim().parse().unwrap();
    let tier = grade / 10;
    let badges = match tier {
        10 | 9 => 1,
        8 => 1,
        7 => 1,
        6 => 1,
        _ => 0,
    };
    println!("{}", badges);
}

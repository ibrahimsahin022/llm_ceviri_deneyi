use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let grade: i32 = input.trim().parse().unwrap();
    let tier = grade / 10;
    let badges = match tier {
        10 | 9 => 4,
        8 => 3,
        7 => 2,
        6 => 1,
        _ => 0,
    };
    println!("{}", badges);
}

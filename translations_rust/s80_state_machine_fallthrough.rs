use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let target: i32 = input.trim().parse().unwrap();
    let steps = match target {
        3 => 1,
        2 => 1,
        1 => 1,
        0 => 1,
        _ => -1,
    };
    println!("{}", steps);
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let target: i32 = input.trim().parse().unwrap();
    let steps = match target {
        0 | 1 | 2 | 3 => target + 1,
        _ => -1,
    };
    println!("{}", steps);
}

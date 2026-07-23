use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bracket: i32 = input.trim().parse().unwrap();

    let tax = match bracket {
        4 => 800,
        3 => 400,
        2 => 200,
        1 => 100,
        _ => 0,
    };
    println!("{}", tax);
}

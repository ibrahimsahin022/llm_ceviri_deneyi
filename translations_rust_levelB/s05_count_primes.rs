use std::io::{self, Read};

fn is_prime(x: i32) -> bool {
    if x < 2 { return false; }
    let mut d: i64 = 2;
    while d * d <= x as i64 {
        if x % (d as i32) == 0 { return false; }
        d += 1;
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut count = 0;
    for i in 2..=n {
        if is_prime(i) { count += 1; }
    }
    println!("{}", count);
}

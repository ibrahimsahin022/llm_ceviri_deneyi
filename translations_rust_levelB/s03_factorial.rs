use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().split_whitespace().next().unwrap().parse().unwrap();
    let mut f: u64 = 1;
    let mut i = 2;
    while i <= n {
        f *= i as u64;
        i += 1;
    }
    println!("{}", f);
}

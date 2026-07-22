use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let t = a + b;
        a = b;
        b = t;
    }
    println!("{}", a);
}

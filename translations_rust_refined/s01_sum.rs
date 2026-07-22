use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut sum: i64 = 0;
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        sum += x;
    }
    println!("{}", sum);
}

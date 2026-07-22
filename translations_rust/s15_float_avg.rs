use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut sum = 0.0f64;
    for _ in 0..n {
        let x: f64 = it.next().unwrap().parse().unwrap();
        sum += x;
    }
    let avg = if n > 0 { sum / n as f64 } else { 0.0 };
    println!("{}", avg);
}

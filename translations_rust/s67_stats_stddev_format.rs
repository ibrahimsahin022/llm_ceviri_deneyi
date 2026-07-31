use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut vals = Vec::with_capacity(n);
    let mut sum = 0.0;
    for _ in 0..n {
        let v: f64 = it.next().unwrap().parse().unwrap();
        vals.push(v);
        sum += v;
    }
    let mean = sum / n as f64;
    let mut sq = 0.0;
    for v in &vals {
        let d = v - mean;
        sq += d * d;
    }
    let variance = sq / n as f64;
    let stddev = variance.sqrt();
    println!("{} {}", mean, stddev);
}

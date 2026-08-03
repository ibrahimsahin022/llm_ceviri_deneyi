use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i32 = it.next().unwrap().parse().unwrap();
        arr.push(v);
    }
    let k: usize = it.next().unwrap().parse().unwrap();
    let start = n.saturating_sub(k);
    for i in start..n {
        print!("{} ", arr[i]);
    }
    println!();
}

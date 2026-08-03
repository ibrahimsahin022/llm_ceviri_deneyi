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
    let mut i: usize = 0;
    let tmp = arr[i];
    i += 1;
    arr[i] = arr[1];
    i += 1;
    arr[1] = tmp;
    for v in &arr {
        print!("{} ", v);
    }
    println!();
    println!("i={}", i);
}

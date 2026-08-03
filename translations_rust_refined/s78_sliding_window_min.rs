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
    let mut count = 0;
    for i in 0..n {
        if i > 0 && i + 1 < n {
            let prev_idx = i - 1;
            let next_idx = i + 1;
            if arr[i] < arr[prev_idx] && arr[i] < arr[next_idx] {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

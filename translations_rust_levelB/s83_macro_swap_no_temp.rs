use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    if n < 2 || n > 100 {
        return;
    }
    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let v: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        arr.push(v);
    }

    // SWAP(arr[i++], arr[1], tmp) -> tmp = arr[i++]; arr[i++] = arr[1]; arr[1] = tmp;
    // 'a' makro govdesinde iki kez gectigi icin i IKI KEZ artar.
    let mut i: usize = 0;
    let tmp = arr[i];
    i += 1;
    let idx = i;
    i += 1;
    arr[idx] = arr[1];
    arr[1] = tmp;

    for v in &arr {
        print!("{} ", v);
    }
    println!();
    println!("i={}", i);
}

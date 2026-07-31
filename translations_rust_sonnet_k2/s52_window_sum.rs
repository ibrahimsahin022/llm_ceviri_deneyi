use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let k: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    if n < 0 || k < 0 {
        return;
    }

    let mut arr = vec![0i32; n.max(0) as usize];
    for i in 0..n {
        let v: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        arr[i as usize] = v;
    }

    let mut start = n - k;
    if start < 0 {
        start = 0;
    }

    let mut sum: i64 = 0;
    for i in start..n {
        sum += arr[i as usize] as i64;
    }

    println!("{}", sum);
}

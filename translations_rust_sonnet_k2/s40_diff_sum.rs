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

    let mut arr = [0i32; 1000];
    for i in 0..n {
        let v: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        arr[i as usize] = v;
    }

    let mut diffsum: i32 = 0;
    for i in 0..(n - 1) {
        diffsum += arr[(i + 1) as usize] - arr[i as usize];
    }

    println!("{}", diffsum);
}

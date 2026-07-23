use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut arr = [0i32; 1000];
    for i in 0..n {
        let val: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        if i >= 0 && (i as usize) < 1000 {
            arr[i as usize] = val;
        }
    }

    let mut diffsum: i32 = 0;
    let mut i: i32 = 0;
    while i < n - 1 {
        let idx1 = i as usize;
        let idx2 = (i + 1) as usize;
        if idx2 < 1000 {
            diffsum = diffsum.wrapping_add(arr[idx2].wrapping_sub(arr[idx1]));
        }
        i += 1;
    }

    println!("{}", diffsum);
}

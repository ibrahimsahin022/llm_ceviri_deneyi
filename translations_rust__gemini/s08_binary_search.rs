use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let mut n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut a = [0i32; 1000];
    if n > 1000 {
        n = 1000;
    }

    for i in 0..n {
        if let Some(s) = tokens.next() {
            if let Ok(val) = s.parse::<i32>() {
                a[i as usize] = val;
            }
        }
    }

    let target: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut lo: i32 = 0;
    let mut hi: i32 = n - 1;
    let mut ans: i32 = -1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let val = a[mid as usize];
        if val == target {
            ans = mid;
            break;
        } else if val < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    println!("{}", ans);
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let mut n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut a = [0i32; 1000];
    if n > 1000 {
        n = 1000;
    }
    for i in 0..n {
        let v: i32 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        a[i as usize] = v;
    }

    let target: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut lo = 0i32;
    let mut hi = n - 1;
    let mut ans = -1i32;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if a[mid as usize] == target {
            ans = mid;
            break;
        } else if a[mid as usize] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    println!("{}", ans);
}

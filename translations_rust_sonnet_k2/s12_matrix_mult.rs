use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let r1: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let c1: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut a = vec![vec![0i64; 64]; 64];
    let mut b = vec![vec![0i64; 64]; 64];
    let mut c = vec![vec![0i64; 64]; 64];

    for i in 0..r1 {
        for j in 0..c1 {
            let v: i64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            a[i as usize][j as usize] = v;
        }
    }

    let r2: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let c2: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    for i in 0..r2 {
        for j in 0..c2 {
            let v: i64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            b[i as usize][j as usize] = v;
        }
    }

    for i in 0..r1 {
        for j in 0..c2 {
            let mut s: i64 = 0;
            for k in 0..c1 {
                s += a[i as usize][k as usize] * b[k as usize][j as usize];
            }
            c[i as usize][j as usize] = s;
        }
    }

    for i in 0..r1 {
        for j in 0..c2 {
            print!("{}", c[i as usize][j as usize]);
            if j < c2 - 1 {
                print!(" ");
            }
        }
        println!();
    }
}

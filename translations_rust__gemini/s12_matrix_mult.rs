use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let r1: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let c1: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut a = [[0i64; 64]; 64];
    for i in 0..r1 {
        for j in 0..c1 {
            if let Some(tok) = tokens.next() {
                if let Ok(val) = tok.parse::<i64>() {
                    if i >= 0 && i < 64 && j >= 0 && j < 64 {
                        a[i as usize][j as usize] = val;
                    }
                }
            }
        }
    }

    let r2: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let c2: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut b = [[0i64; 64]; 64];
    for i in 0..r2 {
        for j in 0..c2 {
            if let Some(tok) = tokens.next() {
                if let Ok(val) = tok.parse::<i64>() {
                    if i >= 0 && i < 64 && j >= 0 && j < 64 {
                        b[i as usize][j as usize] = val;
                    }
                }
            }
        }
    }

    let mut c = [[0i64; 64]; 64];
    for i in 0..r1 {
        for j in 0..c2 {
            let mut s: i64 = 0;
            for k in 0..c1 {
                if i >= 0 && i < 64 && j >= 0 && j < 64 && k >= 0 && k < 64 {
                    s = s.wrapping_add(a[i as usize][k as usize].wrapping_mul(b[k as usize][j as usize]));
                }
            }
            if i >= 0 && i < 64 && j >= 0 && j < 64 {
                c[i as usize][j as usize] = s;
            }
        }
    }

    for i in 0..r1 {
        for j in 0..c2 {
            if i >= 0 && i < 64 && j >= 0 && j < 64 {
                print!("{}", c[i as usize][j as usize]);
            }
            if j < c2 - 1 {
                print!(" ");
            }
        }
        println!();
    }
}

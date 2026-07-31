use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let r: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let c: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut m = vec![vec![0i64; 64]; 64];
    for i in 0..(r as usize) {
        for j in 0..(c as usize) {
            let v: i64 = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            m[i][j] = v;
        }
    }

    for j in 0..c {
        for i in 0..r {
            print!("{}", m[i as usize][j as usize]);
            if i < r - 1 {
                print!(" ");
            }
        }
        println!();
    }
}

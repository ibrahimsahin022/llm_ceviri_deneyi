use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let r: usize = match tokens.next().and_then(|s| s.parse::<isize>().ok()) {
        Some(v) if v >= 0 => v as usize,
        _ => return,
    };
    let c: usize = match tokens.next().and_then(|s| s.parse::<isize>().ok()) {
        Some(v) if v >= 0 => v as usize,
        _ => return,
    };

    let mut m = vec![vec![0i64; c]; r];
    for i in 0..r {
        for j in 0..c {
            if let Some(tok) = tokens.next() {
                if let Ok(val) = tok.parse::<i64>() {
                    m[i][j] = val;
                }
            }
        }
    }

    for j in 0..c {
        for i in 0..r {
            print!("{}", m[i][j]);
            if i < r - 1 {
                print!(" ");
            }
        }
        println!();
    }
}

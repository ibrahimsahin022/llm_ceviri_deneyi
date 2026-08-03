use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();

    let n: usize = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) if val > 0 && val <= 10 => val,
        _ => return,
    };

    let mut a = [[0.0f64; 20]; 10];
    for i in 0..n {
        for j in 0..n {
            a[i][j] = match tokens.next().and_then(|s| s.parse().ok()) {
                Some(val) => val,
                None => return,
            };
        }
        for j in 0..n {
            a[i][n + j] = if i == j { 1.0 } else { 0.0 };
        }
    }

    for col in 0..n {
        let pivot = a[col][col];
        if pivot.abs() < 1e-9 {
            println!("SINGULAR");
            return;
        }
        for j in 0..2 * n {
            a[col][j] /= pivot;
        }
        let pivot_row = a[col];
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = a[row][col];
            for j in 0..2 * n {
                a[row][j] -= factor * pivot_row[j];
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{:.4} ", a[i][n + j]);
        }
        println!();
    }
}

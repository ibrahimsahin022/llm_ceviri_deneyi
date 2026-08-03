use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut a = vec![vec![0.0f64; 2 * n]; n];
    for i in 0..n {
        for j in 0..n {
            a[i][j] = it.next().unwrap().parse().unwrap();
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
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = a[row][col];
            for j in 0..2 * n {
                a[row][j] -= factor * a[col][j];
            }
        }
    }

    for i in 0..n {
        let mut line = String::new();
        for j in 0..n {
            line.push_str(&format!("{:.4} ", a[i][n + j]));
        }
        println!("{}", line);
    }
}

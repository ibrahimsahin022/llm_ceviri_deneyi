use std::io::{self, BufRead};

fn det(m: &[[i64; 8]; 8], n: usize) -> i64 {
    if n == 1 {
        return m[0][0];
    }
    if n == 2 {
        return m[0][0] * m[1][1] - m[0][1] * m[1][0];
    }

    let mut result: i64 = 0;
    for col in 0..n {
        let mut sub = [[0i64; 8]; 8];
        for i in 1..n {
            let mut subcol = 0;
            for j in 0..n {
                if j == col {
                    continue;
                }
                sub[i - 1][subcol] = m[i][j];
                subcol += 1;
            }
        }
        let sign: i64 = if col % 2 == 0 { 1 } else { -1 };
        result += sign * m[0][col] * det(&sub, n - 1);
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut m = [[0i64; 8]; 8];
    for i in 0..n {
        for j in 0..n {
            if let Some(Ok(line)) = lines.next() {
                if let Ok(val) = line.trim().parse::<i64>() {
                    m[i][j] = val;
                }
            }
        }
    }

    println!("{}", det(&m, n));
}

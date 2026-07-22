use std::io::{self, Read};

fn det(m: &[[i64; 8]; 8], n: usize) -> i64 {
    if n == 1 {
        return m[0][0];
    }
    if n == 2 {
        return m[0][0] * m[1][1] - m[0][1] * m[1][0];
    }
    let mut result: i64 = 0;
    let mut sub = [[0i64; 8]; 8];
    for col in 0..n {
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
        let sign = if col % 2 == 0 { 1 } else { -1 };
        result += sign * m[0][col] * det(&sub, n - 1);
    }
    result
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();
    let n: usize = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut m = [[0i64; 8]; 8];
    for i in 0..n {
        for j in 0..n {
            if let Some(val) = tokens.next().and_then(|s| s.parse().ok()) {
                m[i][j] = val;
            }
        }
    }

    println!("{}", det(&m, n));
}

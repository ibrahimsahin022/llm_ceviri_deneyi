use std::io::{self, Read};

fn det(m: &Vec<Vec<i64>>, n: usize) -> i64 {
    if n == 1 {
        return m[0][0];
    }
    if n == 2 {
        return m[0][0] * m[1][1] - m[0][1] * m[1][0];
    }
    let mut result = 0i64;
    for col in 0..n {
        let mut sub = vec![vec![0i64; n - 1]; n - 1];
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut m = vec![vec![0i64; n]; n];
    for i in 0..n {
        for j in 0..n {
            m[i][j] = it.next().unwrap().parse().unwrap();
        }
    }
    println!("{}", det(&m, n));
}

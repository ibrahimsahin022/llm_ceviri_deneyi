use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let r1: usize = it.next().unwrap().parse().unwrap();
    let c1: usize = it.next().unwrap().parse().unwrap();
    let mut a = vec![vec![0i64; c1]; r1];
    for i in 0..r1 {
        for j in 0..c1 {
            a[i][j] = it.next().unwrap().parse().unwrap();
        }
    }
    let r2: usize = it.next().unwrap().parse().unwrap();
    let c2: usize = it.next().unwrap().parse().unwrap();
    let mut b = vec![vec![0i64; c2]; r2];
    for i in 0..r2 {
        for j in 0..c2 {
            b[i][j] = it.next().unwrap().parse().unwrap();
        }
    }
    let mut c = vec![vec![0i64; c2]; r1];
    for i in 0..r1 {
        for j in 0..c2 {
            let mut s: i64 = 0;
            for k in 0..c1 {
                s += a[i][k] * b[k][j];
            }
            c[i][j] = s;
        }
    }
    let mut out = String::new();
    for i in 0..r1 {
        let row: Vec<String> = (0..c2).map(|j| c[i][j].to_string()).collect();
        out.push_str(&row.join(" "));
        out.push('\n');
    }
    print!("{}", out);
}

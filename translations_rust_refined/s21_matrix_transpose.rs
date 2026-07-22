use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let r: usize = it.next().unwrap().parse().unwrap();
    let c: usize = it.next().unwrap().parse().unwrap();
    let mut m = vec![vec![0i64; c]; r];
    for i in 0..r {
        for j in 0..c {
            m[i][j] = it.next().unwrap().parse().unwrap();
        }
    }
    let mut out = String::new();
    for j in 0..c {
        let row: Vec<String> = (0..r).map(|i| m[i][j].to_string()).collect();
        out.push_str(&row.join(" "));
        out.push('\n');
    }
    print!("{}", out);
}

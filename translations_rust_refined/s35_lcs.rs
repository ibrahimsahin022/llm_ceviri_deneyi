use std::io::{self, Read};

fn lcs(a: &[u8], b: &[u8]) -> (i32, String) {
    let n = a.len();
    let m = b.len();
    let mut c = vec![vec![0i32; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                c[i][j] = c[i - 1][j - 1] + 1;
            } else {
                c[i][j] = std::cmp::max(c[i - 1][j], c[i][j - 1]);
            }
        }
    }

    let t = c[n][m];
    let mut s = vec![0u8; t as usize];
    let mut i = n;
    let mut j = m;
    let mut k = t as isize - 1;

    while k >= 0 {
        if a[i - 1] == b[j - 1] {
            s[k as usize] = a[i - 1];
            i -= 1;
            j -= 1;
            k -= 1;
        } else if c[i][j - 1] > c[i - 1][j] {
            j -= 1;
        } else {
            i -= 1;
        }
    }

    (t, String::from_utf8(s).unwrap())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let a = lines.next().unwrap_or("").trim();
    let b = lines.next().unwrap_or("").trim();

    let (len, s) = lcs(a.as_bytes(), b.as_bytes());
    println!("len={} lcs={}", len, s);
}

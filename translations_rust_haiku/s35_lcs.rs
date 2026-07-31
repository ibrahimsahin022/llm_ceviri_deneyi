use std::io::{self, BufRead};

fn lcs(a: &str, b: &str) -> (i32, String) {
    let n = a.len();
    let m = b.len();
    let mut c = vec![vec![0; m + 1]; n + 1];

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    for i in 1..=n {
        for j in 1..=m {
            if a_bytes[i - 1] == b_bytes[j - 1] {
                c[i][j] = c[i - 1][j - 1] + 1;
            } else {
                c[i][j] = if c[i - 1][j] > c[i][j - 1] {
                    c[i - 1][j]
                } else {
                    c[i][j - 1]
                };
            }
        }
    }

    let t = c[n][m];
    let mut s = String::with_capacity(t as usize);

    let mut i = n;
    let mut j = m;
    while i > 0 && j > 0 {
        if a_bytes[i - 1] == b_bytes[j - 1] {
            s.insert(0, a_bytes[i - 1] as char);
            i -= 1;
            j -= 1;
        } else if c[i][j - 1] > c[i - 1][j] {
            j -= 1;
        } else {
            i -= 1;
        }
    }

    (t as i32, s)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(mut a)) = lines.next() {
        while a.ends_with('\n') || a.ends_with('\r') {
            a.pop();
        }

        if let Some(Ok(mut b)) = lines.next() {
            while b.ends_with('\n') || b.ends_with('\r') {
                b.pop();
            }

            let (len, lcs_str) = lcs(&a, &b);
            println!("len={} lcs={}", len, lcs_str);
        }
    }
}

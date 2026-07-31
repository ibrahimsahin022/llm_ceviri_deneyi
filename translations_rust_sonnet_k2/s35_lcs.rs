use std::io::{self, Read};

fn lcs(a: &[u8], n: usize, b: &[u8], m: usize) -> (i32, Vec<u8>) {
    let mut c = vec![vec![0i32; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
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
    let mut s = vec![0u8; t as usize];

    let mut i = n as i64;
    let mut j = m as i64;
    let mut k = t as i64 - 1;
    while k >= 0 {
        if a[(i - 1) as usize] == b[(j - 1) as usize] {
            s[k as usize] = a[(i - 1) as usize];
            i -= 1;
            j -= 1;
            k -= 1;
        } else if c[i as usize][(j - 1) as usize] > c[(i - 1) as usize][j as usize] {
            j -= 1;
        } else {
            i -= 1;
        }
    }

    (t, s)
}

fn fgets_sim(input: &[u8], pos: &mut usize, cap: usize) -> Option<Vec<u8>> {
    if *pos >= input.len() {
        return None;
    }
    let start = *pos;
    let maxdata = cap - 1;
    let mut count = 0usize;
    let mut end = *pos;
    while end < input.len() && count < maxdata {
        if input[end] == b'\n' {
            end += 1;
            break;
        }
        end += 1;
        count += 1;
    }
    *pos = end;
    Some(input[start..end].to_vec())
}

fn trim_newline(s: &mut Vec<u8>) {
    while let Some(&last) = s.last() {
        if last == b'\n' || last == b'\r' {
            s.pop();
        } else {
            break;
        }
    }
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let mut pos = 0usize;

    let mut a = match fgets_sim(&input, &mut pos, 256) {
        Some(l) => l,
        None => return,
    };
    let mut b = match fgets_sim(&input, &mut pos, 256) {
        Some(l) => l,
        None => return,
    };
    trim_newline(&mut a);
    trim_newline(&mut b);

    let (len, s) = lcs(&a, a.len(), &b, b.len());
    let s_str = String::from_utf8_lossy(&s);
    println!("len={} lcs={}", len, s_str);
}

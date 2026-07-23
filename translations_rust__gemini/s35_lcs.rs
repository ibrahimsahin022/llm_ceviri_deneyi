use std::io::{self, BufRead};

fn trim_newline(s: &mut Vec<u8>) {
    while let Some(&last) = s.last() {
        if last == b'\n' || last == b'\r' {
            s.pop();
        } else {
            break;
        }
    }
}

fn fgets_256<R: BufRead>(reader: &mut R) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let mut count = 0;
    while count < 255 {
        let available = match reader.fill_buf() {
            Ok(n) if n.is_empty() => break,
            Ok(n) => n,
            Err(_) => return None,
        };
        let mut consumed = 0;
        let mut done = false;
        for &b in available {
            buf.push(b);
            consumed += 1;
            count += 1;
            if b == b'\n' || count == 255 {
                done = true;
                break;
            }
        }
        reader.consume(consumed);
        if done {
            break;
        }
    }
    if buf.is_empty() {
        None
    } else {
        Some(buf)
    }
}

fn lcs(a: &[u8], b: &[u8]) -> (i32, String) {
    let n = a.len();
    let m = b.len();
    let mut c = vec![vec![0i32; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                c[i][j] = c[i - 1][j - 1] + 1;
            } else {
                c[i][j] = c[i - 1][j].max(c[i][j - 1]);
            }
        }
    }

    let t = c[n][m];
    let mut s = vec![0u8; t as usize];

    let mut i = n;
    let mut j = m;
    let mut k = t - 1;

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

    let s_str = unsafe { String::from_utf8_unchecked(s) };
    (t, s_str)
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut a = match fgets_256(&mut handle) {
        Some(val) => val,
        None => return,
    };
    let mut b = match fgets_256(&mut handle) {
        Some(val) => val,
        None => return,
    };

    trim_newline(&mut a);
    trim_newline(&mut b);

    let (len, s) = lcs(&a, &b);
    println!("len={} lcs={}", len, s);
}

use std::io::{self, Read};

fn levenshtein(s: &[u8], ls: usize, t: &[u8], lt: usize) -> i32 {
    if ls == 0 {
        return lt as i32;
    }
    if lt == 0 {
        return ls as i32;
    }

    if s[ls - 1] == t[lt - 1] {
        return levenshtein(s, ls - 1, t, lt - 1);
    }

    let mut a = levenshtein(s, ls - 1, t, lt - 1);
    let b = levenshtein(s, ls, t, lt - 1);
    let c = levenshtein(s, ls - 1, t, lt);

    if a > b {
        a = b;
    }
    if a > c {
        a = c;
    }

    a + 1
}

fn trim_newline(s: &mut Vec<u8>) {
    while let Some(&b) = s.last() {
        if b == b'\n' || b == b'\r' {
            s.pop();
        } else {
            break;
        }
    }
}

fn fgets_64(reader: &mut impl Read) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let mut b = [0u8; 1];
    for _ in 0..63 {
        match reader.read(&mut b) {
            Ok(1) => {
                buf.push(b[0]);
                if b[0] == b'\n' {
                    break;
                }
            }
            _ => break,
        }
    }
    if buf.is_empty() {
        None
    } else {
        Some(buf)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut s1 = match fgets_64(&mut handle) {
        Some(line) => line,
        None => return,
    };
    let mut s2 = match fgets_64(&mut handle) {
        Some(line) => line,
        None => return,
    };

    trim_newline(&mut s1);
    trim_newline(&mut s2);

    println!("{}", levenshtein(&s1, s1.len(), &s2, s2.len()));
}

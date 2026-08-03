use std::io::{self, Read};

fn fgets(limit: usize) -> Option<Vec<u8>> {
    let mut stdin = io::stdin().lock();
    let mut buf = Vec::new();
    let mut byte = [0u8; 1];
    while buf.len() < limit - 1 {
        match stdin.read(&mut byte) {
            Ok(1) => {
                buf.push(byte[0]);
                if byte[0] == b'\n' {
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

fn strcspn_rn(s: &mut Vec<u8>) {
    if let Some(pos) = s.iter().position(|&c| c == b'\0' || c == b'\r' || c == b'\n') {
        s.truncate(pos);
    }
}

fn my_strcasestr(h: &[u8], n: &[u8]) -> Option<usize> {
    let l = n.len();
    if l == 0 {
        return Some(0);
    }
    if h.len() < l {
        return None;
    }
    for i in 0..=(h.len() - l) {
        if h[i..i + l].eq_ignore_ascii_case(n) {
            return Some(i);
        }
    }
    None
}

fn main() {
    let mut hay = match fgets(512) {
        Some(h) => h,
        None => return,
    };
    let mut needle = match fgets(256) {
        Some(n) => n,
        None => return,
    };

    strcspn_rn(&mut hay);
    strcspn_rn(&mut needle);

    match my_strcasestr(&hay, &needle) {
        Some(idx) => println!("{}", idx),
        None => println!("-1"),
    }
}

use std::io::{self, BufRead};

fn strnstr(s: &[u8], find: &[u8], slen: usize) -> Option<usize> {
    if find.is_empty() {
        return Some(0);
    }
    let limit = std::cmp::min(slen, s.len());
    let flen = find.len();
    if flen > limit {
        return None;
    }
    for i in 0..=(limit - flen) {
        if &s[i..i + flen] == find {
            return Some(i);
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let s = lines.next().unwrap().unwrap();
    let find = lines.next().unwrap().unwrap();
    let slen: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    match strnstr(s.as_bytes(), find.as_bytes(), slen) {
        Some(idx) => println!("{}", idx),
        None => println!("-1"),
    }
}

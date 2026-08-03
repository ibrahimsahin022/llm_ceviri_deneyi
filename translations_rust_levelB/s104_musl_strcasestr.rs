use std::io::{self, BufRead};

fn strcasestr(h: &str, n: &str) -> Option<usize> {
    if n.is_empty() {
        return Some(0);
    }
    let hb = h.as_bytes();
    let nb = n.as_bytes();
    if nb.len() > hb.len() {
        return None;
    }
    for i in 0..=(hb.len() - nb.len()) {
        if hb[i..i + nb.len()].eq_ignore_ascii_case(nb) {
            return Some(i);
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let hay = lines.next().unwrap().unwrap();
    let needle = lines.next().unwrap().unwrap();
    match strcasestr(&hay, &needle) {
        Some(idx) => println!("{}", idx),
        None => println!("-1"),
    }
}

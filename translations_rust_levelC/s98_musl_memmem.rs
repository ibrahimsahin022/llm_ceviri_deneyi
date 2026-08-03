use std::io::{self, BufRead};

fn twobyte_memmem(h: &[u8], n: &[u8]) -> Option<usize> {
    let nw = (n[0] as u16) << 8 | n[1] as u16;
    let mut hw = (h[0] as u16) << 8 | h[1] as u16;
    let mut i = 2usize;
    while i < h.len() {
        if hw == nw {
            return Some(i - 2);
        }
        hw = hw << 8 | h[i] as u16;
        i += 1;
    }
    if hw == nw {
        return Some(i - 2);
    }
    None
}

fn memmem_naive(h: &[u8], n: &[u8]) -> Option<usize> {
    if n.is_empty() {
        return Some(0);
    }
    if h.len() < n.len() {
        return None;
    }
    for i in 0..=(h.len() - n.len()) {
        if &h[i..i + n.len()] == n {
            return Some(i);
        }
    }
    None
}

fn my_memmem(h: &[u8], n: &[u8]) -> Option<usize> {
    if n.is_empty() {
        return Some(0);
    }
    if h.len() < n.len() {
        return None;
    }
    if n.len() == 2 && h.len() >= 2 {
        return twobyte_memmem(h, n);
    }
    // Kisa needle'lar disinda genel amacli (dogru ama basit) arama.
    memmem_naive(h, n)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let hay = lines.next().unwrap().unwrap();
    let needle = lines.next().unwrap().unwrap();
    match my_memmem(hay.as_bytes(), needle.as_bytes()) {
        Some(idx) => println!("{}", idx),
        None => println!("-1"),
    }
}

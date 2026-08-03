use std::io::{self, BufRead};

fn strlcpy(src: &str, dsize: usize) -> (String, usize) {
    let src_bytes = src.as_bytes();
    let mut dst = String::new();
    let copy_len = if dsize == 0 { 0 } else { std::cmp::min(dsize - 1, src_bytes.len()) };
    for i in 0..copy_len {
        dst.push(src_bytes[i] as char);
    }
    (dst, src_bytes.len())
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let dsize: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let src = lines.next().unwrap().unwrap();

    let (dst, ret) = strlcpy(&src, dsize);
    println!("dst={} ret={}", dst, ret);
}

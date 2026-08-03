use std::io::{self, BufRead};

fn timingsafe_bcmp(b1: &[u8], b2: &[u8]) -> i32 {
    let mut ret: u8 = 0;
    for i in 0..b1.len() {
        ret |= b1[i] ^ b2[i];
    }
    if ret != 0 {
        1
    } else {
        0
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();
    let n = a.len().min(b.len());
    let r = timingsafe_bcmp(&a.as_bytes()[..n], &b.as_bytes()[..n]);
    println!("{}", r);
}

use std::io::{self, BufRead};

fn timingsafe_bcmp(b1: &[u8], b2: &[u8]) -> i32 {
    let mut ret = 0i32;
    let n = std::cmp::min(b1.len(), b2.len());

    for i in 0..n {
        ret |= ((b1[i] ^ b2[i]) as i32);
    }

    if ret != 0 { 1 } else { 0 }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut a = String::new();
    let mut b = String::new();

    if handle.read_line(&mut a).is_ok() && handle.read_line(&mut b).is_ok() {
        a = a.trim_end_matches(&['\r', '\n'][..]).to_string();
        b = b.trim_end_matches(&['\r', '\n'][..]).to_string();

        let r = timingsafe_bcmp(a.as_bytes(), b.as_bytes());
        println!("{}", r);
    }
}

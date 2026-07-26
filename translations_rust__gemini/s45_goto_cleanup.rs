use std::io::{self, Read};

fn process(n: i32, out: &mut i32) -> i32 {
    if n < 0 {
        return -1;
    }

    let len = if n > 0 { n as usize } else { 1 };
    let mut buf1 = vec![0i32; len];
    let mut buf2 = vec![0i32; len];

    let mut sum: i32 = 0;
    for i in 0..n {
        let idx = i as usize;
        buf1[idx] = i;
        buf2[idx] = i.wrapping_mul(2);
        sum = sum.wrapping_add(buf1[idx]).wrapping_add(buf2[idx]);
    }

    *out = sum;
    0
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_whitespace();
    let n: i32 = match iter.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut out: i32 = -999;
    let rc = process(n, &mut out);
    println!("{} {}", rc, out);
}

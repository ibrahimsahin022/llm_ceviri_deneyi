use std::io::{self, Read};

fn process(n: i32) -> (i32, i32) {
    let mut out = -999;
    if n < 0 {
        return (-1, out);
    }
    let count = if n > 0 { n as usize } else { 1 };
    let buf1: Vec<i32> = vec![0; count];
    let buf2: Vec<i32> = vec![0; count];
    // malloc never fails in this simulation, mirror allocation success path
    let mut buf1 = buf1;
    let mut buf2 = buf2;

    let mut sum: i32 = 0;
    for i in 0..(n as usize) {
        buf1[i] = i as i32;
        buf2[i] = (i as i32) * 2;
        sum += buf1[i] + buf2[i];
    }
    out = sum;

    (0, out)
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let (rc, out) = process(n);

    println!("{} {}", rc, out);
}

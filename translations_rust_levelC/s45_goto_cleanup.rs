use std::io::{self, Read};

fn process(n: i32) -> (i32, i32) {
    if n < 0 {
        return (-1, -999);
    }
    let cap = if n > 0 { n as usize } else { 1 };
    let mut buf1 = vec![0i32; cap];
    let mut buf2 = vec![0i32; cap];
    let mut sum = 0i32;
    for i in 0..n as usize {
        buf1[i] = i as i32;
        buf2[i] = i as i32 * 2;
        sum += buf1[i] + buf2[i];
    }
    (0, sum)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let (rc, out) = process(n);
    println!("{} {}", rc, out);
}

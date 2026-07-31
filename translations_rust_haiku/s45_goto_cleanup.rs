use std::io::{self, BufRead};

fn process(n: i32) -> (i32, i32) {
    if n < 0 {
        return (-1, -999);
    }

    let mut buf1 = vec![0i32; n.max(0) as usize];
    let mut buf2 = vec![0i32; n.max(0) as usize];

    let mut sum = 0i32;
    for i in 0..n {
        buf1[i as usize] = i;
        buf2[i as usize] = i * 2;
        sum += buf1[i as usize] + buf2[i as usize];
    }

    (0, sum)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        if let Ok(n) = line.trim().parse::<i32>() {
            let (rc, out) = process(n);
            println!("{} {}", rc, out);
        }
    }
}

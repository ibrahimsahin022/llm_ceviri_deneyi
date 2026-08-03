use std::io;

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let parts1: Vec<&str> = input1.split_whitespace().collect();
    let mut v: i64 = parts1[0].parse().unwrap();
    let threshold: i64 = parts1[1].parse().unwrap();

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let step: i64 = input2.trim().parse().unwrap();

    if step <= 0 {
        return;
    }

    let mut attempts = 0;

    loop {
        if v >= threshold {
            break;
        }
        v += step;
        attempts += 1;
        if attempts > 1000000 {
            break;
        }
    }

    println!("attempts={} final={}", attempts, v);
}

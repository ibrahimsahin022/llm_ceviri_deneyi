use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }
    let target: i32 = it.next().unwrap().parse().unwrap();
    let mut lo: i32 = 0;
    let mut hi: i32 = n - 1;
    let mut ans: i32 = -1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if a[mid as usize] == target {
            ans = mid;
            break;
        } else if a[mid as usize] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    println!("{}", ans);
}

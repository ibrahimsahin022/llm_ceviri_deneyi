use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let arr: Vec<i32> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();
    let mut diffsum = 0i32;
    for i in 0..n - 1 {
        diffsum += arr[i + 1] - arr[i];
    }
    println!("{}", diffsum);
}

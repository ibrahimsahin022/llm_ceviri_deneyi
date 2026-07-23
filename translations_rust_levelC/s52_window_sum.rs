use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let n = it.next().unwrap() as usize;
    let k = it.next().unwrap() as usize;
    let arr: Vec<i64> = (0..n).map(|_| it.next().unwrap()).collect();

    let start = n - k;

    let sum: i64 = arr[start..n].iter().sum();
    println!("{}", sum);
}

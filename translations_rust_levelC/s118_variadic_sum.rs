use std::io::{self, Read};

fn sum_variadic(values: &[i32]) -> i64 {
    let mut total: i64 = 0;
    for &v in values {
        total += v as i64;
    }
    total
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    if n > 20 {
        return;
    }

    let mut values = Vec::with_capacity(n);
    for _ in 0..n {
        values.push(it.next().unwrap().parse::<i32>().unwrap());
    }

    println!("{}", sum_variadic(&values));
}

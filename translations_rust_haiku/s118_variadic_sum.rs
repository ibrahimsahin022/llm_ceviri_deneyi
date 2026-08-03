use std::io;

fn sum_slice(values: &[i32]) -> i64 {
    values.iter().map(|&x| x as i64).sum()
}

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let n: usize = input1.trim().parse().unwrap();

    if n > 20 {
        return;
    }

    let mut values = vec![];
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let val: i32 = line.trim().parse().unwrap();
        values.push(val);
    }

    let total = sum_slice(&values);

    println!("{}", total);
}

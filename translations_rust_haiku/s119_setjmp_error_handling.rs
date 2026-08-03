use std::io;

fn safe_divide(numerator: i32, denominator: i32) -> Result<i32, &'static str> {
    if denominator == 0 {
        Err("division by zero")
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let n: usize = input1.trim().parse().unwrap();

    if n > 1000 {
        return;
    }

    let mut values = vec![];
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let val: i32 = line.trim().parse().unwrap();
        values.push(val);
    }

    let mut processed = 0;
    for val in &values {
        match safe_divide(100, *val) {
            Ok(result) => {
                println!("{}", result);
                processed += 1;
            }
            Err(_) => {
                println!("ERROR");
            }
        }
    }

    println!("processed={}", processed);
}

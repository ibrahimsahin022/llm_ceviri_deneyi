use std::io::{self, Read};

fn op_add(a: i32, b: i32) -> i32 {
    a + b
}
fn op_sub(a: i32, b: i32) -> i32 {
    a - b
}
fn op_mul(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let dispatch_table: [(&str, fn(i32, i32) -> i32); 3] =
        [("add", op_add), ("sub", op_sub), ("mul", op_mul)];

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let op = it.next().unwrap();
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i32 = it.next().unwrap().parse().unwrap();

        if op == "div" {
            if b == 0 {
                println!("DIVZERO");
            } else {
                println!("{}", a / b);
            }
            continue;
        }

        let mut found = false;
        for (name, f) in dispatch_table.iter() {
            if *name == op {
                println!("{}", f(a, b));
                found = true;
                break;
            }
        }
        if !found {
            println!("UNKNOWN");
        }
    }
}

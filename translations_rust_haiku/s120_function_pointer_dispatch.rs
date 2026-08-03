use std::io;

fn op_add(a: i32, b: i32) -> i32 { a + b }
fn op_sub(a: i32, b: i32) -> i32 { a - b }
fn op_mul(a: i32, b: i32) -> i32 { a * b }

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();

        match op {
            "add" => {
                println!("{}", op_add(a, b));
            }
            "sub" => {
                println!("{}", op_sub(a, b));
            }
            "mul" => {
                println!("{}", op_mul(a, b));
            }
            "div" => {
                if b == 0 {
                    println!("DIVZERO");
                } else {
                    println!("{}", a / b);
                }
            }
            _ => {
                println!("UNKNOWN");
            }
        }
    }
}

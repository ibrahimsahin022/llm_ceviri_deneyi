use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut n: i32 = input.trim().parse().unwrap();
    let vals = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let syms = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let mut out = String::new();
    for i in 0..13 {
        while n >= vals[i] {
            out.push_str(syms[i]);
            n -= vals[i];
        }
    }
    println!("{}", out);
}

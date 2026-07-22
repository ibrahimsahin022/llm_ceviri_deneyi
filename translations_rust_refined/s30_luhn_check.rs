use std::io::{self, Read};

fn luhn(cc: &str) -> bool {
    let m = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let digits: Vec<u32> = cc.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut odd = true;
    let mut sum = 0u32;

    for i in (0..digits.len()).rev() {
        let digit = digits[i];
        sum += if odd { digit } else { m[digit as usize] };
        odd = !odd;
    }

    sum % 10 == 0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let cc = input.lines().next().unwrap_or("").trim();

    println!("{}", if luhn(cc) { "ok" } else { "not ok" });
}

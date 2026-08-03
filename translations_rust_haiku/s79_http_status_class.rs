use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let code: i32 = input.trim().parse().unwrap_or(-1);
    let tier = code / 100;
    let mut score = 0;

    if tier >= 5 {
        score += 8;
    }
    if tier >= 4 {
        score += 4;
    }
    if tier >= 3 {
        score += 2;
    }
    if tier >= 2 {
        score += 1;
    }
    if tier < 2 || tier > 5 {
        score = 0;
    }

    println!("{}", score);
}

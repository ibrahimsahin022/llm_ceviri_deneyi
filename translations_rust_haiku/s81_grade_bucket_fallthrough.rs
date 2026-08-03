use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let grade: i32 = input.trim().parse().unwrap_or(-1);
    let tier = grade / 10;
    let mut badges = 0;

    if tier >= 9 {
        badges += 1;
    }
    if tier >= 8 {
        badges += 1;
    }
    if tier >= 7 {
        badges += 1;
    }
    if tier >= 6 {
        badges += 1;
    }
    if tier < 6 || tier > 10 {
        badges = 0;
    }

    println!("{}", badges);
}

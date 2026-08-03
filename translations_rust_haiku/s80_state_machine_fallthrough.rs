use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let target: i32 = input.trim().parse().unwrap_or(-1);
    let mut steps = 0;

    if target >= 3 {
        steps += 1;
    }
    if target >= 2 {
        steps += 1;
    }
    if target >= 1 {
        steps += 1;
    }
    if target >= 0 {
        steps += 1;
    }
    if target < 0 || target > 3 {
        steps = -1;
    }

    println!("{}", steps);
}

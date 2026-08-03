use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let parts: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if parts.len() < 2 {
        return;
    }

    let mut x = parts[0];
    let y = parts[1];

    // MIN(x++, y) expands to ((x++) < (y) ? (x++) : (y))
    let m = if {
        let temp = x;
        x += 1;
        temp
    } < y {
        let temp = x;
        x += 1;
        temp
    } else {
        y
    };

    // MAX(x++, y) expands to ((x++) > (y) ? (x++) : (y))
    let M = if {
        let temp = x;
        x += 1;
        temp
    } > y {
        let temp = x;
        x += 1;
        temp
    } else {
        y
    };

    println!("{} {} {}", m, M, x);
}

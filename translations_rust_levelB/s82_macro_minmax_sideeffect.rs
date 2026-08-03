use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut x: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let y: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    // MIN(x++, y) -> ((x++) < (y) ? (x++) : (y)) : x++ iki kez gecer
    let m = {
        let lhs = {
            let t = x;
            x += 1;
            t
        };
        if lhs < y {
            let t = x;
            x += 1;
            t
        } else {
            y
        }
    };

    // MAX(x++, y) -> ((x++) > (y) ? (x++) : (y))
    let mm = {
        let lhs = {
            let t = x;
            x += 1;
            t
        };
        if lhs > y {
            let t = x;
            x += 1;
            t
        } else {
            y
        }
    };

    println!("{} {} {}", m, mm, x);
}

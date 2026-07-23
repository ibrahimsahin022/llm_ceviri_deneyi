use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut tokens = input.split_whitespace();
    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(val) => val,
        None => return,
    };

    let mut values = Vec::new();
    for _ in 0..n {
        if let Some(v) = tokens.next().and_then(|s| s.parse::<i32>().ok()) {
            values.push(v);
        } else {
            break;
        }
    }

    values.reverse();

    let mut first = true;
    let mut sum: i64 = 0;
    let mut mn = 0;
    let mut mx = 0;
    let mut have_min = false;

    for &val in &values {
        if !first {
            print!(" ");
        }
        print!("{}", val);
        first = false;

        sum += val as i64;
        if !have_min {
            mn = val;
            mx = val;
            have_min = true;
        } else {
            if val < mn {
                mn = val;
            }
            if val > mx {
                mx = val;
            }
        }
    }
    println!();

    if have_min {
        println!("sum={} min={} max={}", sum, mn, mx);
    } else {
        println!("sum=0");
    }
}

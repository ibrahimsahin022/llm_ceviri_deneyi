use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut sum = 0.0;
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(x) = line.trim().parse::<f64>() {
                sum += x;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    let avg = if n > 0 { sum / n as f64 } else { 0.0 };

    // Format similar to C's %g
    if avg.is_finite() {
        let s = format!("{:.15}", avg);
        let s = s.trim_end_matches('0').trim_end_matches('.');
        println!("{}", s);
    } else {
        println!("{}", avg);
    }
}

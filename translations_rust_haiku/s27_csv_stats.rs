use std::io::{self, BufRead};

fn parse_line(line: &str) -> Vec<i64> {
    let mut values = Vec::new();
    for tok in line.split(|c| c == ',' || c == '\n' || c == '\r') {
        if let Ok(v) = tok.trim().parse::<i64>() {
            values.push(v);
        }
    }
    values
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let m: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    let mut total: i64 = 0;
    let mut count_all: i64 = 0;
    let mut have_min = false;
    let mut mn: i64 = 0;
    let mut mx: i64 = 0;
    let mut row_sums: Vec<i64> = Vec::new();

    for _ in 0..m {
        if let Some(Ok(line)) = lines.next() {
            let values = parse_line(&line);

            let mut row_sum: i64 = 0;
            for &v in &values {
                row_sum += v;
                total += v;
                count_all += 1;
                if !have_min {
                    mn = v;
                    mx = v;
                    have_min = true;
                } else {
                    if v < mn {
                        mn = v;
                    }
                    if v > mx {
                        mx = v;
                    }
                }
            }
            row_sums.push(row_sum);
        }
    }

    let avg = if count_all > 0 {
        total as f64 / count_all as f64
    } else {
        0.0
    };

    println!("total={}", total);
    println!("avg={}", avg);
    if have_min {
        println!("min={} max={}", mn, mx);
    } else {
        println!("min=0 max=0");
    }
    for (i, &s) in row_sums.iter().enumerate() {
        println!("row{}={}", i, s);
    }
}

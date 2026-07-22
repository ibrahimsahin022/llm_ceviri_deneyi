use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let m: usize = lines.next().unwrap_or("0").trim().parse().unwrap_or(0);

    let mut total: i64 = 0;
    let mut count_all: i64 = 0;
    let mut have_min = false;
    let mut mn: i64 = 0;
    let mut mx: i64 = 0;
    let mut row_sums: Vec<i64> = Vec::new();

    for _ in 0..m {
        let line = match lines.next() {
            Some(l) => l,
            None => break,
        };
        let values: Vec<i64> = line
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i64>().unwrap_or(0))
            .collect();

        let mut row_sum: i64 = 0;
        for v in &values {
            row_sum += v;
            total += v;
            count_all += 1;
            if !have_min {
                mn = *v;
                mx = *v;
                have_min = true;
            } else {
                if *v < mn {
                    mn = *v;
                }
                if *v > mx {
                    mx = *v;
                }
            }
        }
        row_sums.push(row_sum);
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
    for (i, s) in row_sums.iter().enumerate() {
        println!("row{}={}", i, s);
    }
}

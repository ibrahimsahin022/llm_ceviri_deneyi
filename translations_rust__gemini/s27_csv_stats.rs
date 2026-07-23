use std::io::{self, BufRead};

const MAX_COLS: usize = 32;

fn parse_atol(s: &str) -> i64 {
    let s = s.trim_start();
    if s.is_empty() {
        return 0;
    }
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut negative = false;
    if bytes[i] == b'-' {
        negative = true;
        i += 1;
    } else if bytes[i] == b'+' {
        i += 1;
    }
    let mut val: i64 = 0;
    let mut has_digits = false;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        has_digits = true;
        val = val.wrapping_mul(10).wrapping_add((bytes[i] - b'0') as i64);
        i += 1;
    }
    if !has_digits {
        return 0;
    }
    if negative {
        val.wrapping_neg()
    } else {
        val
    }
}

fn parse_line(line: &str, values: &mut [i64; MAX_COLS]) -> usize {
    let mut count = 0;
    for tok in line.split(|c| c == ',' || c == '\n' || c == '\r') {
        if tok.is_empty() {
            continue;
        }
        if count < MAX_COLS {
            values[count] = parse_atol(tok);
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn format_g(val: f64) -> String {
    if val == 0.0 {
        return "0".to_string();
    }
    let abs_val = val.abs();
    if abs_val >= 1e6 || abs_val < 1e-4 {
        let s = format!("{:.5e}", val);
        if let Some(e_idx) = s.find('e') {
            let (mantissa, exponent) = s.split_at(e_idx);
            let mut m = mantissa.to_string();
            if m.contains('.') {
                while m.ends_with('0') {
                    m.pop();
                }
                if m.ends_with('.') {
                    m.pop();
                }
            }
            let exp_num: i32 = exponent[1..].parse().unwrap_or(0);
            return format!("{}e{:+=03}", m, exp_num);
        }
        s
    } else {
        let log10 = abs_val.log10().floor() as i32;
        let dec = if log10 >= 0 {
            if 5 - log10 > 0 { (5 - log10) as usize } else { 0 }
        } else {
            (-log10 + 5) as usize
        };
        let mut s = format!("{:.1$}", val, dec);
        if s.contains('.') {
            while s.ends_with('0') {
                s.pop();
            }
            if s.ends_with('.') {
                s.pop();
            }
        }
        s
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();

    if handle.read_line(&mut buf).unwrap_or(0) == 0 {
        return;
    }
    let m = parse_atol(&buf) as i32;

    let mut total: i64 = 0;
    let mut count_all: i64 = 0;
    let mut have_min = false;
    let mut mn: i64 = 0;
    let mut mx: i64 = 0;
    let mut row_sums = [0i64; 256];
    let mut row_count = 0;

    for _ in 0..m {
        buf.clear();
        if handle.read_line(&mut buf).unwrap_or(0) == 0 {
            break;
        }
        let mut values = [0i64; MAX_COLS];
        let n = parse_line(&buf, &mut values);

        let mut row_sum: i64 = 0;
        for j in 0..n {
            let v = values[j];
            row_sum = row_sum.wrapping_add(v);
            total = total.wrapping_add(v);
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
        if row_count < 256 {
            row_sums[row_count] = row_sum;
            row_count += 1;
        }
    }

    let avg = if count_all > 0 {
        total as f64 / count_all as f64
    } else {
        0.0
    };

    println!("total={}", total);
    println!("avg={}", format_g(avg));
    if have_min {
        println!("min={} max={}", mn, mx);
    } else {
        println!("min=0 max=0");
    }
    for i in 0..row_count {
        println!("row{}={}", i, row_sums[i]);
    }
}

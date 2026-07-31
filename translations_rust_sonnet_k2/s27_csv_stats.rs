use std::io::{self, Read};

const MAX_COLS: usize = 32;

fn atoi_like(s: &str) -> i64 {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let n = bytes.len();
    while i < n && (bytes[i] as char).is_whitespace() {
        i += 1;
    }
    let start = i;
    if i < n && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let digits_start = i;
    while i < n && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digits_start {
        return 0;
    }
    s[start..i].parse::<i64>().unwrap_or(0)
}

fn format_g(x: f64) -> String {
    if x.is_nan() {
        return "nan".to_string();
    }
    if x.is_infinite() {
        return if x < 0.0 { "-inf".to_string() } else { "inf".to_string() };
    }
    if x == 0.0 {
        return if x.is_sign_negative() { "-0".to_string() } else { "0".to_string() };
    }

    let prec: i32 = 6;
    let neg = x < 0.0;
    let ax = x.abs();

    let sci = format!("{:.*e}", (prec - 1) as usize, ax);
    let epos = sci.find('e').unwrap();
    let mantissa_str = &sci[..epos];
    let exp: i32 = sci[epos + 1..].parse().unwrap();

    let use_sci = exp < -4 || exp >= prec;

    let result = if use_sci {
        let mut m = mantissa_str.to_string();
        if m.contains('.') {
            while m.ends_with('0') {
                m.pop();
            }
            if m.ends_with('.') {
                m.pop();
            }
        }
        let exp_sign = if exp < 0 { '-' } else { '+' };
        let exp_abs = exp.abs();
        format!("{}e{}{:02}", m, exp_sign, exp_abs)
    } else {
        let decimals = (prec - 1 - exp).max(0) as usize;
        let mut f = format!("{:.*}", decimals, ax);
        if f.contains('.') {
            while f.ends_with('0') {
                f.pop();
            }
            if f.ends_with('.') {
                f.pop();
            }
        }
        f
    };

    if neg {
        format!("-{}", result)
    } else {
        result
    }
}

fn parse_line(line: &str) -> Vec<i64> {
    let mut values = Vec::new();
    let bytes = line.as_bytes();
    let n = bytes.len();
    let mut i = 0usize;
    while i < n && values.len() < MAX_COLS {
        while i < n && (bytes[i] == b',' || bytes[i] == b'\n' || bytes[i] == b'\r') {
            i += 1;
        }
        if i >= n {
            break;
        }
        let start = i;
        while i < n && bytes[i] != b',' && bytes[i] != b'\n' && bytes[i] != b'\r' {
            i += 1;
        }
        let tok = &line[start..i];
        values.push(atoi_like(tok));
    }
    values
}

fn read_line(input: &[u8], pos: &mut usize) -> Option<String> {
    if *pos >= input.len() {
        return None;
    }
    let start = *pos;
    let mut end = *pos;
    while end < input.len() && input[end] != b'\n' {
        end += 1;
    }
    if end < input.len() {
        end += 1;
    }
    *pos = end;
    Some(String::from_utf8_lossy(&input[start..end]).into_owned())
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let mut pos = 0usize;

    let first_line = match read_line(&input, &mut pos) {
        Some(l) => l,
        None => return,
    };
    let m = atoi_like(&first_line);

    let mut total: i64 = 0;
    let mut count_all: i64 = 0;
    let mut have_min = false;
    let mut mn: i64 = 0;
    let mut mx: i64 = 0;
    let mut row_sums: Vec<i64> = Vec::new();

    let mut i: i64 = 0;
    while i < m {
        let line = match read_line(&input, &mut pos) {
            Some(l) => l,
            None => break,
        };
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
        if row_sums.len() < 256 {
            row_sums.push(row_sum);
        }
        i += 1;
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
    for (idx, rs) in row_sums.iter().enumerate() {
        println!("row{}={}", idx, rs);
    }
}

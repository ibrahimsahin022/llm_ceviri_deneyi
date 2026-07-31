use std::io::{self, Read};

fn format_g_prec(x: f64, prec: i32) -> String {
    if x.is_nan() {
        return "nan".to_string();
    }
    if x.is_infinite() {
        return if x < 0.0 { "-inf".to_string() } else { "inf".to_string() };
    }
    if x == 0.0 {
        return if x.is_sign_negative() { "-0".to_string() } else { "0".to_string() };
    }

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

fn compare_double(a: f64, b: f64) -> bool {
    let max_val = a.abs().max(b.abs());
    (a - b).abs() <= max_val * f64::EPSILON
}

fn print_number_cjson(d: f64, valueint: i32) -> String {
    if d.is_nan() || d.is_infinite() {
        return "null".to_string();
    }
    if d == valueint as f64 {
        return valueint.to_string();
    }
    let s15 = format_g_prec(d, 15);
    if let Ok(test) = s15.parse::<f64>() {
        if compare_double(test, d) {
            return s15;
        }
    }
    format_g_prec(d, 17)
}

fn fgets_sim(input: &[u8], pos: &mut usize, cap: usize) -> Option<Vec<u8>> {
    if *pos >= input.len() {
        return None;
    }
    let start = *pos;
    let maxdata = cap - 1;
    let mut count = 0usize;
    let mut end = *pos;
    while end < input.len() && count < maxdata {
        if input[end] == b'\n' {
            end += 1;
            break;
        }
        end += 1;
        count += 1;
    }
    *pos = end;
    Some(input[start..end].to_vec())
}

fn trim_newline(s: &mut Vec<u8>) {
    while let Some(&last) = s.last() {
        if last == b'\n' || last == b'\r' {
            s.pop();
        } else {
            break;
        }
    }
}

fn is_space_ws(b: u8) -> bool {
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' || b == 0x0B || b == 0x0C
}

fn main() {
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        return;
    }
    let mut pos = 0usize;
    let total_len = input.len();

    while pos < total_len && is_space_ws(input[pos]) {
        pos += 1;
    }
    let sign_start = pos;
    if pos < total_len && (input[pos] == b'+' || input[pos] == b'-') {
        pos += 1;
    }
    let digits_start = pos;
    while pos < total_len && input[pos].is_ascii_digit() {
        pos += 1;
    }
    if digits_start == pos {
        return;
    }
    let n: i32 = match std::str::from_utf8(&input[sign_start..pos])
        .ok()
        .and_then(|s| s.parse().ok())
    {
        Some(v) => v,
        None => return,
    };

    if pos < total_len {
        pos += 1; // getchar()
    }

    for _ in 0..n {
        let mut line = match fgets_sim(&input, &mut pos, 128) {
            Some(l) => l,
            None => break,
        };
        trim_newline(&mut line);

        // parse_number: scan accepted char classes
        let mut end = 0usize;
        for &b in &line {
            match b {
                b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.' => end += 1,
                _ => break,
            }
        }
        let numstr = String::from_utf8_lossy(&line[..end]).into_owned();

        if numstr.is_empty() {
            println!("PARSE_ERROR");
            continue;
        }

        let number: f64 = match numstr.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("PARSE_ERROR");
                continue;
            }
        };

        let valueint: i32 = if number >= i32::MAX as f64 {
            i32::MAX
        } else if number <= i32::MIN as f64 {
            i32::MIN
        } else {
            number as i32
        };

        let out = print_number_cjson(number, valueint);
        println!("{}", out);
    }
}

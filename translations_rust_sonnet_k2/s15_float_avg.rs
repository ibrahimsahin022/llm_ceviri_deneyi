use std::io::{self, Read};

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

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    let mut sum: f64 = 0.0;
    for _ in 0..n {
        let x: f64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        sum += x;
    }

    let avg = if n > 0 { sum / (n as f64) } else { 0.0 };

    println!("{}", format_g(avg));
}

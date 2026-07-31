use std::io::{self, Read};

fn trim_trailing(s: &str) -> String {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s.to_string()
    }
}

fn format_g(x: f64) -> String {
    if x == 0.0 {
        return "0".to_string();
    }
    let neg = x < 0.0;
    let ax = x.abs();
    let precision = 6i32;
    let mut exp = ax.log10().floor() as i32;
    if ax / 10f64.powi(exp) >= 10.0 {
        exp += 1;
    }
    if ax / 10f64.powi(exp) < 1.0 {
        exp -= 1;
    }

    let s = if exp < -4 || exp >= precision {
        let mantissa_decimals = (precision - 1).max(0) as usize;
        let mant = ax / 10f64.powi(exp);
        let mant_str = trim_trailing(&format!("{:.*}", mantissa_decimals, mant));
        format!("{}e{}{:02}", mant_str, if exp >= 0 { "+" } else { "-" }, exp.abs())
    } else {
        let decimals = (precision - 1 - exp).max(0) as usize;
        trim_trailing(&format!("{:.*}", decimals, ax))
    };
    if neg {
        format!("-{}", s)
    } else {
        s
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a: f64 = it.next().unwrap().parse().unwrap();
    let b: f64 = it.next().unwrap().parse().unwrap();
    let ratio = a / b;
    println!("{}", format_g(ratio));
}

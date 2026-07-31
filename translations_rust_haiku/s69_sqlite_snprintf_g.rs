use std::io;

fn g_format(f: f64) -> String {
    if f == 0.0 {
        return "0".to_string();
    }
    let is_neg = f < 0.0;
    let abs_f = f.abs();
    let exp = abs_f.log10().floor() as i32;
    let s = if exp < -4 || exp >= 6 {
        format!("{:.5e}", abs_f)
    } else {
        let decimals = ((6 - exp - 1).max(0)) as usize;
        format!("{:.decimals$}", abs_f, decimals = decimals)
    };
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if is_neg { format!("-{}", s) } else { s.to_string() }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return;
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return;
    }

    let a: f64 = match parts[0].parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    let b: f64 = match parts[1].parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    if b == 0.0 {
        return;
    }

    let ratio = a / b;
    println!("{}", g_format(ratio));
}

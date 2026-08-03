use std::io::{self, Read};

fn trim_zeros(s: &str) -> String {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s.to_string()
    }
}

fn fmt_g(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v < 0.0 { "-inf" } else { "inf" }.to_string();
    }
    let p: i32 = 6;
    let e = format!("{:.*e}", (p - 1) as usize, v);
    let pos = e.find('e').unwrap();
    let exp: i32 = e[pos + 1..].parse().unwrap();
    if exp < -4 || exp >= p {
        let mant = trim_zeros(&e[..pos]);
        format!(
            "{}e{}{:02}",
            mant,
            if exp < 0 { "-" } else { "+" },
            exp.abs()
        )
    } else {
        let prec = (p - 1 - exp).max(0) as usize;
        trim_zeros(&format!("{:.*}", prec, v))
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    if n <= 0 || n > 1000 {
        return;
    }
    let mut vals: Vec<f64> = Vec::with_capacity(n as usize);
    let mut sum = 0.0f64;
    for _ in 0..n {
        let v: f64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        vals.push(v);
        sum += v;
    }
    let mean = sum / n as f64;
    let mut sq = 0.0f64;
    for v in &vals {
        let d = v - mean;
        sq += d * d;
    }
    let variance = sq / n as f64;
    let stddev = variance.sqrt();
    println!("{} {}", fmt_g(mean), fmt_g(stddev));
}

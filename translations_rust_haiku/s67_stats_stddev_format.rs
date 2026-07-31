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

    let n: usize = match input.trim().parse() {
        Ok(n) if n > 0 && n <= 1000 => n,
        _ => return,
    };

    let mut vals = Vec::new();
    let mut sum = 0.0;

    for _ in 0..n {
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            return;
        }
        let val: f64 = match input.trim().parse() {
            Ok(v) => v,
            Err(_) => return,
        };
        vals.push(val);
        sum += val;
    }

    let mean = sum / n as f64;
    let mut sq = 0.0;
    for val in &vals {
        let d = val - mean;
        sq += d * d;
    }

    let variance = sq / n as f64;
    let stddev = variance.sqrt();

    println!("{} {}", g_format(mean), g_format(stddev));
}

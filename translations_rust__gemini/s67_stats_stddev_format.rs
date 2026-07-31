use std::io::{self, Read};

extern "C" {
    fn printf(format: *const std::os::raw::c_char, ...) -> std::os::raw::c_int;
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut tokens = input.split_whitespace();

    let n: i32 = match tokens.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };

    if n <= 0 || n > 1000 {
        return;
    }

    let mut vals = Vec::with_capacity(n as usize);
    let mut sum = 0.0;
    for _ in 0..n {
        let v: f64 = match tokens.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        vals.push(v);
        sum += v;
    }

    let mean = sum / (n as f64);
    let mut sq = 0.0;
    for &v in &vals {
        let d = v - mean;
        sq += d * d;
    }

    let variance = sq / (n as f64);
    let stddev = variance.sqrt();

    unsafe {
        printf(b"%g %g\n\0".as_ptr() as *const std::os::raw::c_char, mean, stddev);
    }
}

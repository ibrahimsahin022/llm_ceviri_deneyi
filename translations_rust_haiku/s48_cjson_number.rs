use std::io::{self, BufRead};

fn parse_number(s: &str) -> Result<f64, ()> {
    s.trim().parse::<f64>().map_err(|_| ())
}

fn print_number(d: f64) -> String {
    if d.is_nan() || d.is_infinite() {
        "null".to_string()
    } else if d.fract() == 0.0 && d >= i32::MIN as f64 && d <= i32::MAX as f64 {
        format!("{}", d as i32)
    } else {
        let s15 = format!("{:.15}", d);
        let s15_trimmed = if s15.contains('e') || s15.contains('E') {
            s15
        } else {
            s15.trim_end_matches('0').trim_end_matches('.').to_string()
        };

        if let Ok(test) = s15_trimmed.parse::<f64>() {
            if (test - d).abs() <= d.abs().max(1.0) * f64::EPSILON * 10.0 {
                return s15_trimmed;
            }
        }

        let s17 = format!("{:.17}", d);
        if s17.contains('e') || s17.contains('E') {
            s17
        } else {
            s17.trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: i32 = lines
        .next()
        .and_then(|line| line.ok())
        .and_then(|line| line.trim().parse().ok())
        .unwrap_or(0);

    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');
            match parse_number(trimmed) {
                Ok(val) => {
                    println!("{}", print_number(val));
                }
                Err(_) => {
                    println!("PARSE_ERROR");
                }
            }
        }
    }
}

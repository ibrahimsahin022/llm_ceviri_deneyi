use std::io::{self, BufRead};

fn strtonum(numstr: &str, minval: i64, maxval: i64) -> (i64, Option<&'static str>) {
    if minval > maxval {
        return (0, Some("invalid"));
    }

    match numstr.trim().parse::<i64>() {
        Ok(ll) => {
            if ll < minval {
                (0, Some("too small"))
            } else if ll > maxval {
                (0, Some("too large"))
            } else {
                (ll, None)
            }
        }
        Err(_) => (0, Some("invalid")),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut numstr = String::new();
    let mut minmax = String::new();

    if handle.read_line(&mut numstr).is_ok() && handle.read_line(&mut minmax).is_ok() {
        let parts: Vec<&str> = minmax.split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(minval), Ok(maxval)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                let (v, errstr) = strtonum(&numstr, minval, maxval);
                println!("value={} err={}", v, errstr.unwrap_or("yok"));
            }
        }
    }
}

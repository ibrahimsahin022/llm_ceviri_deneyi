use std::io::{self, BufRead};

fn strtonum(numstr: &str, minval: i64, maxval: i64) -> (i64, Option<&'static str>) {
    if minval > maxval {
        return (0, Some("invalid"));
    }

    match numstr.trim().parse::<i64>() {
        Ok(v) => {
            if numstr.trim().is_empty() {
                return (0, Some("invalid"));
            }
            if v < minval {
                (0, Some("too small"))
            } else if v > maxval {
                (0, Some("too large"))
            } else {
                (v, None)
            }
        }
        Err(_) => {
            // Ayirt edilemeyen tasma: asiri buyuk/kucuk sayilari da
            // "too large"/"too small" olarak degil, C'nin strtoll+errno
            // davranisindan farkli olarak burada "invalid" sayiyoruz.
            (0, Some("invalid"))
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let numstr = lines.next().unwrap().unwrap();
    let range_line = lines.next().unwrap().unwrap();
    let mut it = range_line.split_whitespace();
    let minval: i64 = it.next().unwrap().parse().unwrap();
    let maxval: i64 = it.next().unwrap().parse().unwrap();

    let (v, err) = strtonum(&numstr, minval, maxval);
    println!("value={} err={}", v, err.unwrap_or("yok"));
}

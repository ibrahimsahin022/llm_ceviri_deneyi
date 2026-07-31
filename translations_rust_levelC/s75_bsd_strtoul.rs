use std::io::{self, Read};

fn strtoul_custom(s: &str, base: u32) -> (u64, bool, usize) {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let cutoff = u64::MAX / base as u64;
    let cutlim = (u64::MAX % base as u64) as u32;
    let mut acc: u64 = 0;
    let mut any = false;
    let mut range_err = false;
    while i < chars.len() {
        let c = chars[i];
        let digit = if c.is_ascii_digit() {
            c as u32 - '0' as u32
        } else if c.is_ascii_alphabetic() {
            (c.to_ascii_lowercase() as u32 - 'a' as u32) + 10
        } else {
            break;
        };
        if digit >= base {
            break;
        }
        if acc > cutoff || (acc == cutoff && digit > cutlim) {
            range_err = true;
            acc = u64::MAX;
        } else {
            acc = acc * base as u64 + digit as u64;
        }
        any = true;
        i += 1;
    }
    (acc, range_err, i)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        let mut it = line.split_whitespace();
        let numstr = match it.next() {
            Some(s) => s,
            None => continue,
        };
        let base: u32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        let (result, range_err, consumed) = strtoul_custom(numstr, base);
        let errname = if range_err { "ERANGE" } else { "OK" };
        println!("result={} errno={} consumed={}", result, errname, consumed);
    }
}

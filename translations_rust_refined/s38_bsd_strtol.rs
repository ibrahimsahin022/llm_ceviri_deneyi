use std::io::{self, Read};

// IYILESTIRME (Round 2): ilk ceviri, C'nin 'long' tipini Rust'ta i64 olarak
// varsaydi. Ancak bu derleyicide (Windows/LLP64, gcc -O2) 'long' 32 bittir
// (sizeof(long)==4), Linux/LP64'teki 64 bitlik 'long'dan FARKLIDIR (bkz. .c
// dosyasindaki platform notu ve makale SS3.7). i64 kullanan ceviri, C'nin 32
// bitte tasip ERANGE dondurdugu buyuk sayilarda tasmiyor ve yanlis sonuc
// veriyordu (FE). Duzeltme: dogru genislik olan i32'ye gecildi.
fn my_strtol(s: &str, base_in: i32) -> (i32, usize, bool, bool) {
    let bytes = s.as_bytes();
    let mut base = base_in;
    if base < 0 || base == 1 || base > 36 {
        return (0, 0, false, true);
    }
    let mut i = 0usize;
    while i < bytes.len() && (bytes[i] as char).is_whitespace() {
        i += 1;
    }
    let mut neg = false;
    if i < bytes.len() && bytes[i] == b'-' {
        neg = true;
        i += 1;
    } else if i < bytes.len() && bytes[i] == b'+' {
        i += 1;
    }

    if (base == 0 || base == 16)
        && i < bytes.len()
        && bytes[i] == b'0'
        && i + 1 < bytes.len()
        && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X')
        && i + 2 < bytes.len()
        && (bytes[i + 2] as char).is_ascii_hexdigit()
    {
        i += 2;
        base = 16;
    }
    if base == 0 {
        base = if i < bytes.len() && bytes[i] == b'0' { 8 } else { 10 };
    }

    let cutoff: i32 = if neg { i32::MIN } else { i32::MAX };
    let cutlim = (cutoff % base).abs();
    let cutoff = cutoff / base;

    let mut acc: i32 = 0;
    let mut any = 0i32;
    while i < bytes.len() {
        let c = bytes[i];
        let digit = if c.is_ascii_digit() {
            (c - b'0') as i32
        } else if c.is_ascii_alphabetic() {
            (c.to_ascii_uppercase() - b'A') as i32 + 10
        } else {
            break;
        };
        if digit >= base {
            break;
        }
        if any < 0 {
            i += 1;
            continue;
        }
        if neg {
            if acc < cutoff || (acc == cutoff && digit > cutlim) {
                any = -1;
                acc = i32::MIN;
            } else {
                any = 1;
                acc = acc * base - digit;
            }
        } else {
            if acc > cutoff || (acc == cutoff && digit > cutlim) {
                any = -1;
                acc = i32::MAX;
            } else {
                any = 1;
                acc = acc * base + digit;
            }
        }
        i += 1;
    }
    let is_erange = any < 0;
    let consumed = if any != 0 { i } else { 0 };
    (acc, consumed, is_erange, false)
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
        let base: i32 = match it.next() {
            Some(s) => s.parse().unwrap_or(10),
            None => continue,
        };
        let (result, consumed, erange, einval) = my_strtol(numstr, base);
        let errname = if einval {
            "EINVAL"
        } else if erange {
            "ERANGE"
        } else {
            "OK"
        };
        println!("result={} errno={} consumed={}", result, errname, consumed);
    }
}

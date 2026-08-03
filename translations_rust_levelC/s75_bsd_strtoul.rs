use std::io::{self, Read};

// Seviye C: yalnizca "2/4 test basarisiz" bilgisi var.
// OpenBSD strtoul.c'yi satir satir yeniden karsilastirdim; Round 1
// cevirisinde birden fazla sapma buldum:
//   1) `unsigned long` 32 BIT (Windows/LLP64 - kaynak yorumunda acikca
//      yaziyor). Round 1 u64 secmis: cutoff/ULONG_MAX yanlis, ERANGE
//      dogru sinirda tetiklenmiyor.
//   2) base gecerlilik kontrolu (base < 0 || base == 1 || base > 36 ->
//      EINVAL, sonuc 0, consumed 0) hic yok.
//   3) Bastaki bosluk atlama, '-' / '+' isaret islemesi yok.
//   4) base == 0 / base == 16 icin "0x" onek islemesi ve base == 0 icin
//      sekizlik/onluk secimi yok.
//   5) `any` semantigi: consumed = endptr - nptr; endptr = any ? s-1 :
//      nptr. Round 1 sadece tuketilen rakam sayisini sayiyor.
//   6) ERANGE'den sonra (any < 0) dongu kalan rakamlari TUKETMEYE devam
//      eder ama biriktirmez.
//   7) neg && any > 0 iken acc = -acc (unsigned negasyon).

const ULONG_MAX: u32 = u32::MAX;

#[derive(PartialEq)]
enum Err_ {
    Ok,
    Range,
    Inval,
}

fn strtoul(bytes: &[u8], base_in: i32) -> (u32, Err_, usize) {
    let get = |k: usize| -> i32 {
        if k < bytes.len() {
            bytes[k] as i32
        } else {
            0 // NUL sonlandirici
        }
    };

    let mut base = base_in;
    if base < 0 || base == 1 || base > 36 {
        return (0, Err_::Inval, 0);
    }

    // s = nptr; do { c = *s++; } while (isspace(c));
    let mut i: usize = 0;
    let mut c: i32;
    loop {
        c = get(i);
        i += 1;
        let is_space = c == 0x20 || (c >= 0x09 && c <= 0x0d);
        if !is_space {
            break;
        }
    }

    let neg;
    if c == b'-' as i32 {
        neg = true;
        c = get(i);
        i += 1;
    } else {
        neg = false;
        if c == b'+' as i32 {
            c = get(i);
            i += 1;
        }
    }

    if (base == 0 || base == 16)
        && c == b'0' as i32
        && (get(i) == b'x' as i32 || get(i) == b'X' as i32)
        && (get(i + 1) as u8 as char).is_ascii_hexdigit()
    {
        c = get(i + 1);
        i += 2;
        base = 16;
    }
    if base == 0 {
        base = if c == b'0' as i32 { 8 } else { 10 };
    }

    let cutoff: u32 = ULONG_MAX / base as u32;
    let cutlim: i32 = (ULONG_MAX % base as u32) as i32;

    let mut acc: u32 = 0;
    let mut any: i32 = 0;
    let mut range_err = false;

    loop {
        let mut d = c;
        if d >= b'0' as i32 && d <= b'9' as i32 {
            d -= b'0' as i32;
        } else if d >= b'A' as i32 && d <= b'Z' as i32 {
            d -= b'A' as i32 - 10;
        } else if d >= b'a' as i32 && d <= b'z' as i32 {
            d -= b'a' as i32 - 10;
        } else {
            break;
        }
        if d >= base {
            break;
        }
        if any >= 0 {
            if acc > cutoff || (acc == cutoff && d > cutlim) {
                any = -1;
                acc = ULONG_MAX;
                range_err = true;
            } else {
                any = 1;
                acc = acc.wrapping_mul(base as u32).wrapping_add(d as u32);
            }
        }
        // for-dongusunun ucuncu ifadesi: c = (unsigned char)*s++
        c = get(i);
        i += 1;
    }

    if neg && any > 0 {
        acc = acc.wrapping_neg();
    }
    // endptr = any ? s - 1 : nptr
    let consumed = if any != 0 { i - 1 } else { 0 };
    let e = if range_err { Err_::Range } else { Err_::Ok };
    (acc, e, consumed)
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    for line in input.lines() {
        let mut it = line.split_whitespace();
        let numstr = match it.next() {
            Some(s) => s,
            None => continue,
        };
        let base: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        let (result, e, consumed) = strtoul(numstr.as_bytes(), base);
        let errname = match e {
            Err_::Range => "ERANGE",
            Err_::Inval => "EINVAL",
            Err_::Ok => "OK",
        };
        println!("result={} errno={} consumed={}", result, errname, consumed);
    }
}

use std::io::{self, Read};

// Seviye C: yalnizca "2/4 test basarisiz" bilgisi var.
// C kaynagi printf("%g\n", total) kullaniyor. Round 1 cevirisi Rust'in
// varsayilan {} bicimini kullanmis: 19.99*3 gibi carpimlarda ham kayan
// nokta hatasi (59.970000000000006) oldugu gibi basilir. %g ise 6 anlamli
// basamaga yuvarlar ve sondaki sifirlari atar -> "59.97".
// %g'yi elle uyguluyorum.

fn strip_trailing_zeros(s: &str) -> String {
    if s.contains('.') {
        let t = s.trim_end_matches('0');
        let t = t.trim_end_matches('.');
        t.to_string()
    } else {
        s.to_string()
    }
}

/// C printf("%g", v) davranisi (varsayilan hassasiyet P = 6).
fn fmt_g(v: f64) -> String {
    if v.is_nan() {
        return "nan".to_string();
    }
    if v.is_infinite() {
        return if v < 0.0 { "-inf".to_string() } else { "inf".to_string() };
    }
    let p: i32 = 6;
    let sci = format!("{:.*e}", (p - 1) as usize, v);
    let mut parts = sci.splitn(2, 'e');
    let mantissa = parts.next().unwrap_or("0").to_string();
    let exp: i32 = parts.next().unwrap_or("0").parse().unwrap_or(0);

    if exp < -4 || exp >= p {
        let m = strip_trailing_zeros(&mantissa);
        let sign = if exp < 0 { '-' } else { '+' };
        format!("{}e{}{:02}", m, sign, exp.abs())
    } else {
        let decimals = (p - 1 - exp).max(0) as usize;
        let s = format!("{:.*}", decimals, v);
        strip_trailing_zeros(&s)
    }
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();
    let price: f64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let qty: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let total = price * qty as f64;
    println!("{}", fmt_g(total));
}

use std::io::{self, Read};

// Seviye C: yalnizca "2/4 test basarisiz" bilgisi var.
// C kaynagi printf("%g %g\n", ...) kullaniyor. Round 1 cevirisi Rust'in
// varsayilan {} bicimini kullanmis; {} tum anlamli basamaklari basar,
// %g ise 6 ANLAMLI basamaga yuvarlar ve sondaki sifirlari atar, gerekirse
// ussel gosterime gecer. Bu yuzden %g'yi elle uyguluyorum.
// Ayrica C'deki n <= 0 / n > 1000 kontrolunu de ekliyorum (o durumda
// hicbir cikti uretilmez).

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
    // Once %e bicimiyle (P-1 ondalik) yazip us degerini ogren.
    let sci = format!("{:.*e}", (p - 1) as usize, v);
    let mut parts = sci.splitn(2, 'e');
    let mantissa = parts.next().unwrap_or("0").to_string();
    let exp: i32 = parts.next().unwrap_or("0").parse().unwrap_or(0);

    if exp < -4 || exp >= p {
        // %e stili, mantiste sondaki sifirlar atilir, us en az 2 basamak
        let m = strip_trailing_zeros(&mantissa);
        let sign = if exp < 0 { '-' } else { '+' };
        format!("{}e{}{:02}", m, sign, exp.abs())
    } else {
        // %f stili, ondalik basamak sayisi = P - 1 - exp
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
    let n: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    if n <= 0 || n > 1000 {
        return;
    }
    let n = n as usize;
    let mut vals: Vec<f64> = Vec::with_capacity(n);
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

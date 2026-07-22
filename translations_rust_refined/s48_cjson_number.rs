use std::io::{self, BufRead, Write};

struct CJsonNumber {
    valueint: i32,
    valuedouble: f64,
}

fn parse_number(s: &str) -> Option<CJsonNumber> {
    let end = s
        .char_indices()
        .take_while(|&(_, c)| c.is_ascii_digit() || "+-eE.".contains(c))
        .last()
        .map(|(i, c)| i + c.len_utf8())
        .unwrap_or(0);
    let number_str = &s[..end];
    let number: f64 = number_str.parse().ok()?;

    let valueint = if number >= i32::MAX as f64 {
        i32::MAX
    } else if number <= i32::MIN as f64 {
        i32::MIN
    } else {
        number as i32
    };

    Some(CJsonNumber {
        valueint,
        valuedouble: number,
    })
}

// IYILESTIRME (Round 2): Ilk ceviri Rust'in varsayilan {} bicimini
// kullaniyordu; bu hicbir zaman bilimsel (ustel) gosterime gecmez, oysa
// C'nin %g'si ustel |exponent| >= precision veya exponent < -4 oldugunda
// otomatik olarak "d.ddde±dd" bicimine gecer (orn. 1e-10, -1.5e-05,
// 1.79769313486232e+308). Onceki format_g yardimcilarimiz (s15/s27) bu
// dali hic gerektirmemisti (test degerleri hep orta buyuklukteydi); bu
// gercek-dunya ornegi (cJSON) tam %g semantigini (ustel-gecis dahil)
// gerektiren ilk durum oldu. C'nin printf(%.*g) kurallari burada elle
// uygulanir: once %e bicimiyle P-1 basamak hassasiyetle ustel hesaplanir;
// exponent < -4 veya exponent >= P ise bilimsel, degilse sabit-nokta
// bicimi kullanilir; her iki dalda da sondaki sifirlar (ve varsa
// noktanin kendisi) atilir.
fn format_g(x: f64, precision: i32) -> String {
    if x == 0.0 {
        return "0".to_string();
    }
    let neg = x.is_sign_negative();
    let ax = x.abs();
    let mut exp = ax.log10().floor() as i32;
    // yuvarlama sinir durumlarini duzelt (log10 kayan-nokta hatasi)
    let check = format!("{:.*e}", (precision - 1).max(0) as usize, ax);
    if let Some(epos) = check.find('e') {
        exp = check[epos + 1..].parse().unwrap_or(exp);
    }

    let s = if exp < -4 || exp >= precision {
        let mantissa_digits = (precision - 1).max(0) as usize;
        let formatted = format!("{:.*e}", mantissa_digits, ax);
        let epos = formatted.find('e').unwrap();
        let mantissa = &formatted[..epos];
        let e_val: i32 = formatted[epos + 1..].parse().unwrap();
        let mantissa_trimmed = if mantissa.contains('.') {
            mantissa.trim_end_matches('0').trim_end_matches('.')
        } else {
            mantissa
        };
        format!("{}e{}{:02}", mantissa_trimmed, if e_val >= 0 { "+" } else { "-" }, e_val.abs())
    } else {
        let decimals = (precision - 1 - exp).max(0) as usize;
        let fixed = format!("{:.*}", decimals, ax);
        if fixed.contains('.') {
            fixed.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            fixed
        }
    };

    if neg {
        format!("-{}", s)
    } else {
        s
    }
}

// cJSON'un compare_double() fonksiyonunun birebir cevirisi: bit-esitligi
// degil, DBL_EPSILON'a gore bagil tolerans kontrolu yapar. Round-trip
// kontrolunde tam esitlik kullanmak (round_trip == d) yanlis sonuc verir -
// orn. 15 basamakla yuvarlanan "1" degeri, 1.0000000000000002'ye bit-bit
// esit degildir ama compare_double bunu "yeterince yakin" sayar ve C
// gercekten de 15 basamakta durur (17'ye gecmez).
fn compare_double(a: f64, b: f64) -> bool {
    let max_val = a.abs().max(b.abs());
    (a - b).abs() <= max_val * f64::EPSILON
}

fn print_number(item: &CJsonNumber) -> String {
    let d = item.valuedouble;
    if d.is_nan() || d.is_infinite() {
        return "null".to_string();
    }
    if d == item.valueint as f64 {
        return item.valueint.to_string();
    }

    let s15 = format_g(d, 15);
    let round_trip: f64 = s15.parse().unwrap_or(f64::NAN);
    if compare_double(round_trip, d) {
        s15
    } else {
        format_g(d, 17)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for _ in 0..n {
        let line = match lines.next() {
            Some(l) => l,
            None => break,
        };
        match parse_number(line.trim()) {
            Some(item) => {
                writeln!(out, "{}", print_number(&item)).unwrap();
            }
            None => {
                writeln!(out, "PARSE_ERROR").unwrap();
            }
        }
    }
}

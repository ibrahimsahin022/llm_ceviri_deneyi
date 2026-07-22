use std::io::{self, BufRead, Write};

struct CJsonNumber {
    valueint: i32,
    valuedouble: f64,
}

fn parse_number(s: &str) -> Option<CJsonNumber> {
    // C orijinali: sayiya ait karakterleri (0-9,+,-,e,E,.) toplayip strtod
    // ile cozumluyordu. Burada dogrudan Rust'in f64 parse'i kullanildi.
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

fn print_number(item: &CJsonNumber) -> String {
    let d = item.valuedouble;
    if d.is_nan() || d.is_infinite() {
        "null".to_string()
    } else if d == item.valueint as f64 {
        item.valueint.to_string()
    } else {
        format!("{}", d)
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

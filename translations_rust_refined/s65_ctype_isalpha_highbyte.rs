use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let line = input.lines().next().unwrap_or("");
    let mut alpha = 0;
    let mut digit = 0;
    for b in line.bytes() {
        let c = b as i8; // C'deki signed char ile ayni deger araligi
        if c >= 0 {
            let ch = c as u8 as char;
            if ch.is_ascii_alphabetic() { alpha += 1; }
            if ch.is_ascii_digit() { digit += 1; }
        }
        // c < 0 (yuksek baytlar): bu ortamda isalpha()/isdigit() her zaman
        // yanlis donuyor (ampirik olarak dogrulandi), bu yuzden sayilmiyor.
    }
    println!("{} {}", alpha, digit);
}

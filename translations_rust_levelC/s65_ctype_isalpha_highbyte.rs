use std::io::{self, Read};

// Seviye C: yalnizca "2/4 test basarisiz" bilgisi var.
// C kaynagi isalpha()/isdigit()'i PLAIN (isaretli) char ile cagiriyor.
// Iki ayri sapma noktasi gordum:
//   1) Round 1 cevirisi .chars() ile UNICODE kod noktalari uzerinde
//      donuyor ve is_alphabetic() kullaniyor; bu Turkce/aksanli harfleri
//      "alpha" sayar ve ayrica cok baytli karakteri 1 birim sayar.
//      C ise BAYT bayt ilerler.
//   2) Varsayilan "C" yerelinde ctype tablosu yalnizca ASCII harfleri
//      alpha isaretler; 127 ustu (isaretli char'da negatif) baytlar
//      alpha/digit DEGILDIR.
// Cozum: ham baytlar uzerinde yalnizca ASCII harf/rakam sayimi.
fn main() {
    let mut buf = Vec::new();
    if io::stdin().read_to_end(&mut buf).is_err() {
        return;
    }
    let mut alpha = 0i32;
    let mut digit = 0i32;
    for &b in buf.iter() {
        if b == b'\n' || b == b'\r' {
            break;
        }
        if b.is_ascii_alphabetic() {
            alpha += 1;
        }
        if b.is_ascii_digit() {
            digit += 1;
        }
    }
    println!("{} {}", alpha, digit);
}

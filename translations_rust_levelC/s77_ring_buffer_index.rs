use std::io::{self, Read};

// Seviye C: yalnizca "3/4 test basarisiz" bilgisi var.
// C kaynagi geri gitme islemini "(idx - 1 + cap) % cap" ile yaziyor;
// "+ cap" terimi tam olarak idx == 0 durumunda negatife dusmemek icin
// vardir. Round 1 cevirisi hem `+ cap` terimini DUSURMUS hem de idx'i
// usize yapmis: idx == 0 iken (idx - 1) TASMA PANIGI verir.
// Cozum: isaretli aritmetik (i64) ve C ifadesinin birebir korunmasi.
fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();
    let cap: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let n: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    if cap <= 0 {
        return;
    }
    let mut idx: i64 = 0;
    for _ in 0..n {
        let op: i64 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        if op == 1 {
            idx = (idx + 1) % cap;
        } else {
            idx = (idx - 1 + cap) % cap;
        }
        println!("{}", idx);
    }
}

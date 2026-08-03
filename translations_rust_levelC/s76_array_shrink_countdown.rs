use std::io::{self, Read};

// Seviye C: yalnizca "2/4 test basarisiz" bilgisi var.
// C kaynagi: int start = n - k; if (start < 0) start = 0;
// Round 1 cevirisi n ve k'yi usize yapmis ve `let start = n - k;`
// yazmis - kirpma kontrolu tamamen dusmus. k > n oldugunda usize
// cikarmasi TASMA PANIGI verir.
// Cozum: isaretli aritmetik (i64) + C'deki kirpma kontrolu.
// Ayrica C'deki n < 0 / n > 1000 girdi kontrolunu de koruyorum.
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
    if n < 0 || n > 1000 {
        return;
    }
    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let v: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        arr.push(v);
    }
    let k: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let mut start = n - k;
    if start < 0 {
        start = 0;
    }
    let mut i = start;
    while i < n {
        print!("{} ", arr[i as usize]);
        i += 1;
    }
    println!();
}

use std::io::{self, Read};

// IYILESTIRME (Round 2): Bu derleme ortaminda C'nin `long`i 32-bit'tir
// (Windows/LLP64). Ilk ceviri `long`i yaygin bir varsayimla 64-bit `i64`
// olarak secmisti; bu yuzden 32-bit sinirini asan toplamlarda hic kirpma
// yapmiyordu. Duzeltme: i64 yerine bu platformun gercek `long` genisligini
// yansitan i32 kullanildi (bkz. s38_bsd_strtol'deki ayni kok neden, SS3.7).
fn safe_add_clamped(a: i32, b: i32) -> i32 {
    let sum = a as f64 + b as f64;
    if sum > i32::MAX as f64 {
        return i32::MAX;
    }
    if sum < i32::MIN as f64 {
        return i32::MIN;
    }
    a + b
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    println!("{}", safe_add_clamped(a, b));
}

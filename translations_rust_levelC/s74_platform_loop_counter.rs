use std::io::{self, Read};

// Seviye C: yalnizca "2/4 test basarisiz" bilgisi var.
// C kaynagi `long` ve LONG_MAX/LONG_MIN ile kirpma yapiyor; kaynak yorumu
// bu ortamin Windows/LLP64 oldugunu, yani `long`un 32 BIT oldugunu
// soyluyor (LONG_MAX = 2147483647, LONG_MIN = -2147483648).
// Round 1 cevirisi i64 secmis; bu durumda 32-bit sinirinda gerceklesmesi
// gereken kirpma hic tetiklenmiyor.
// Cozum: i32 (+ kenar durumlarda panik olmamasi icin wrapping_mul).

fn safe_mul_clamped(a: i32, b: i32) -> i32 {
    let product = a as f64 * b as f64;
    if product > i32::MAX as f64 {
        return i32::MAX;
    }
    if product < i32::MIN as f64 {
        return i32::MIN;
    }
    a.wrapping_mul(b)
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();
    let a: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    let b: i64 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => return,
    };
    // scanf("%ld") 32-bit long'a okur; aralik disi degerler kirpilir.
    let a = a.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
    let b = b.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
    println!("{}", safe_mul_clamped(a, b));
}

// SEVIYE B duzeltme denemesi: modele yalnizca tam panik metni verildi
// ("attempt to subtract with overflow", satir 10) - beklenen/alinan cikti
// farki GOSTERILMEDI. Panik mesaji tasma turunu (cikarma/subtract) acikca
// belirttigi icin, dogrudan bir usize cikarma tasmasi supheleniliyor ve
// saturating_sub ile duzeltiliyor.
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let arr: Vec<i32> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();
    let mut diffsum = 0i32;
    for i in 0..n.saturating_sub(1) {
        diffsum += arr[i + 1] - arr[i];
    }
    println!("{}", diffsum);
}

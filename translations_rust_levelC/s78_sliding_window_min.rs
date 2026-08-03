use std::io::{self, Read};

// Seviye C: "4/4 test basarisiz" - yani program her girdide bozuluyor,
// bu da tipik olarak bir PANIK anlamina gelir (yanlis cikti genelde
// bazi testleri gecirir).
// C kaynagi: prev_idx = i - 1 KONTROLDEN ONCE hesaplaniyor; int oldugu
// icin i == 0 iken -1 olmasi sorun degil, sonra `prev_idx >= 0` ile
// eleniyor. Round 1 cevirisi i'yi usize yapmis: ilk yinelemede
// (i == 0) `i - 1` USIZE TASMASI ile panik verir.
// Cozum: isaretli aritmetik (i64) ve C'deki kontrol sirasinin korunmasi.
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
    if n < 2 || n > 1000 {
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
    let mut count: i32 = 0;
    for i in 0..n {
        let prev_idx = i - 1;
        let next_idx = i + 1;
        if prev_idx >= 0 && next_idx < n {
            if arr[i as usize] < arr[prev_idx as usize] && arr[i as usize] < arr[next_idx as usize]
            {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

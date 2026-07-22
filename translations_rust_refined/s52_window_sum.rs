use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let n = it.next().unwrap() as usize;
    let k = it.next().unwrap() as usize;
    let arr: Vec<i64> = (0..n).map(|_| it.next().unwrap()).collect();

    // IYILESTIRME (Round 2): C referansi `start = n - k` isaretli int ile
    // hesaplayip negatifse 0'a kirpiyordu. Ilk ceviri ayni cikarmayi usize
    // ile birebir yaptigi icin n < k oldugunda taşma panigi veriyordu.
    // Duzeltme: saturating_sub ile cikarma, usize altina hic inmiyor.
    let start = n.saturating_sub(k);

    let sum: i64 = arr[start..n].iter().sum();
    println!("{}", sum);
}

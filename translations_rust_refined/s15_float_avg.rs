use std::io::{self, Read};

// IYILESTIRME (Round 2): C'nin %g bicimi 6 anlamli basamak kullanir ve sondaki
// sifirlari atar. Ilk ceviri Rust'in varsayilan {} bicimini kullandigi icin
// tam basamak sayisi C'den farkli cikiyordu (FE). Asagidaki format_g, %g'yi taklit eder.
fn format_g(x: f64) -> String {
    if x == 0.0 {
        return "0".to_string();
    }
    let ax = x.abs();
    // d.ddd * 10^exp bicimindeki ustel (exponent); x<1 icin negatif olabilir.
    let exp = ax.log10().floor() as i32;
    let decimals = (6 - 1 - exp).max(0) as usize;
    let s = format!("{:.*}", decimals, x);
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut sum = 0.0f64;
    for _ in 0..n {
        let x: f64 = it.next().unwrap().parse().unwrap();
        sum += x;
    }
    let avg = if n > 0 { sum / n as f64 } else { 0.0 };
    println!("{}", format_g(avg));
}

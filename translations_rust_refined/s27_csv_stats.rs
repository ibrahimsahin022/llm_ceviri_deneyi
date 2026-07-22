use std::io::{self, Read};

// IYILESTIRME (Round 2): C'nin %g bicimi 6 anlamli basamak kullanir ve sondaki
// sifirlari atar. Ilk ceviri Rust'in varsayilan {} bicimini kullandigi icin
// tam basamak sayisi C'den farkli cikiyordu (FE, bkz. s15_float_avg ile ayni
// kok neden). Asagidaki format_g, %g'yi taklit eder.
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
    let mut lines = input.lines();

    let m: usize = lines.next().unwrap_or("0").trim().parse().unwrap_or(0);

    let mut total: i64 = 0;
    let mut count_all: i64 = 0;
    let mut have_min = false;
    let mut mn: i64 = 0;
    let mut mx: i64 = 0;
    let mut row_sums: Vec<i64> = Vec::new();

    for _ in 0..m {
        let line = match lines.next() {
            Some(l) => l,
            None => break,
        };
        let values: Vec<i64> = line
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i64>().unwrap_or(0))
            .collect();

        let mut row_sum: i64 = 0;
        for v in &values {
            row_sum += v;
            total += v;
            count_all += 1;
            if !have_min {
                mn = *v;
                mx = *v;
                have_min = true;
            } else {
                if *v < mn {
                    mn = *v;
                }
                if *v > mx {
                    mx = *v;
                }
            }
        }
        row_sums.push(row_sum);
    }

    let avg = if count_all > 0 {
        total as f64 / count_all as f64
    } else {
        0.0
    };

    println!("total={}", total);
    println!("avg={}", format_g(avg));
    if have_min {
        println!("min={} max={}", mn, mx);
    } else {
        println!("min=0 max=0");
    }
    for (i, s) in row_sums.iter().enumerate() {
        println!("row{}={}", i, s);
    }
}

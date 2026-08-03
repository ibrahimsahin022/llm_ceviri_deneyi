use std::io::{self, Read};

// Seviye C: "4/4 test basarisiz" - her girdide yanlis sonuc.
// Makroyu metinsel olarak actim. SWAP(arr[i++], arr[1], tmp) ->
//   do { tmp = arr[i++]; arr[i++] = arr[1]; arr[1] = tmp; } while (0)
// 'a' parametresi govdede IKI KEZ gecer, dolayisiyla i IKI KEZ artar ve
// ikinci erisim FARKLI bir elemana yazar:
//   i = 0: tmp = arr[0];  i -> 1
//          arr[1] = arr[1];  i -> 2      (kendine atama, etkisiz)
//          arr[1] = tmp;                 (arr[1] = eski arr[0])
// Yani takas OLMAZ: arr[0] degismez, arr[1] eski arr[0] olur ve i = 2.
// Round 1 cevirisi argumani BIR KEZ degerlendirmis (i sadece 1 artmis,
// gercek takas yapilmis) - hem dizi hem i satiri yanlis, bu da 4/4
// basarisizlikla tutarli.
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
    if n < 2 || n > 100 {
        return;
    }
    let n = n as usize;
    let mut arr: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i32 = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return,
        };
        arr.push(v);
    }

    let mut i: usize = 0;
    // tmp = arr[i++];
    let idx1 = i;
    i += 1;
    let tmp = arr[idx1];
    // arr[i++] = arr[1];
    let idx2 = i;
    i += 1;
    arr[idx2] = arr[1];
    // arr[1] = tmp;
    arr[1] = tmp;

    for v in &arr {
        print!("{} ", v);
    }
    println!();
    println!("i={}", i);
}

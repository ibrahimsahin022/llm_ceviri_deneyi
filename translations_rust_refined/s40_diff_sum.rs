use std::io::{self, Read};

// IYILESTIRME (Round 2): C kodu 'n' icin isaretli 'int' kullanir; n==0 iken
// "i < n-1" karsilastirmasi (0 < -1) guvenle yanlis olur, dongu hic calismaz.
// Ilk ceviri 'n'yi (dogal/idiyomatik bir secim olan) usize yaptigi icin,
// n==0 durumunda 'n - 1' usize altinda TASTI ve debug modda panik verdi (RE).
// Duzeltme: cikarma islemi saturating_sub ile yapilarak n==0 durumunda
// dongu araliginin bos (0..0) olmasi saglandi - boylece kaynak C'nin
// isaretli aritmetigiyle ayni sonuc (dongu hic calismaz) elde edildi.
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

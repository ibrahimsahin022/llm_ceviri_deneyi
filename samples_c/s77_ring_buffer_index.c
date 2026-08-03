#include <stdio.h>

/* Girdi: ilk satirda cap (tampon kapasitesi) ve n (islem sayisi),
   ardindan n satir: her biri "1" (ileri git) veya "0" (geri git).
   Cikti: her adimdan sonra mevcut yazma indeksi.
   NOT: Kok Neden G (usize tasmasi) icin 4. bagimsiz ornek. "Bir onceki
   indekse don" islemi C'de "(idx - 1 + cap) % cap" ile GUVENLE yazilir
   (idx=0 iken bile negatif olmaz). Rust'a cevrilirken usize ile ayni
   ifade `+ cap` korunmazsa idx=0 iken (idx - 1) TASMA PANIGI verir. */
int main(void) {
    int cap, n;
    if (scanf("%d %d", &cap, &n) != 2 || cap <= 0) return 0;
    int idx = 0;
    for (int t = 0; t < n; t++) {
        int op;
        if (scanf("%d", &op) != 1) return 0;
        if (op == 1) {
            idx = (idx + 1) % cap;
        } else {
            idx = (idx - 1 + cap) % cap;
        }
        printf("%d\n", idx);
    }
    return 0;
}

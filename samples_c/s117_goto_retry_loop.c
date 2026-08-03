#include <stdio.h>

/* Girdi: iki tam sayi - deger v, basari esigi threshold. Ayrica bir satir
   daha: n (deneme basina v'ye eklenecek artis miktari).
   Cikti: v, threshold'a ULASANA KADAR n eklenerek kac deneme yapildigi
   (v basitce >= threshold ise 0).
   NOT: `goto retry;` ile yeniden-deneme dongusu klasik ama artik "eski
   moda" sayilan bir C idiomudur (Linux cekirdegi ve bircok sistem
   programinda hala yaygin). Rust'ta `goto` YOK - dogru ceviri `loop {}`
   + `break`/`continue` gerektirir; yanlis cevrilirse ya SONSUZ DONGU ya
   da erken/gec durma olabilir. */

int main(void) {
    long v, threshold, step;
    if (scanf("%ld %ld", &v, &threshold) != 2) return 0;
    if (scanf("%ld", &step) != 1) return 0;
    if (step <= 0) return 0;

    int attempts = 0;

retry:
    if (v >= threshold) {
        goto done;
    }
    v += step;
    attempts++;
    if (attempts > 1000000) {
        goto done;
    }
    goto retry;

done:
    printf("attempts=%d final=%ld\n", attempts, v);
    return 0;
}

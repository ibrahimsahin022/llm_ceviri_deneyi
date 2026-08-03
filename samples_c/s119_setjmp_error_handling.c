#include <stdio.h>
#include <setjmp.h>

/* Girdi: ilk satirda n (islenecek deger sayisi), ardindan n tam sayi.
   Cikti: her deger icin, sifira bolme "hatasi" simule edilirse (deger==0)
   setjmp/longjmp ile checkpoint'e donulup "ERROR" yazdirilir; aksi halde
   100/deger sonucu yazdirilir. Sonda islenen (hataya dusmeyen) deger
   sayisi.
   NOT: setjmp/longjmp, C'nin "yapisal olmayan" (non-local) kontrol akisi
   mekanizmasidir - fonksiyonlar arasi ZIPLAMA yapar. Rust'ta DOGRUDAN
   karsiligi YOKTUR; dogal ceviri Result<T, E> + `?` operatorudur. Bu,
   "hata isleme felsefesi" farkinin (C: geriye-zipla, Rust: degeri
   yaymali/propagate) somut bir test edilebilir ornegidir. */

static jmp_buf checkpoint;

static int safe_divide(int numerator, int denominator) {
    if (denominator == 0) {
        longjmp(checkpoint, 1);
    }
    return numerator / denominator;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1 || n < 0 || n > 1000) return 0;

    int values[1000];
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &values[i]) != 1) return 0;
    }

    int processed = 0;
    for (int i = 0; i < n; i++) {
        if (setjmp(checkpoint) == 0) {
            int result = safe_divide(100, values[i]);
            printf("%d\n", result);
            processed++;
        } else {
            printf("ERROR\n");
        }
    }

    printf("processed=%d\n", processed);
    return 0;
}

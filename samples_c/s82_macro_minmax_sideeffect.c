#include <stdio.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

/* Girdi: iki tam sayi x, y.
   Cikti: "min max x" (MIN/MAX makrolari x++ gibi yan etkili bir ifadeyle
   cagrilir).
   NOT: Kok Neden I (makro coklu-degerlendirme) icin 2. bagimsiz ornek
   (s56'nin X-Macro'suz, yalnizca coklu-degerlendirme tuzagina odaklanan
   hali). C makrolari metinsel ikamedir; x++ gibi yan etkili bir arguman
   govde icinde BIRDEN FAZLA kez gecerse birden fazla kez calisir.
   Rust'ta fonksiyon/generic olarak cevrilen "min/max" argumanlarini TAM
   OLARAK BIR KEZ degerlendirir. */
int main(void) {
    int x, y;
    if (scanf("%d %d", &x, &y) != 2) return 0;
    int m = MIN(x++, y);
    int M = MAX(x++, y);
    printf("%d %d %d\n", m, M, x);
    return 0;
}

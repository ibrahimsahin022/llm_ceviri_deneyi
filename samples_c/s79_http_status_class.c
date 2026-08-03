#include <stdio.h>

/* Girdi: HTTP durum kodu (tam sayi).
   Cikti: kodun "sinif ozellikleri" bit toplami (dusmeli switch ile
   kumulatif). 5xx->8, 4xx dahil ek +4, 3xx dahil ek +2, 2xx dahil ek +1;
   1xx ve bilinmeyenler icin 0.
   NOT: Kok Neden H (switch dusmesi) icin 3. bagimsiz ornek (bkz. s43,
   s53). C'nin switch dusme (fallthrough) davranisi burada kumulatif bir
   puan olusturur. Rust'in match'i varsayilan olarak dusmez; LLM bunu
   acikca yeniden uretmezse sonuc yanlis olur. */
int main(void) {
    int code;
    if (scanf("%d", &code) != 1) return 0;
    int tier = code / 100;
    int score = 0;
    switch (tier) {
        case 5: score += 8; /* fallthrough */
        case 4: score += 4; /* fallthrough */
        case 3: score += 2; /* fallthrough */
        case 2: score += 1;
                break;
        default: score = 0;
    }
    printf("%d\n", score);
    return 0;
}

#include <stdio.h>

/* Girdi: tek satir metin.
   Cikti: "<min> <max>" (metindeki karakterlerin `char` degeri olarak min/max).
   NOT: Kok Neden C (char isaretliligi) icin 3. bagimsiz ornek (bkz. s20,
   s49 - onlar toplama/sayma yapiyordu, bu min/max karsilastirmasi yapiyor).
   Bu derleme ortaminda `char` ISARETLIDIR; 127'den buyuk baytlar negatif
   sayilir, bu yuzden min genellikle cok negatif bir deger olur. Dogal bir
   Rust cevirisi baytlari u8 (0..255) okursa min/max hep pozitif ve farkli
   cikar. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    int have = 0;
    char mn = 0, mx = 0;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '\n' || buf[i] == '\r') break;
        char c = buf[i];
        if (!have) {
            mn = mx = c;
            have = 1;
        } else {
            if (c < mn) mn = c;
            if (c > mx) mx = c;
        }
    }
    printf("%d %d\n", (int)mn, (int)mx);
    return 0;
}

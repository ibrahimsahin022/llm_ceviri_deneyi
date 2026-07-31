#include <stdio.h>

/* Girdi: tek satir metin.
   Cikti: metindeki tum karakterlerin XOR toplami (32-bit unsigned olarak
   yazdirilir).
   NOT: Kok Neden C (char isaretliligi) icin 5. bagimsiz ornek. `char`
   ISARETLIDIR; XOR'dan once int'e YUKSELTILIRKEN negatif bir char
   ISARET GENISLETMESI ile (ust bitler 1 ile doldurularak) int'e donusur.
   Rust'ta u8 -> u32 donusumu ise SIFIR GENISLETMESI yapar. Yuksek baytli
   girdilerde iki yaklasim farkli 32-bit sonuc uretir. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    unsigned int acc = 0;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '\n' || buf[i] == '\r') break;
        char c = buf[i];
        acc ^= (unsigned int)(int)c;
    }
    printf("%u\n", acc);
    return 0;
}

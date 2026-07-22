#include <stdio.h>

/* Girdi: tek satir metin.
   Cikti: metnin djb2 hash degeri (32-bit unsigned).
   NOT: Hesaplama sirasinda bilerek unsigned tasma (overflow) olur;
   C dilinde unsigned tasma tanimlidir ve modulo 2^32 sarar. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    unsigned int hash = 5381;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '\n' || buf[i] == '\r') break;
        hash = hash * 33u + (unsigned int)(unsigned char)buf[i];
    }
    printf("%u\n", hash);
    return 0;
}

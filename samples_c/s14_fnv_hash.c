#include <stdio.h>

/* Girdi: tek satir metin.
   Cikti: FNV-1a 32-bit hash degeri.
   Not: Carpim adiminda bilerek unsigned tasma olur (C'de tanimli, mod 2^32). */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    unsigned int hash = 2166136261u;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '\n' || buf[i] == '\r') break;
        hash ^= (unsigned int)(unsigned char)buf[i];
        hash *= 16777619u;
    }
    printf("%u\n", hash);
    return 0;
}

#include <stdio.h>

/* Girdi: tek satir metin.
   Cikti: sdbm karma degeri (32-bit unsigned).
   NOT: Kok Neden A (unsigned tasma) icin 5. bagimsiz ornek. sdbm formulu
   (hash = c + (hash<<6) + (hash<<16) - hash) birkac vardiyali toplama/
   cikarma iceriyor; kisa girdilerde bile 32-bit siniri hemen asilir. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    unsigned int hash = 0;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '\n' || buf[i] == '\r') break;
        unsigned int c = (unsigned int)(unsigned char)buf[i];
        hash = c + (hash << 6) + (hash << 16) - hash;
    }
    printf("%u\n", hash);
    return 0;
}

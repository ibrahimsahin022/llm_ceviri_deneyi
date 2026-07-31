#include <stdio.h>

/* Girdi: tek satir metin.
   Cikti: 31-tabanli polinom karma (h = h*31 + c), 32-bit unsigned.
   NOT: Kok Neden A (unsigned tasma) icin 4. bagimsiz ornek (bkz. s09, s14).
   Carpma h*31 birkac karakter sonra 32-bit siniri asar; C'de tanimli
   davranista mod 2^32 sarar, Rust'ta varsayilan (debug) derlemede panik verir. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    unsigned int h = 0;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '\n' || buf[i] == '\r') break;
        h = h * 31u + (unsigned int)(unsigned char)buf[i];
    }
    printf("%u\n", h);
    return 0;
}

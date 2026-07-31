#include <stdio.h>

/* Girdi: tek satir metin.
   Cikti: klasik ELF/PJW karma degeri (32-bit unsigned).
   NOT: Kok Neden A (unsigned tasma) icin 6. bagimsiz ornek. Ust 4 bitin
   maskeyle temizlenip XOR'lanmasi, vardiyali toplamalarla birlikte tasmaya
   ozellikle acik bir desendir. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    unsigned int h = 0, g;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '\n' || buf[i] == '\r') break;
        h = (h << 4) + (unsigned int)(unsigned char)buf[i];
        g = h & 0xF0000000u;
        if (g != 0) {
            h ^= g >> 24;
        }
        h &= ~g;
    }
    printf("%u\n", h);
    return 0;
}

#include <stdio.h>

/* Girdi: bir satir rakam dizesi.
   Cikti: dizenin `unsigned long` degerine ayristirilmis hali.
   NOT: Kok Neden F (platforma bagli tamsayi genisligi) icin 4. bagimsiz
   ornek (bkz. s38, s51). Elle yazilmis basit bir sayisal ayristirici,
   `unsigned long` biriktirici kullanir. Bu ortamda (Windows/LLP64)
   unsigned long 32 bittir (ULONG_MAX=4294967295); Linux/LP64'te 64
   bittir. Unsigned tasma C'de TANIMLIDIR (mod 2^n sarar) - dogal bir
   Rust cevirisi `unsigned long`u u64 secerse, 32-bit sinirinda
   gerceklesmesi gereken sarma HIC gerceklesmez. */
int main(void) {
    char buf[64];
    if (scanf("%63s", buf) != 1) return 0;
    unsigned long v = 0;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] < '0' || buf[i] > '9') break;
        v = v * 10UL + (unsigned long)(buf[i] - '0');
    }
    printf("%lu\n", v);
    return 0;
}

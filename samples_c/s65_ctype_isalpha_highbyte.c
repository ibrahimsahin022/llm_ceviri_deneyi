#include <stdio.h>
#include <ctype.h>

/* Girdi: tek satir metin.
   Cikti: "<alpha_sayisi> <digit_sayisi>".
   NOT: Kok Neden C (char isaretliligi) icin 4. bagimsiz ornek. isalpha()/
   isdigit() PLAIN `char` ile cagrilir (unsigned char'a donusturulmeden) -
   standartta bu, EOF disindaki negatif degerler icin tanimsizdir; bu
   derleme/kutuphane ortaminda (glibc/UCRT benzeri tablo tabanli ctype)
   deterministik ama platforma bagli sonuc uretir - kok nedeni C'nin diger
   bir yuzunu (kutuphane API sozlesmesindeki isaretlilik varsayimi) sinar. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    int alpha = 0, digit = 0;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '\n' || buf[i] == '\r') break;
        char c = buf[i];
        if (isalpha(c)) alpha++;
        if (isdigit(c)) digit++;
    }
    printf("%d %d\n", alpha, digit);
    return 0;
}

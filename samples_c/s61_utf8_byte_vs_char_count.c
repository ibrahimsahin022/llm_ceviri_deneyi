#include <stdio.h>

/* Girdi: tek satir metin (UTF-8 olabilir, Turkce karakterler icerebilir).
   Cikti: "<bayt_sayisi> <karakter_sayisi>".
   NOT: Kok Neden B (string modeli) icin 3. bagimsiz ornek (bkz. s06, s13).
   Bayt sayisi ile UTF-8 kod noktasi sayisi cok baytli girdide farklidir;
   "dogal" bir Rust cevirisi .len() (bayt) ile .chars().count() (kod noktasi)
   arasindaki farki gozden kacirabilir. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    int bytes = 0, chars = 0;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '\n' || buf[i] == '\r') break;
        unsigned char c = (unsigned char)buf[i];
        bytes++;
        if ((c & 0xC0) != 0x80) {
            chars++;
        }
    }
    printf("%d %d\n", bytes, chars);
    return 0;
}

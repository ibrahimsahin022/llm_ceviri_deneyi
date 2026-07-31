#include <stdio.h>
#include <string.h>

/* Girdi: tek satir metin.
   Cikti: "EVET" (bayt dizisi kendinin tersiyse) veya "HAYIR".
   NOT: Kok Neden B (string modeli) icin 5. bagimsiz ornek. Karsilastirma
   dogrudan BAYT indeksiyle yapilir (buf[i] vs buf[len-1-i]); cok baytli
   UTF-8 girdide bu, kod noktasi bazinda simetrik olan bir metni bile
   bayt duzeyinde asimetrik gosterebilir - Rust'ta .chars().rev() kullanan
   "dogal" bir ceviri farkli sonuc verir. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    size_t len = strlen(buf);
    while (len > 0 && (buf[len - 1] == '\n' || buf[len - 1] == '\r')) {
        len--;
    }
    int ok = 1;
    for (size_t i = 0; i < len / 2; i++) {
        if (buf[i] != buf[len - 1 - i]) {
            ok = 0;
            break;
        }
    }
    printf("%s\n", ok ? "EVET" : "HAYIR");
    return 0;
}

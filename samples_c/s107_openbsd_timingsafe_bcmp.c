#include <stdio.h>
#include <string.h>

/* Kaynak: OpenBSD src/lib/libc/string/timingsafe_bcmp.c - birebir (DEF_WEAK
   satiri haric).
   https://raw.githubusercontent.com/openbsd/src/master/lib/libc/string/timingsafe_bcmp.c
   Lisans: ISC-benzeri (Damien Miller).
   NOT: Sabit-zamanli karsilastirma - erken donus YOK, ilk farkta durmaz
   (yan-kanal zamanlama saldirilarina karsi korunma). `ret |= ...` biriktirme
   deseni, Rust'a "erken donuslu" (early-return) bir karsilastirmaya
   cevrilirse GUVENLIK OZELLIGI kaybolur (fonksiyonel olarak ayni sonucu
   verir ama sabit-zamanlilik garantisi bozulur - bu testte GOZLENEMEZ
   ama gercek dunyada onemli bir sessiz semantik kayiptir).
   Girdi: iki satir, ESIT UZUNLUKTA metin.
   Cikti: "0" (esit) veya "1" (farkli). */

int my_timingsafe_bcmp(const void *b1, const void *b2, size_t n) {
    const unsigned char *p1 = b1, *p2 = b2;
    int ret = 0;

    for (; n > 0; n--)
        ret |= *p1++ ^ *p2++;
    return (ret != 0);
}

int main(void) {
    char a[256], b[256];
    if (!fgets(a, sizeof(a), stdin)) return 0;
    if (!fgets(b, sizeof(b), stdin)) return 0;
    a[strcspn(a, "\r\n")] = 0;
    b[strcspn(b, "\r\n")] = 0;
    size_t n = strlen(a);
    if (strlen(b) < n) n = strlen(b);
    int r = my_timingsafe_bcmp(a, b, n);
    printf("%d\n", r);
    return 0;
}

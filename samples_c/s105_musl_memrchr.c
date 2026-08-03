#include <stdio.h>
#include <string.h>

/* Kaynak: musl libc src/string/memrchr.c - birebir (weak_alias satiri
   haric).
   https://raw.githubusercontent.com/kraj/musl/master/src/string/memrchr.c
   Lisans: MIT.
   NOT: `c = (unsigned char)c;` satiri Kok Neden C (char isaretliligi) ile
   dogrudan ilgilidir - int parametresi once unsigned char'a kirpiliyor,
   boylece s[n]==c karsilastirmasi negatif char degerleriyle bile dogru
   calisiyor.
   Girdi: iki satir - metin, aranan tek karakter.
   Cikti: metindeki SONDAN ilk eslesmenin 0-tabanli indeksi, yoksa -1. */

void *my_memrchr(const void *m, int c, size_t n) {
    const unsigned char *s = m;
    c = (unsigned char)c;
    while (n--) if (s[n] == c) return (void *)(s + n);
    return 0;
}

int main(void) {
    char text[512], ch[8];
    if (!fgets(text, sizeof(text), stdin)) return 0;
    if (!fgets(ch, sizeof(ch), stdin)) return 0;
    text[strcspn(text, "\r\n")] = 0;
    ch[strcspn(ch, "\r\n")] = 0;
    if (ch[0] == 0) return 0;

    void *r = my_memrchr(text, (unsigned char)ch[0], strlen(text));
    if (r) {
        printf("%ld\n", (long)((char *)r - text));
    } else {
        printf("-1\n");
    }
    return 0;
}

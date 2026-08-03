#include <stdio.h>
#include <string.h>
#include <strings.h>

/* Kaynak: musl libc src/string/strcasestr.c - birebir.
   https://raw.githubusercontent.com/kraj/musl/master/src/string/strcasestr.c
   Lisans: MIT.
   Girdi: iki satir - h (aranacak metin), n (aranan, buyuk/kucuk harf
   duyarsiz).
   Cikti: ilk eslesmenin 0-tabanli indeksi, bulunamazsa -1. */

char *my_strcasestr(const char *h, const char *n) {
    size_t l = strlen(n);
    if (!l) return (char *)h;
    for (; *h; h++) if (!strncasecmp(h, n, l)) return (char *)h;
    return 0;
}

int main(void) {
    char hay[512], needle[256];
    if (!fgets(hay, sizeof(hay), stdin)) return 0;
    if (!fgets(needle, sizeof(needle), stdin)) return 0;
    hay[strcspn(hay, "\r\n")] = 0;
    needle[strcspn(needle, "\r\n")] = 0;
    char *r = my_strcasestr(hay, needle);
    if (r) {
        printf("%ld\n", (long)(r - hay));
    } else {
        printf("-1\n");
    }
    return 0;
}

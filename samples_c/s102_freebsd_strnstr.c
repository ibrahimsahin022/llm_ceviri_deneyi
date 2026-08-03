#include <stdio.h>
#include <string.h>

/* Kaynak: FreeBSD src/lib/libc/string/strnstr.c - birebir.
   https://raw.githubusercontent.com/freebsd/freebsd-src/main/lib/libc/string/strnstr.c
   Lisans: BSD-3-Clause.
   Girdi: uc satir - s (arama yapilacak metin), find (aranan alt-dize),
   slen (arama sinirinin s'nin ilk kac karakteriyle sinirlandigi, ondalik).
   Cikti: bulunan konumun 0-tabanli indeksi, bulunamazsa -1. */

char *my_strnstr(const char *s, const char *find, size_t slen) {
    char c, sc;
    size_t len;

    if ((c = *find++) != '\0') {
        len = strlen(find);
        do {
            do {
                if (slen-- < 1 || (sc = *s++) == '\0')
                    return NULL;
            } while (sc != c);
            if (len > slen)
                return NULL;
        } while (strncmp(s, find, len) != 0);
        s--;
    }
    return (char *)s;
}

int main(void) {
    char s[512], find[256];
    int slen;
    if (!fgets(s, sizeof(s), stdin)) return 0;
    if (!fgets(find, sizeof(find), stdin)) return 0;
    if (scanf("%d", &slen) != 1) return 0;
    s[strcspn(s, "\r\n")] = 0;
    find[strcspn(find, "\r\n")] = 0;
    if (slen < 0) slen = 0;

    char *r = my_strnstr(s, find, (size_t)slen);
    if (r) {
        printf("%ld\n", (long)(r - s));
    } else {
        printf("-1\n");
    }
    return 0;
}

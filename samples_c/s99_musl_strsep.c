#include <stdio.h>
#include <string.h>

/* Kaynak: musl libc src/string/strsep.c - birebir.
   https://raw.githubusercontent.com/kraj/musl/master/src/string/strsep.c
   Lisans: MIT.
   Girdi: iki satir - once ayrac karakter kumesi (orn. ",;"), sonra
   ayrilacak metin.
   Cikti: her token, kendi satirinda (strsep ile NULL donene kadar
   cagrilir). */

char *my_strsep(char **str, const char *sep) {
    char *s = *str, *end;
    if (!s) return NULL;
    end = s + strcspn(s, sep);
    if (*end) *end++ = 0;
    else end = 0;
    *str = end;
    return s;
}

int main(void) {
    char sep[64], text[512];
    if (!fgets(sep, sizeof(sep), stdin)) return 0;
    if (!fgets(text, sizeof(text), stdin)) return 0;
    sep[strcspn(sep, "\r\n")] = 0;
    text[strcspn(text, "\r\n")] = 0;

    char *p = text;
    char *tok;
    while ((tok = my_strsep(&p, sep)) != NULL) {
        printf("%s\n", tok);
    }
    return 0;
}

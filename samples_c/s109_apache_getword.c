#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/* Kaynak: Apache HTTP Server server/util.c, ap_getword() - birebir mantik
   (apr_pool_t/apr_pstrmemdup soyutlamalari, havuzsuz duz malloc+strncpy ile
   degistirildi; islevsel davranis AYNI).
   https://raw.githubusercontent.com/apache/httpd/trunk/server/util.c
   Lisans: Apache-2.0.
   Girdi: iki satir - durdurma karakteri (tek karakter) ve ayristirilacak
   metin.
   Cikti: metindeki ARDISIK "kelime" alanlari, her biri kendi satirinda,
   metin tukenene kadar tekrar tekrar cikarilir (gercek ap_getword'un
   cagri-donguleriyle ayni sekilde). */

char *my_getword(const char **line, char stop) {
    const char *pos = *line;
    int len;
    char *res;

    while ((*pos != stop) && *pos) {
        ++pos;
    }

    len = (int)(pos - *line);
    res = malloc(len + 1);
    memcpy(res, *line, len);
    res[len] = 0;

    if (stop) {
        while (*pos == stop) {
            ++pos;
        }
    }
    *line = pos;

    return res;
}

int main(void) {
    char stopc[8], text[512];
    if (!fgets(stopc, sizeof(stopc), stdin)) return 0;
    if (!fgets(text, sizeof(text), stdin)) return 0;
    stopc[strcspn(stopc, "\r\n")] = 0;
    text[strcspn(text, "\r\n")] = 0;
    if (stopc[0] == 0) return 0;

    const char *p = text;
    while (*p) {
        char *w = my_getword(&p, stopc[0]);
        printf("%s\n", w);
        free(w);
    }
    return 0;
}

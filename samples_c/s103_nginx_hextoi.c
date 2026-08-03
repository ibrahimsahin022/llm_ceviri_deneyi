#include <stdio.h>
#include <string.h>

/* Kaynak: nginx src/core/ngx_string.c, ngx_hextoi() - birebir.
   https://raw.githubusercontent.com/nginx/nginx/master/src/core/ngx_string.c
   Lisans: BSD-2-Clause.
   DEGISIKLIK: ngx_int_t/u_char tipleri standart C tiplerine (long/unsigned
   char) esitlendi; NGX_ERROR (-1) ve NGX_MAX_INT_T_VALUE (platformun
   LONG_MAX'i) yerel olarak tanimlandi. Govde DEGISTIRILMEDEN alindi.
   NOT: `value > cutoff` tasma-onleme kontrolu, Kok Neden A/G ile ilgilidir -
   Rust'a "value * 16 + digit" olarak DOGRUDAN (kontrolsuz) cevrilirse
   buyuk girdilerde TASMA PANIGI verir.
   Girdi: bir satir, hex rakamlarindan olusan metin (0-9, a-f, A-F -
   onek "0x" YOK).
   Cikti: cozumlenen deger (ondalik) veya -1 (gecersiz/tasma). */

#define NGX_ERROR (-1)
#define NGX_MAX_INT_T_VALUE LONG_MAX
#include <limits.h>

long ngx_hextoi(const unsigned char *line, size_t n) {
    unsigned char c, ch;
    long value, cutoff;

    if (n == 0) {
        return NGX_ERROR;
    }

    cutoff = NGX_MAX_INT_T_VALUE / 16;

    for (value = 0; n--; line++) {
        if (value > cutoff) {
            return NGX_ERROR;
        }

        ch = *line;

        if (ch >= '0' && ch <= '9') {
            value = value * 16 + (ch - '0');
            continue;
        }

        c = (unsigned char)(ch | 0x20);

        if (c >= 'a' && c <= 'f') {
            value = value * 16 + (c - 'a' + 10);
            continue;
        }

        return NGX_ERROR;
    }

    return value;
}

int main(void) {
    char line[128];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    long r = ngx_hextoi((const unsigned char *)line, strlen(line));
    printf("%ld\n", r);
    return 0;
}

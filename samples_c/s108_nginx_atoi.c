#include <stdio.h>
#include <string.h>
#include <limits.h>

/* Kaynak: nginx src/core/ngx_string.c, ngx_atoi() - birebir.
   https://raw.githubusercontent.com/nginx/nginx/master/src/core/ngx_string.c
   Lisans: BSD-2-Clause.
   DEGISIKLIK: ngx_int_t -> long, NGX_ERROR -> -1, NGX_MAX_INT_T_VALUE ->
   LONG_MAX. Govde DEGISTIRILMEDEN alindi.
   NOT: s103 (ngx_hextoi) ile ayni dosyadan, 10-tabanli kardesi; "cutoff/
   cutlim" ikili tasma-onleme deseni Kok Neden A/G icin ayri bir varyant.
   Girdi: bir satir, ondalik rakamlardan olusan metin (onek/isaret YOK).
   Cikti: cozumlenen deger veya -1 (gecersiz/tasma). */

long ngx_atoi(const unsigned char *line, size_t n) {
    long value, cutoff, cutlim;

    if (n == 0) {
        return -1;
    }

    cutoff = LONG_MAX / 10;
    cutlim = LONG_MAX % 10;

    for (value = 0; n--; line++) {
        if (*line < '0' || *line > '9') {
            return -1;
        }

        if (value >= cutoff && (value > cutoff || *line - '0' > cutlim)) {
            return -1;
        }

        value = value * 10 + (*line - '0');
    }

    return value;
}

int main(void) {
    char line[64];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    long r = ngx_atoi((const unsigned char *)line, strlen(line));
    printf("%ld\n", r);
    return 0;
}

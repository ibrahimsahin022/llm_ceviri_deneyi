#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <errno.h>

/* Kaynak: OpenBSD src/lib/libc/stdlib/reallocarray.c (birebir, DEF_WEAK
   satiri haric).
   https://raw.githubusercontent.com/openbsd/src/master/lib/libc/stdlib/reallocarray.c
   Telif: Copyright (c) 2008 Otto Moerbeek. Lisans: ISC-benzeri (OpenBSD izin
   veren lisans metni).
   Bu, s103 icin planlanan orijinal OSS kaynagiydi; Grup 2 sirasi
   duzenlenirken s94 slotuna alindi (Kok Neden A ile guclu ortusme:
   MUL_NO_OVERFLOW = sqrt(SIZE_MAX+1) esigiyle CARPMA TASMASINI proaktif
   tespit eden klasik, gercek uretim kodu deseni).
   Girdi: iki satir - nmemb ve size (ikisi de size_t, ondalik).
   Cikti: tasma tespit edilirse "OVERFLOW", edilmezse "OK size=<nmemb*size>". */

#define MUL_NO_OVERFLOW ((size_t)1 << (sizeof(size_t) * 4))

void *reallocarray(void *optr, size_t nmemb, size_t size) {
    if ((nmemb >= MUL_NO_OVERFLOW || size >= MUL_NO_OVERFLOW) &&
        nmemb > 0 && SIZE_MAX / nmemb < size) {
        errno = ENOMEM;
        return NULL;
    }
    return realloc(optr, size * nmemb);
}

int main(void) {
    unsigned long long nmemb, size;
    if (scanf("%llu %llu", &nmemb, &size) != 2) return 0;
    void *p = reallocarray(NULL, (size_t)nmemb, (size_t)size);
    if (p == NULL) {
        printf("OVERFLOW\n");
    } else {
        printf("OK size=%llu\n", nmemb * size);
        free(p);
    }
    return 0;
}

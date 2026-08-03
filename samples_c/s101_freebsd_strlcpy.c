#include <stdio.h>
#include <string.h>

/* Kaynak: FreeBSD/OpenBSD src/lib/libc/string/strlcpy.c - birebir.
   https://raw.githubusercontent.com/freebsd/freebsd-src/main/lib/libc/string/strlcpy.c
   Lisans: BSD-3-Clause (OpenBSD kokenli, Todd C. Miller).
   Girdi: iki satir - dsize (hedef tampon boyutu, ondalik) ve src (kopyalanacak
   metin).
   Cikti: "dst=<kirpilmis-veya-tam-kopya> ret=<src'nin tam uzunlugu>" (ret,
   dsize'dan BUYUK/ESIT olursa KIRPILMA oldugunu gosterir - klasik strlcpy
   sozlesmesi). */

size_t my_strlcpy(char * restrict dst, const char * restrict src, size_t dsize) {
    const char *osrc = src;
    size_t nleft = dsize;

    if (nleft != 0) {
        while (--nleft != 0) {
            if ((*dst++ = *src++) == '\0')
                break;
        }
    }

    if (nleft == 0) {
        if (dsize != 0)
            *dst = '\0';
        while (*src++)
            ;
    }

    return (src - osrc - 1);
}

int main(void) {
    int dsize;
    char src[512];
    if (scanf("%d", &dsize) != 1) return 0;
    int c = getchar();
    (void)c;
    if (!fgets(src, sizeof(src), stdin)) return 0;
    src[strcspn(src, "\r\n")] = 0;

    char dst[600];
    if (dsize < 0) dsize = 0;
    if (dsize > (int)sizeof(dst)) dsize = (int)sizeof(dst);
    size_t ret = my_strlcpy(dst, src, (size_t)dsize);
    printf("dst=%s ret=%zu\n", dsize > 0 ? dst : "", ret);
    return 0;
}

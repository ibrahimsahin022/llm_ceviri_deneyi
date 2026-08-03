#include <stdio.h>
#include <ctype.h>
#include <string.h>

/* Kaynak: musl libc src/string/strverscmp.c - birebir.
   https://raw.githubusercontent.com/kraj/musl/master/src/string/strverscmp.c
   Lisans: MIT.
   Girdi: iki satir (l0, r0).
   Cikti: strverscmp'nin dondurdugu HAM int fark degeri (s85 ile ayni
   yontem - normallestirilmis -1/0/1 DEGIL). */

int my_strverscmp(const char *l0, const char *r0) {
    const unsigned char *l = (const void *)l0;
    const unsigned char *r = (const void *)r0;
    size_t i, dp, j;
    int z = 1;

    for (dp=i=0; l[i]==r[i]; i++) {
        int c = l[i];
        if (!c) return 0;
        if (!isdigit(c)) dp=i+1, z=1;
        else if (c!='0') z=0;
    }

    if (l[dp]-'1'<9U && r[dp]-'1'<9U) {
        for (j=i; isdigit(l[j]); j++)
            if (!isdigit(r[j])) return 1;
        if (isdigit(r[j])) return -1;
    } else if (z && dp<i && (isdigit(l[i]) || isdigit(r[i]))) {
        return (unsigned char)(l[i]-'0') - (unsigned char)(r[i]-'0');
    }

    return l[i] - r[i];
}

int main(void) {
    char left[256], right[256];
    if (!fgets(left, sizeof(left), stdin)) return 0;
    if (!fgets(right, sizeof(right), stdin)) return 0;
    left[strcspn(left, "\r\n")] = 0;
    right[strcspn(right, "\r\n")] = 0;
    printf("%d\n", my_strverscmp(left, right));
    return 0;
}

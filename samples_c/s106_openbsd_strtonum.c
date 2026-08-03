#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>
#include <limits.h>

/* Kaynak: OpenBSD src/lib/libc/stdlib/strtonum.c - birebir (DEF_WEAK
   satiri haric).
   https://raw.githubusercontent.com/openbsd/src/master/lib/libc/stdlib/strtonum.c
   Lisans: ISC-benzeri (Ted Unangst / Todd Miller).
   Girdi: uc satir - numstr (ayristirilacak metin), minval, maxval (ikisi
   de long long, ondalik).
   Cikti: "value=<v> err=<hata-mesaji-veya-yok>" (hata yoksa "err=yok"). */

#define INVALID  1
#define TOOSMALL 2
#define TOOLARGE 3

long long my_strtonum(const char *numstr, long long minval, long long maxval,
    const char **errstrp)
{
    long long ll = 0;
    int error = 0;
    char *ep;
    struct errval {
        const char *errstr;
        int err;
    } ev[4] = {
        { NULL,       0 },
        { "invalid",   EINVAL },
        { "too small", ERANGE },
        { "too large", ERANGE },
    };

    ev[0].err = errno;
    errno = 0;
    if (minval > maxval) {
        error = INVALID;
    } else {
        ll = strtoll(numstr, &ep, 10);
        if (numstr == ep || *ep != '\0')
            error = INVALID;
        else if ((ll == LLONG_MIN && errno == ERANGE) || ll < minval)
            error = TOOSMALL;
        else if ((ll == LLONG_MAX && errno == ERANGE) || ll > maxval)
            error = TOOLARGE;
    }
    if (errstrp != NULL)
        *errstrp = ev[error].errstr;
    errno = ev[error].err;
    if (error)
        ll = 0;

    return ll;
}

int main(void) {
    char numstr[64];
    long long minval, maxval;
    if (!fgets(numstr, sizeof(numstr), stdin)) return 0;
    numstr[strcspn(numstr, "\r\n")] = 0;
    if (scanf("%lld %lld", &minval, &maxval) != 2) return 0;

    const char *errstr = NULL;
    long long v = my_strtonum(numstr, minval, maxval, &errstr);
    printf("value=%lld err=%s\n", v, errstr ? errstr : "yok");
    return 0;
}

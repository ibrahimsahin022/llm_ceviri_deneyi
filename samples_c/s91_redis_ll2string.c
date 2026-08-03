#include <stdio.h>
#include <limits.h>
#include <string.h>

/* Kaynak: Redis src/util.c, ll2string() (birebir).
   https://raw.githubusercontent.com/redis/redis/unstable/src/util.c
   Lisans: BSD-3-Clause.
   DEGISIKLIK: ll2string'in cagirdigi ull2string() (iki-hane ondalik arama
   tablosuyla hizlandirilmis donusum) performans-odakli oldugu icin BASIT
   bir rakam-cikarma donguswiyle degistirildi (sonuc birebir ayni bayt
   dizisini uretir); ll2string GOVDESI - ozellikle LLONG_MIN icin ozel
   isaretli-tasma-guvenli dal (svalue != LLONG_MIN kontrolu) - DEGISTIRILMEDEN
   alindi. Bu ozel dal, Kok Neden A/G ile dogrudan ilgilidir: "-svalue" ifadesi
   svalue == LLONG_MIN iken TANIMSIZ DAVRANIS/tasmadir (2'nin tumleyeninde
   LLONG_MIN'in pozitif esdegeri LLONG_MAX'i asar); Redis bunu acikca
   LLONG_MAX+1 ile telafi eder. Rust'a naif cevrilirse (`-svalue` veya
   `.abs()`) i64::MIN girdisinde PANIK verir.
   Girdi: bir satir, imzali 64-bit tam sayi (orn. -9223372036854775808).
   Cikti: ayni sayinin ondalik metin hali. */

static int ull2string(char *dst, size_t dstlen, unsigned long long value) {
    char tmp[20];
    int len = 0;
    if (value == 0) {
        tmp[len++] = '0';
    } else {
        while (value > 0) {
            tmp[len++] = (char)('0' + (value % 10));
            value /= 10;
        }
    }
    if (dstlen < (size_t)len) return 0;
    for (int i = 0; i < len; i++) {
        dst[i] = tmp[len - 1 - i];
    }
    return len;
}

int ll2string(char *dst, size_t dstlen, long long svalue) {
    unsigned long long value;
    int negative = 0;

    if (svalue < 0) {
        if (svalue != LLONG_MIN) {
            value = -svalue;
        } else {
            value = ((unsigned long long) LLONG_MAX)+1;
        }
        if (dstlen < 2) return 0;
        negative = 1;
        dst[0] = '-';
        dst++;
        dstlen--;
    } else {
        value = svalue;
    }

    int length = ull2string(dst, dstlen, value);
    if (length == 0) return 0;
    return length + negative;
}

int main(void) {
    long long v;
    if (scanf("%lld", &v) != 1) return 0;
    char buf[32];
    int n = ll2string(buf, sizeof(buf), v);
    buf[n] = 0;
    printf("%s\n", buf);
    return 0;
}

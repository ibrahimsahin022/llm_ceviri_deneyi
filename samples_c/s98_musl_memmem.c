#include <stdio.h>
#include <stdint.h>
#include <string.h>

/* Kaynak: musl libc src/string/memmem.c - TAMAMI birebir (twobyte/threebyte/
   fourbyte hizli yollar + twoway_memmem Two-Way dize eslestirme algoritmasi).
   https://raw.githubusercontent.com/kraj/musl/master/src/string/memmem.c
   Lisans: MIT.
   NOT: Bu, Grup 2'deki EN UZUN/EN KARMASIK gercek fonksiyondur (~120 satir,
   bit-kumesi tablosu + periyodiklik analizi). Kasitli olarak "zor" bir
   ceviri testi: dogru cevrilmesi hem usize aritmetigi (Kok Neden G ile
   guclu kesisim - `ip = -1` gibi size_t alt-tasmasi ICIN KASITLI kullanilan
   bir C deyimi var!) hem de dogru bit-duzeyi mantik gerektirir.
   Girdi: iki satir - saman ligi (haystack), aranan (needle).
   Cikti: ilk eslesmenin 0-tabanli indeksi, bulunamazsa -1. */

static char *twobyte_memmem(const unsigned char *h, size_t k, const unsigned char *n) {
    uint16_t nw = n[0]<<8 | n[1], hw = h[0]<<8 | h[1];
    for (h+=2, k-=2; k; k--, hw = hw<<8 | *h++)
        if (hw == nw) return (char *)h-2;
    return hw == nw ? (char *)h-2 : 0;
}

static char *threebyte_memmem(const unsigned char *h, size_t k, const unsigned char *n) {
    uint32_t nw = (uint32_t)n[0]<<24 | n[1]<<16 | n[2]<<8;
    uint32_t hw = (uint32_t)h[0]<<24 | h[1]<<16 | h[2]<<8;
    for (h+=3, k-=3; k; k--, hw = (hw|*h++)<<8)
        if (hw == nw) return (char *)h-3;
    return hw == nw ? (char *)h-3 : 0;
}

static char *fourbyte_memmem(const unsigned char *h, size_t k, const unsigned char *n) {
    uint32_t nw = (uint32_t)n[0]<<24 | n[1]<<16 | n[2]<<8 | n[3];
    uint32_t hw = (uint32_t)h[0]<<24 | h[1]<<16 | h[2]<<8 | h[3];
    for (h+=4, k-=4; k; k--, hw = hw<<8 | *h++)
        if (hw == nw) return (char *)h-4;
    return hw == nw ? (char *)h-4 : 0;
}

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))

#define BITOP(a,b,op) \
 ((a)[(size_t)(b)/(8*sizeof *(a))] op (size_t)1<<((size_t)(b)%(8*sizeof *(a))))

static char *twoway_memmem(const unsigned char *h, const unsigned char *z, const unsigned char *n, size_t l) {
    size_t i, ip, jp, k, p, ms, p0, mem, mem0;
    size_t byteset[32 / sizeof(size_t)] = { 0 };
    size_t shift[256];

    for (i=0; i<l; i++)
        BITOP(byteset, n[i], |=), shift[n[i]] = i+1;

    ip = -1; jp = 0; k = p = 1;
    while (jp+k<l) {
        if (n[ip+k] == n[jp+k]) {
            if (k == p) {
                jp += p;
                k = 1;
            } else k++;
        } else if (n[ip+k] > n[jp+k]) {
            jp += k;
            k = 1;
            p = jp - ip;
        } else {
            ip = jp++;
            k = p = 1;
        }
    }
    ms = ip;
    p0 = p;

    ip = -1; jp = 0; k = p = 1;
    while (jp+k<l) {
        if (n[ip+k] == n[jp+k]) {
            if (k == p) {
                jp += p;
                k = 1;
            } else k++;
        } else if (n[ip+k] < n[jp+k]) {
            jp += k;
            k = 1;
            p = jp - ip;
        } else {
            ip = jp++;
            k = p = 1;
        }
    }
    if (ip+1 > ms+1) ms = ip;
    else p = p0;

    if (memcmp(n, n+p, ms+1)) {
        mem0 = 0;
        p = MAX(ms, l-ms-1) + 1;
    } else mem0 = l-p;
    mem = 0;

    for (;;) {
        if ((size_t)(z-h) < l) return 0;

        if (BITOP(byteset, h[l-1], &)) {
            k = l-shift[h[l-1]];
            if (k) {
                if (k < mem) k = mem;
                h += k;
                mem = 0;
                continue;
            }
        } else {
            h += l;
            mem = 0;
            continue;
        }

        for (k=MAX(ms+1,mem); k<l && n[k] == h[k]; k++);
        if (k < l) {
            h += k-ms;
            mem = 0;
            continue;
        }
        for (k=ms+1; k>mem && n[k-1] == h[k-1]; k--);
        if (k <= mem) return (char *)h;
        h += p;
        mem = mem0;
    }
}

void *my_memmem(const void *h0, size_t k, const void *n0, size_t l) {
    const unsigned char *h = h0, *n = n0;

    if (!l) return (void *)h;
    if (k<l) return 0;

    h = memchr(h0, *n, k);
    if (!h || l==1) return (void *)h;
    k -= h - (const unsigned char *)h0;
    if (k<l) return 0;
    if (l==2) return twobyte_memmem(h, k, n);
    if (l==3) return threebyte_memmem(h, k, n);
    if (l==4) return fourbyte_memmem(h, k, n);

    return twoway_memmem(h, h+k, n, l);
}

int main(void) {
    char hay[512], needle[512];
    if (!fgets(hay, sizeof(hay), stdin)) return 0;
    if (!fgets(needle, sizeof(needle), stdin)) return 0;
    hay[strcspn(hay, "\r\n")] = 0;
    needle[strcspn(needle, "\r\n")] = 0;
    void *r = my_memmem(hay, strlen(hay), needle, strlen(needle));
    if (r) {
        printf("%ld\n", (long)((char *)r - hay));
    } else {
        printf("-1\n");
    }
    return 0;
}

#include <stdio.h>
#include <string.h>

/* Kaynak: zlib adler32.c, adler32_z()/adler32() - cekirdek algoritma.
   https://raw.githubusercontent.com/madler/zlib/develop/adler32.c
   Telif: Copyright (C) 1995-2011, 2016 Mark Adler. Lisans: zlib License.
   DEGISIKLIK: SIMD/derleyiciye-ozgu DO1/DO16 acilim makrolari ve
   NO_DIVIDE optimizasyon yolu CIKARILDI (yalniz performans icin, sonucu
   etkilemez); BASE=65521 ve NMAX=5552 sabitleri ile "NMAX bloklarinda
   iki toplam biriktir, sonra mod al" cekirdek algoritmasi BIREBIR
   korundu (bu, s58_adler32_checksum'daki basitlestirilmis ozgun anlatimdan
   FARKLI OLARAK, zlib'in gercek bloklama stratejisini yansitir - Kok Neden
   A icin ayrica ilginctir cunku "sum2" degeri de u32 sinirlarinda tasabilir).
   Girdi: bir satir metin (bayt dizisi olarak islenir).
   Cikti: adler32 sonucu, ondalik (unsigned long). */

#define BASE 65521U
#define NMAX 5552

static unsigned long adler32_z(unsigned long adler, const unsigned char *buf, size_t len) {
    unsigned long sum2;
    unsigned n;

    sum2 = (adler >> 16) & 0xffff;
    adler &= 0xffff;

    if (len == 1) {
        adler += buf[0];
        if (adler >= BASE) adler -= BASE;
        sum2 += adler;
        if (sum2 >= BASE) sum2 -= BASE;
        return adler | (sum2 << 16);
    }

    if (buf == NULL) return 1L;

    if (len < 16) {
        while (len--) {
            adler += *buf++;
            sum2 += adler;
        }
        if (adler >= BASE) adler -= BASE;
        sum2 %= BASE;
        return adler | (sum2 << 16);
    }

    while (len >= NMAX) {
        len -= NMAX;
        n = NMAX / 16;
        do {
            for (int k = 0; k < 16; k++) {
                adler += *buf++;
                sum2 += adler;
            }
        } while (--n);
        adler %= BASE;
        sum2 %= BASE;
    }

    if (len) {
        while (len >= 16) {
            len -= 16;
            for (int k = 0; k < 16; k++) {
                adler += *buf++;
                sum2 += adler;
            }
        }
        while (len--) {
            adler += *buf++;
            sum2 += adler;
        }
        adler %= BASE;
        sum2 %= BASE;
    }

    return adler | (sum2 << 16);
}

int main(void) {
    char line[4096];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    unsigned long result = adler32_z(1L, (const unsigned char *)line, strlen(line));
    printf("%lu\n", result);
    return 0;
}

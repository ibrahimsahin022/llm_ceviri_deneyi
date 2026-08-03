#include <stdio.h>
#include <string.h>

/* Kaynak: OpenSSL crypto/evp/encode.c, evp_encodeblock_int() /
   EVP_EncodeBlock() (OpenSSL_1_1_1w etiketi, birebir; AVX2/SIMD yollari
   olmayan klasik surum).
   https://raw.githubusercontent.com/openssl/openssl/OpenSSL_1_1_1w/crypto/evp/encode.c
   Lisans: Apache-2.0.
   DEGISIKLIK: EVP_ENCODE_CTX/SRP-alfabesi secim mantigi (yalnizca SRP
   protokolu icin alternatif tabloya gecis) CIKARILDI - her zaman standart
   data_bin2ascii tablosu kullanilir. 3-baytlik gruplari 4 base64 karakterine
   ceviren ana dongu ve `=` doldurma mantigi DEGISTIRILMEDEN alindi.
   Girdi: bir satir metin (bayt dizisi olarak islenir).
   Cikti: base64 kodlanmis metin. */

static const unsigned char data_bin2ascii[65] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#define conv_bin2ascii(a, table) ((table)[(a)&0x3f])

static int evp_encodeblock_int(unsigned char *t, const unsigned char *f, int dlen) {
    int i, ret = 0;
    unsigned long l;
    const unsigned char *table = data_bin2ascii;

    for (i = dlen; i > 0; i -= 3) {
        if (i >= 3) {
            l = (((unsigned long)f[0]) << 16L) |
                (((unsigned long)f[1]) << 8L) | f[2];
            *(t++) = conv_bin2ascii(l >> 18L, table);
            *(t++) = conv_bin2ascii(l >> 12L, table);
            *(t++) = conv_bin2ascii(l >> 6L, table);
            *(t++) = conv_bin2ascii(l, table);
        } else {
            l = ((unsigned long)f[0]) << 16L;
            if (i == 2)
                l |= ((unsigned long)f[1] << 8L);

            *(t++) = conv_bin2ascii(l >> 18L, table);
            *(t++) = conv_bin2ascii(l >> 12L, table);
            *(t++) = (i == 1) ? '=' : conv_bin2ascii(l >> 6L, table);
            *(t++) = '=';
        }
        ret += 4;
        f += 3;
    }

    *t = '\0';
    return ret;
}

int main(void) {
    char line[1024];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    unsigned char out[2048];
    evp_encodeblock_int(out, (const unsigned char *)line, (int)strlen(line));
    printf("%s\n", out);
    return 0;
}

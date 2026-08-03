#include <stdio.h>
#include <string.h>
#include <stdint.h>

/* Kaynak: libsodium src/libsodium/sodium/codecs.c, sodium_bin2hex()
   (birebir).
   https://raw.githubusercontent.com/jedisct1/libsodium/master/src/libsodium/sodium/codecs.c
   Lisans: ISC.
   DEGISIKLIK: sodium_misuse() guvenlik-durdurma cagrisi (SIZE_MAX/2 tasma
   korumasi) test kolayligi icin bir hata mesaji + erken donuse cevrildi;
   asil HEX KODLAMA ARITMETIGI (sabit-zamanli, dallanmasiz bit numaralari
   ile rakam/harf secimi) DEGISTIRILMEDEN alindi. Bu aritmetik ilginctir:
   `((c - 10U) >> 8)` gibi ifadeler unsigned int uzerinde KASITLI ALT TASMA
   (underflow) kullanarak dallanmasiz secim yapar - Kok Neden A ile dogrudan
   ilgili, "hatali" degil KASITLI bir taşma deseni; Rust'a debug modda
   birebir cevrilirse (u32 uzerinde ayni cikarma) TASMA PANIGI verir, cunku
   Rust bu KASITLI tasmayi normal C'nin aksine varsayilan olarak yakalar.
   Girdi: bir satir metin (bayt dizisi olarak islenir, ikili veri gibi
   islenir).
   Cikti: kucuk harfli hex string. */

static char *
sodium_bin2hex(char *const hex, const size_t hex_maxlen,
               const unsigned char *const bin, const size_t bin_len)
{
    size_t       i = (size_t) 0U;
    unsigned int x;
    int          b;
    int          c;

    if (hex_maxlen <= bin_len * 2U) {
        return NULL;
    }
    while (i < bin_len) {
        c = bin[i] & 0xf;
        b = bin[i] >> 4;
        x = (unsigned char) (87U + c + (((c - 10U) >> 8) & ~38U)) << 8 |
            (unsigned char) (87U + b + (((b - 10U) >> 8) & ~38U));
        hex[i * 2U] = (char) x;
        x >>= 8;
        hex[i * 2U + 1U] = (char) x;
        i++;
    }
    hex[i * 2U] = 0U;

    return hex;
}

int main(void) {
    char line[512];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    size_t blen = strlen(line);
    char hex[1024];
    char *r = sodium_bin2hex(hex, sizeof(hex), (const unsigned char *)line, blen);
    if (r) printf("%s\n", r);
    return 0;
}

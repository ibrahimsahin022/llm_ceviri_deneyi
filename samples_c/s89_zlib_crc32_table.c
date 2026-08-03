#include <stdio.h>
#include <string.h>

/* Kaynak: zlib crc32.c - tablo uretim dongusu ve bayt-bazli guncelleme
   dongusu (birebir).
   https://raw.githubusercontent.com/madler/zlib/master/crc32.c
   Telif: Copyright (C) 1995-2006, 2010, 2011, 2012, 2016 Mark Adler.
   Lisans: zlib License.
   DEGISIKLIK: Gercek zlib dosyasindaki 8-bayt-birden (braid) hizlandirma
   ve derleme-zamani tablo secenekleri CIKARILDI; klasik POLY=0xedb88320
   tablo uretimi ve tek-bayt guncelleme cekirdegi BIREBIR korundu.
   Girdi: bir satir metin (bayt dizisi olarak islenir).
   Cikti: crc32 sonucu, ondalik (unsigned long). */

#define POLY 0xedb88320UL

static unsigned long crc_table[256];
static int table_ready = 0;

static void make_crc_table(void) {
    unsigned long c;
    int n, k;
    for (n = 0; n < 256; n++) {
        c = (unsigned long)n;
        for (k = 0; k < 8; k++) {
            c = c & 1 ? (POLY ^ (c >> 1)) : (c >> 1);
        }
        crc_table[n] = c;
    }
    table_ready = 1;
}

static unsigned long crc32_impl(unsigned long crc, const unsigned char *buf, size_t len) {
    if (!table_ready) make_crc_table();
    crc = crc ^ 0xffffffffUL;
    while (len >= 8) {
        len -= 8;
        crc = (crc >> 8) ^ crc_table[(crc ^ *buf++) & 0xff];
        crc = (crc >> 8) ^ crc_table[(crc ^ *buf++) & 0xff];
        crc = (crc >> 8) ^ crc_table[(crc ^ *buf++) & 0xff];
        crc = (crc >> 8) ^ crc_table[(crc ^ *buf++) & 0xff];
        crc = (crc >> 8) ^ crc_table[(crc ^ *buf++) & 0xff];
        crc = (crc >> 8) ^ crc_table[(crc ^ *buf++) & 0xff];
        crc = (crc >> 8) ^ crc_table[(crc ^ *buf++) & 0xff];
        crc = (crc >> 8) ^ crc_table[(crc ^ *buf++) & 0xff];
    }
    while (len) {
        len--;
        crc = (crc >> 8) ^ crc_table[(crc ^ *buf++) & 0xff];
    }
    return crc ^ 0xffffffffUL;
}

int main(void) {
    char line[4096];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    unsigned long result = crc32_impl(0L, (const unsigned char *)line, strlen(line));
    printf("%lu\n", result);
    return 0;
}

#include <inttypes.h>
#include <stdio.h>
#include <string.h>

/* KAYNAK: Rosetta Code, "CRC-32", C cozumu (zlib'siz, tablo tabanli varyant).
   https://rosettacode.org/wiki/CRC-32
   (GFDL 1.2 / CC-BY-SA lisansli, atifla yeniden kullanima acik).
   rc_crc32() fonksiyonu KAYNAKTAN DEGISTIRILMEDEN alinmistir; yalnizca
   main() bu deney ortaminin stdin/stdout sozlesmesine uyacak sekilde
   YENIDEN YAZILMISTIR (orijinalde sabit kodlanmis bir dize vardi). */

uint32_t
rc_crc32(uint32_t crc, const char *buf, size_t len)
{
	static uint32_t table[256];
	static int have_table = 0;
	uint32_t rem;
	uint8_t octet;
	int i, j;
	const char *p, *q;

	if (have_table == 0) {
		for (i = 0; i < 256; i++) {
			rem = i;
			for (j = 0; j < 8; j++) {
				if (rem & 1) {
					rem >>= 1;
					rem ^= 0xedb88320;
				} else
					rem >>= 1;
			}
			table[i] = rem;
		}
		have_table = 1;
	}

	crc = ~crc;
	q = buf + len;
	for (p = buf; p < q; p++) {
		octet = *p;
		crc = (crc >> 8) ^ table[(crc & 0xff) ^ octet];
	}
	return ~crc;
}

static void trim_newline(char *s) {
    size_t len = strlen(s);
    while (len > 0 && (s[len-1] == '\n' || s[len-1] == '\r')) {
        s[--len] = '\0';
    }
}

int
main(void)
{
	char buf[1024];
	if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
	trim_newline(buf);
	printf("%" PRIX32 "\n", rc_crc32(0, buf, strlen(buf)));
	return 0;
}

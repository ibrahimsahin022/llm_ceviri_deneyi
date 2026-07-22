#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

/* KAYNAK: Rosetta Code, "Soundex", C cozumu.
   https://rosettacode.org/wiki/Soundex
   (GFDL 1.2 / CC-BY-SA lisansli, atifla yeniden kullanima acik).
   add_code/init/soundex fonksiyonlari KAYNAKTAN DEGISTIRILMEDEN alinmistir;
   yalnizca main() bu deney ortaminin stdin/stdout sozlesmesine uyacak
   sekilde YENIDEN YAZILMISTIR (orijinalde sabit kodlanmis bir isim listesi
   ve karsilastirma vardi). */

/* for ASCII only */
static char code[128] = { 0 };
void add_code(const char *s, int c)
{
	while (*s) {
		code[(int)*s] = code[0x20 ^ (int)*s] = c;
		s++;
	}
}

void init(void)
{
	static const char *cls[] =
		{ "AEIOU", "", "BFPV", "CGJKQSXZ", "DT", "L", "MN", "R", 0};
	int i;
	for (i = 0; cls[i]; i++)
		add_code(cls[i], i - 1);
}

/* returns a static buffer; user must copy if want to save
   result across calls */
const char* soundex(const char *s)
{
	static char out[5];
	int c, prev, i;

	out[0] = out[4] = 0;
	if (!s || !*s) return out;

	out[0] = *s++;

	/* first letter, though not coded, can still affect next letter: Pfister */
	prev = code[(int)out[0]];
	for (i = 1; *s && i < 4; s++) {
		if ((c = code[(int)*s]) == prev) continue;

		if (c == -1) prev = 0;	/* vowel as separator */
		else if (c > 0) {
			out[i++] = c + '0';
			prev = c;
		}
	}
	while (i < 4) out[i++] = '0';
	return out;
}

int main(void)
{
	char buf[128];
	if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
	size_t len = strlen(buf);
	while (len > 0 && (buf[len-1] == '\n' || buf[len-1] == '\r')) {
		buf[--len] = '\0';
	}
	init();
	printf("%s\n", soundex(buf));
	return 0;
}

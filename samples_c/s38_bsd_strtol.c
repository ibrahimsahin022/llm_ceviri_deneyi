/*
 * Kaynak: OpenBSD libc strtol.c (BSD-3-Clause)
 * https://raw.githubusercontent.com/openbsd/src/master/lib/libc/stdlib/strtol.c
 * Copyright (c) 1990 The Regents of the University of California.
 * Orijinal, 1990'dan kalma gerçek işletim sistemi (libc) kodudur (~36 yıllık).
 *
 * DEĞİŞİKLİK: strtol() fonksiyonunun KENDİSİ kaynaktan HİÇBİR DEĞİŞİKLİK
 * YAPILMADAN alınmıştır (yalnızca OpenBSD'ye özgü son satır "DEF_STRONG(strtol);"
 * kaldırıldı, bu bir sembol-görünürlük makrosu olup mantığın parçası değildir).
 * main() bu deney ortamının stdin/stdout sözleşmesine uyacak şekilde tarafımızca
 * yazılmıştır (orijinal dosya bir kütüphane fonksiyonudur, main() içermez).
 *
 * ÖNEMLİ PLATFORM NOTU: Bu derleyicide (MSYS2/UCRT64 gcc, Windows LLP64 veri
 * modeli) `long` 32 bittir (sizeof(long)==4), Linux/LP64'teki 64 bitlik
 * `long`dan FARKLIDIR. Taşma eşiği testleri (aşağıda) bu platforma göre
 * seçilmiştir; başka bir platformda farklı eşik gerekebilir (bkz. makale §3.7).
 * Bu, aynı zamanda C↔Rust arasında bir ek semantik boşluğu (tamsayı GENİŞLİĞİ
 * platforma bağlıdır, Rust'ta ise i32/i64 açıkça seçilir) gözlemlemek için
 * bilinçli olarak korunmuştur.
 */
#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

long
strtol(const char *nptr, char **endptr, int base)
{
	const char *s;
	long acc, cutoff;
	int c;
	int neg, any, cutlim;

	/*
	 * Ensure that base is between 2 and 36 inclusive, or the special
	 * value of 0.
	 */
	if (base < 0 || base == 1 || base > 36) {
		if (endptr != 0)
			*endptr = (char *)nptr;
		errno = EINVAL;
		return 0;
	}

	/*
	 * Skip white space and pick up leading +/- sign if any.
	 * If base is 0, allow 0x for hex and 0 for octal, else
	 * assume decimal; if base is already 16, allow 0x.
	 */
	s = nptr;
	do {
		c = (unsigned char) *s++;
	} while (isspace(c));
	if (c == '-') {
		neg = 1;
		c = *s++;
	} else {
		neg = 0;
		if (c == '+')
			c = *s++;
	}
	if ((base == 0 || base == 16) && c == '0' &&
	    (*s == 'x' || *s == 'X') && isxdigit((unsigned char)s[1])) {
		c = s[1];
		s += 2;
		base = 16;
	}
	if (base == 0)
		base = c == '0' ? 8 : 10;

	/*
	 * Compute the cutoff value between legal numbers and illegal
	 * numbers.  That is the largest legal value, divided by the
	 * base.  An input number that is greater than this value, if
	 * followed by a legal input character, is too big.  One that
	 * is equal to this value may be valid or not; the limit
	 * between valid and invalid numbers is then based on the last
	 * digit.  For instance, if the range for longs is
	 * [-2147483648..2147483647] and the input base is 10,
	 * cutoff will be set to 214748364 and cutlim to either
	 * 7 (neg==0) or 8 (neg==1), meaning that if we have accumulated
	 * a value > 214748364, or equal but the next digit is > 7 (or 8),
	 * the number is too big, and we will return a range error.
	 *
	 * Set any if any `digits' consumed; make it negative to indicate
	 * overflow.
	 */
	cutoff = neg ? LONG_MIN : LONG_MAX;
	cutlim = cutoff % base;
	cutoff /= base;
	if (neg) {
		if (cutlim > 0) {
			cutlim -= base;
			cutoff += 1;
		}
		cutlim = -cutlim;
	}
	for (acc = 0, any = 0;; c = (unsigned char) *s++) {
		if (isdigit(c))
			c -= '0';
		else if (isalpha(c))
			c -= isupper(c) ? 'A' - 10 : 'a' - 10;
		else
			break;
		if (c >= base)
			break;
		if (any < 0)
			continue;
		if (neg) {
			if (acc < cutoff || (acc == cutoff && c > cutlim)) {
				any = -1;
				acc = LONG_MIN;
				errno = ERANGE;
			} else {
				any = 1;
				acc *= base;
				acc -= c;
			}
		} else {
			if (acc > cutoff || (acc == cutoff && c > cutlim)) {
				any = -1;
				acc = LONG_MAX;
				errno = ERANGE;
			} else {
				any = 1;
				acc *= base;
				acc += c;
			}
		}
	}
	if (endptr != 0)
		*endptr = (char *) (any ? s - 1 : nptr);
	return (acc);
}

/* --- main(): deney ortami stdin/stdout sozlesmesi (tarafimizca yazildi) --- */
int main(void) {
	char line[512];
	int base;
	while (fgets(line, sizeof(line), stdin)) {
		char numstr[400];
		if (sscanf(line, "%399s %d", numstr, &base) != 2) continue;
		errno = 0;
		char *endptr;
		long result = strtol(numstr, &endptr, base);
		int consumed = (int)(endptr - numstr);
		const char *errname = (errno == ERANGE) ? "ERANGE" :
		                       (errno == EINVAL) ? "EINVAL" : "OK";
		printf("result=%ld errno=%s consumed=%d\n", result, errname, consumed);
	}
	return 0;
}

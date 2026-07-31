/*
 * Kaynak: OpenBSD libc strtoul.c (BSD-3-Clause) -- strtol'un (s38) esi.
 * https://raw.githubusercontent.com/openbsd/src/master/lib/libc/stdlib/strtoul.c
 * Copyright (c) 1990 The Regents of the University of California.
 * Orijinal, 1990'dan kalma gercek isletim sistemi (libc) kodudur.
 *
 * DEGISIKLIK: strtoul() fonksiyonunun KENDISI kaynaktan HICBIR DEGISIKLIK
 * YAPILMADAN alinmistir (yalnizca OpenBSD'ye ozgu son satir
 * "DEF_STRONG(strtoul);" kaldirildi, bu bir sembol-gorunurluk makrosu olup
 * mantigin parcasi degildir). main() bu deney ortaminin stdin/stdout
 * sozlesmesine uyacak sekilde tarafimizca yazilmistir.
 *
 * ONEMLI PLATFORM NOTU (Kok Neden F, 6. bagimsiz ornek): Bu derleyicide
 * (MSYS2/UCRT64 gcc, Windows LLP64 veri modeli) `unsigned long` 32 bittir
 * (ULONG_MAX=4294967295), Linux/LP64'teki 64 bitlik `unsigned long`dan
 * FARKLIDIR. Dogal bir Rust cevirisi `unsigned long`u u64 secerse, bu
 * platformda gerceklesmesi gereken ERANGE/kirpma davranisini hic tetiklemez.
 */
#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

unsigned long
strtoul(const char *nptr, char **endptr, int base)
{
	const char *s;
	unsigned long acc, cutoff;
	int c;
	int neg, any, cutlim;

	/*
	 * See strtol for comments as to the logic used.
	 */
	if (base < 0 || base == 1 || base > 36) {
		if (endptr != 0)
			*endptr = (char *)nptr;
		errno = EINVAL;
		return 0;
	}

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

	cutoff = ULONG_MAX / (unsigned long)base;
	cutlim = ULONG_MAX % (unsigned long)base;
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
		if (acc > cutoff || (acc == cutoff && c > cutlim)) {
			any = -1;
			acc = ULONG_MAX;
			errno = ERANGE;
		} else {
			any = 1;
			acc *= (unsigned long)base;
			acc += c;
		}
	}
	if (neg && any > 0)
		acc = -acc;
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
		unsigned long result = strtoul(numstr, &endptr, base);
		int consumed = (int)(endptr - numstr);
		const char *errname = (errno == ERANGE) ? "ERANGE" :
		                       (errno == EINVAL) ? "EINVAL" : "OK";
		printf("result=%lu errno=%s consumed=%d\n", result, errname, consumed);
	}
	return 0;
}

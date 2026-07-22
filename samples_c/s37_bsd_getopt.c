/*
 * Kaynak: OpenBSD/FreeBSD libc getopt.c (BSD-3-Clause)
 * https://raw.githubusercontent.com/freebsd/freebsd-src/main/lib/libc/stdlib/getopt.c
 * Copyright (c) 1987, 1993, 1994 The Regents of the University of California.
 * Orijinal, 1987'den kalma, gerçek işletim sistemi (libc) kodudur (~39 yıllık).
 *
 * DEĞİŞİKLİK: Yalnızca aşağıdakiler değiştirildi/eklendi; getopt() fonksiyonunun
 * KENDİSİ kaynaktan HİÇBİR DEĞİŞİKLİK YAPILMADAN alınmıştır:
 *   - _getprogname() BSD'ye özgü olduğundan basit bir sabit döndürecek şekilde
 *     yeniden tanımlandı (uyarı mesajı biçimini etkiler, çekirdek mantığı değil).
 *   - main() bu deney ortamının stdin/stdout sözleşmesine uyacak şekilde
 *     tarafımızca yazıldı (orijinal dosyada main() yoktur, bu bir kütüphane
 *     fonksiyonudur).
 * Bu program, bilinçli olarak "global mutable state" (optarg/optind/optopt/
 * opterr) temasını gerçek, üretim (production) legacy kodunda test eder — bu,
 * veri setindeki s19_global_counter ile aynı semantik boşluğun sentetik değil
 * gerçek bir örneğidir.
 */
#include <stdio.h>
#include <string.h>

static const char *_getprogname(void) { return "prog"; }

int	opterr = 1,		/* if error message should be printed */
	optind = 1,		/* index into parent argv vector */
	optopt,			/* character checked for validity */
	optreset;		/* reset getopt */
char	*optarg;		/* argument associated with option */

#define	BADCH	(int)'?'
#define	BADARG	(int)':'
static char EMSG[] = "";
/*
 * getopt --
 *	Parse argc/argv argument vector.
 */
int
getopt(int nargc, char * const nargv[], const char *ostr)
{
	static char *place = EMSG;		/* option letter processing */
	char *oli;				/* option letter list index */

	if (optreset || *place == 0) {		/* update scanning pointer */
		optreset = 0;
		place = nargv[optind];
		if (optind >= nargc || *place++ != '-') {
			/* Argument is absent or is not an option */
			place = EMSG;
			return (-1);
		}
		optopt = *place++;
		if (optopt == '-' && *place == 0) {
			/* "--" => end of options */
			++optind;
			place = EMSG;
			return (-1);
		}
		if (optopt == 0) {
			/* Solitary '-', treat as a '-' option
			   if the program (eg su) is looking for it. */
			place = EMSG;
			if (strchr(ostr, '-') == NULL)
				return (-1);
			optopt = '-';
		}
	} else
		optopt = *place++;

	/* See if option letter is one the caller wanted... */
	if (optopt == ':' || (oli = strchr(ostr, optopt)) == NULL) {
		if (*place == 0)
			++optind;
		if (opterr && *ostr != ':')
			(void)fprintf(stderr,
			    "%s: illegal option -- %c\n", _getprogname(),
			    optopt);
		return (BADCH);
	}

	/* Does this option need an argument? */
	if (oli[1] != ':') {
		/* don't need argument */
		optarg = NULL;
		if (*place == 0)
			++optind;
	} else {
		/* Option-argument is either the rest of this argument or the
		   entire next argument. */
		if (*place)
			optarg = place;
		else if (oli[2] == ':')
			/*
			 * GNU Extension, for optional arguments if the rest of
			 * the argument is empty, we return NULL
			 */
			optarg = NULL;
		else if (nargc > ++optind)
			optarg = nargv[optind];
		else {
			/* option-argument absent */
			place = EMSG;
			if (*ostr == ':')
				return (BADARG);
			if (opterr)
				(void)fprintf(stderr,
				    "%s: option requires an argument -- %c\n",
				    _getprogname(), optopt);
			return (BADCH);
		}
		place = EMSG;
		++optind;
	}
	return (optopt);			/* return option letter */
}

/* --- main(): deney ortami stdin/stdout sozlesmesi (tarafimizca yazildi) --- */
int main(void) {
	char ostr[256];
	int n;
	if (!fgets(ostr, sizeof(ostr), stdin)) return 0;
	ostr[strcspn(ostr, "\r\n")] = 0;
	if (scanf("%d", &n) != 1) return 0;
	getchar(); /* newline'i tuket */

	char buf[64][256];
	char *argv_[65];
	argv_[0] = "prog";
	for (int i = 0; i < n; i++) {
		if (!fgets(buf[i], sizeof(buf[i]), stdin)) { buf[i][0] = 0; }
		buf[i][strcspn(buf[i], "\r\n")] = 0;
		argv_[i + 1] = buf[i];
	}
	int argc_ = n + 1;

	int c;
	while ((c = getopt(argc_, argv_, ostr)) != -1) {
		if (c == '?')
			printf("opt=? arg=(null)\n");
		else if (c == ':')
			printf("opt=: arg=(null)\n");
		else
			printf("opt=%c arg=%s\n", c, optarg ? optarg : "(null)");
	}
	for (int i = optind; i < argc_; i++) {
		printf("REST:%s\n", argv_[i]);
	}
	return 0;
}

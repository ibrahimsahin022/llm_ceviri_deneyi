/*
 * Kaynak: musl libc, src/stdlib/qsort.c (smoothsort / __qsort_r)
 * URL:    https://raw.githubusercontent.com/kraj/musl/master/src/stdlib/qsort.c
 * Lisans: MIT (Copyright (C) 2011 by Lynn Ochs; musl butunu MIT, bkz.
 *         https://git.musl-libc.org/cgit/musl/tree/COPYRIGHT)
 *
 * musl, Alpine Linux basta olmak uzere pek cok gomulu/konteyner Linux
 * dagitiminda GLIBC yerine kullanilan, gercek uretim (production) bir libc
 * kutuphanesidir. Bu dosyadaki algoritma (smoothsort - Dijkstra'nin adaptif
 * heapsort varyanti, Leonardo sayilariyla calisan bit-duzeyinde bir yigin
 * yapisi kullanir) musl'un qsort() uygulamasinin cekirdegidir ve onceki veri
 * setimizdeki heapsort (s39) orneginden cok daha karmasik/uzun bir gercek
 * dunya algoritmasidir.
 *
 * DEGISTIRILEN: (1) musl-ozel "atomic.h" / a_ctz_l() bagimliligi, platform-
 * bagimsiz bir "ntz_portable()" (count-trailing-zeros) ile degistirildi -
 * algoritmanin mantigi degismedi, sadece bir CPU-atomic makro cagrisi yerine
 * tasinabilir bir esdegeri kullanildi. (2) musl-ozel "weak_alias" makrosu
 * kaldirildi (sembol takma-adi, tek-dosya derlemede anlamsiz). (3) Bu main()
 * ve stdin/stdout suruculeri EKLENDI: n tamsayi okur, __qsort_r ile artan
 * sirada siralar, sonucu yazdirir. DEGISTIRILMEYEN: cycle(), shl(), shr(),
 * pntz(), sift(), trinkle(), __qsort_r() fonksiyonlarinin tum govdeleri
 * satir satir orijinal musl kaynagiyla aynidir.
 */
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

typedef int (*cmpfun)(const void *, const void *, void *);

/* Tasinabilir count-trailing-zeros (orijinal musl'da a_ctz_l / atomic.h) */
static inline int ntz_portable(unsigned long x)
{
	int n = 0;
	if (x == 0) return 8 * (int)sizeof(unsigned long);
	while (!(x & 1)) { x >>= 1; n++; }
	return n;
}
#define ntz(x) ntz_portable((x))

/* returns index of first bit set, excluding the low bit assumed to always
 * be set, starting from low bit of p[0] up through high bit of p[1] */
static inline int pntz(size_t p[2]) {
	if (p[0] != 1) return ntz(p[0] - 1);
	if (p[1]) return 8*sizeof(size_t) + ntz(p[1]);
	return 0;
}

static void cycle(size_t width, unsigned char* ar[], int n)
{
	unsigned char tmp[256];
	size_t l;
	int i;

	if(n < 2) {
		return;
	}

	ar[n] = tmp;
	while(width) {
		l = sizeof(tmp) < width ? sizeof(tmp) : width;
		memcpy(ar[n], ar[0], l);
		for(i = 0; i < n; i++) {
			memcpy(ar[i], ar[i + 1], l);
			ar[i] += l;
		}
		width -= l;
	}
}

/* shl() and shr() need n > 0 */
static inline void shl(size_t p[2], int n)
{
	if(n >= 8 * (int)sizeof(size_t)) {
		n -= 8 * sizeof(size_t);
		p[1] = p[0];
		p[0] = 0;
		if (!n) return;
	}
	p[1] <<= n;
	p[1] |= p[0] >> (sizeof(size_t) * 8 - n);
	p[0] <<= n;
}

static inline void shr(size_t p[2], int n)
{
	if(n >= 8 * (int)sizeof(size_t)) {
		n -= 8 * sizeof(size_t);
		p[0] = p[1];
		p[1] = 0;
		if (!n) return;
	}
	p[0] >>= n;
	p[0] |= p[1] << (sizeof(size_t) * 8 - n);
	p[1] >>= n;
}

/* power-of-two length for working array so that we can mask indices and
 * not depend on any invariant of the algorithm for spatial memory safety.
 * the original size was just 14*sizeof(size_t)+1 */
#define AR_LEN  (16 * sizeof(size_t))
#define AR_MASK (AR_LEN - 1)

static void sift(unsigned char *head, size_t width, cmpfun cmp, void *arg, int pshift, size_t lp[])
{
	unsigned char *rt, *lf;
	unsigned char *ar[AR_LEN];
	int i = 1;

	ar[0] = head;
	while(pshift > 1) {
		rt = head - width;
		lf = head - width - lp[pshift - 2];

		if(cmp(ar[0], lf, arg) >= 0 && cmp(ar[0], rt, arg) >= 0) {
			break;
		}
		if(cmp(lf, rt, arg) >= 0) {
			ar[i++ & AR_MASK] = lf;
			head = lf;
			pshift -= 1;
		} else {
			ar[i++ & AR_MASK] = rt;
			head = rt;
			pshift -= 2;
		}
	}
	cycle(width, ar, i & AR_MASK);
}

static void trinkle(unsigned char *head, size_t width, cmpfun cmp, void *arg, size_t pp[2], int pshift, int trusty, size_t lp[])
{
	unsigned char *stepson,
	              *rt, *lf;
	size_t p[2];
	unsigned char *ar[AR_LEN];
	int i = 1;
	int trail;

	p[0] = pp[0];
	p[1] = pp[1];

	ar[0] = head;
	while(p[0] != 1 || p[1] != 0) {
		stepson = head - lp[pshift];
		if(cmp(stepson, ar[0], arg) <= 0) {
			break;
		}
		if(!trusty && pshift > 1) {
			rt = head - width;
			lf = head - width - lp[pshift - 2];
			if(cmp(rt, stepson, arg) >= 0 || cmp(lf, stepson, arg) >= 0) {
				break;
			}
		}

		ar[i++ & AR_MASK] = stepson;
		head = stepson;
		trail = pntz(p);
		shr(p, trail);
		pshift += trail;
		trusty = 0;
	}
	if(!trusty) {
		cycle(width, ar, i & AR_MASK);
		sift(head, width, cmp, arg, pshift, lp);
	}
}

void __qsort_r(void *base, size_t nel, size_t width, cmpfun cmp, void *arg)
{
	size_t lp[12*sizeof(size_t)];
	size_t i, size = width * nel;
	unsigned char *head, *high;
	size_t p[2] = {1, 0};
	int pshift = 1;
	int trail;

	if (!size) return;

	head = base;
	high = head + size - width;

	/* Precompute Leonardo numbers, scaled by element width */
	for(lp[0]=lp[1]=width, i=2; (lp[i]=lp[i-2]+lp[i-1]+width) < size; i++);

	while(head < high) {
		if((p[0] & 3) == 3) {
			sift(head, width, cmp, arg, pshift, lp);
			shr(p, 2);
			pshift += 2;
		} else {
			if(lp[pshift - 1] >= (size_t)(high - head)) {
				trinkle(head, width, cmp, arg, p, pshift, 0, lp);
			} else {
				sift(head, width, cmp, arg, pshift, lp);
			}

			if(pshift == 1) {
				shl(p, 1);
				pshift = 0;
			} else {
				shl(p, pshift - 1);
				pshift = 1;
			}
		}

		p[0] |= 1;
		head += width;
	}

	trinkle(head, width, cmp, arg, p, pshift, 0, lp);

	while(pshift != 1 || p[0] != 1 || p[1] != 0) {
		if(pshift <= 1) {
			trail = pntz(p);
			shr(p, trail);
			pshift += trail;
		} else {
			shl(p, 2);
			pshift -= 2;
			p[0] ^= 7;
			shr(p, 1);
			trinkle(head - lp[pshift] - width, width, cmp, arg, p, pshift + 1, 1, lp);
			shl(p, 1);
			p[0] |= 1;
			trinkle(head - width, width, cmp, arg, p, pshift, 1, lp);
		}
		head -= width;
	}
}

/* ---- Suruc (bu calisma icin eklendi): stdin'den n ve n tamsayi okur,
 * __qsort_r ile artan sirada siralar, sonucu tek satirda bosluk-ayrikli
 * yazdirir. ---- */
static int cmp_int(const void *a, const void *b, void *arg)
{
	(void)arg;
	int x = *(const int *)a;
	int y = *(const int *)b;
	return (x > y) - (x < y);
}

int main(void)
{
	int n;
	if (scanf("%d", &n) != 1 || n < 0) return 1;
	int *arr = malloc(sizeof(int) * (n > 0 ? n : 1));
	if (!arr) return 1;
	for (int i = 0; i < n; i++) {
		if (scanf("%d", &arr[i]) != 1) { free(arr); return 1; }
	}
	__qsort_r(arr, (size_t)n, sizeof(int), cmp_int, NULL);
	for (int i = 0; i < n; i++) {
		printf("%d", arr[i]);
		if (i + 1 < n) printf(" ");
	}
	printf("\n");
	free(arr);
	return 0;
}

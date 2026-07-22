/*
 * Kaynak: OpenBSD libc heapsort.c (BSD-3-Clause)
 * https://raw.githubusercontent.com/openbsd/src/master/lib/libc/stdlib/heapsort.c
 * Copyright (c) 1991, 1993 The Regents of the University of California.
 * Orijinal, 1991'den kalma gerçek işletim sistemi (libc) kodudur (~35 yıllık).
 *
 * DEĞİŞİKLİK: heapsort() fonksiyonunun KENDİSİ (ve SWAP/COPY/CREATE/SELECT
 * makroları) kaynaktan HİÇBİR DEĞİŞİKLİK YAPILMADAN alınmıştır (yalnızca
 * OpenBSD'ye özgü son satır "DEF_WEAK(heapsort);" kaldırıldı). Bu fonksiyon,
 * qsort(3) ailesinin klasik generic (void*, fonksiyon-işaretçili karşılaştırıcı,
 * bayt-düzeyinde SWAP/COPY) tasarımını kullanır — veri setindeki diğer örneklerin
 * aksine, bu programın çekirdek algoritması BAYT DÜZEYİNDE HAM İŞARETÇİ
 * ARİTMETİĞİNE (void* + size_t offset) yapısal olarak bağımlıdır. int_cmp()
 * karşılaştırıcısı ve main(), Rosetta Code örneklerindeki gibi, bu deney
 * ortamının stdin/stdout sözleşmesine uyacak şekilde tarafımızca yazılmıştır
 * (orijinal dosya bir kütüphane fonksiyonudur, main() içermez; qsort(3) tarzı
 * bir çağıran her zaman kendi karşılaştırıcısını yazmak zorundadır).
 *
 * Bu örnek özellikle şu amaçla eklenmiştir: veri setindeki diğer tüm dinamik
 * veri yapıları (s25, s28, s29) Rust çevirilerinde `unsafe` gerektirmeden
 * (Option<Box<T>> ile) ifade edilebilmişti (§4.8); bu programın void*
 * genericliği ve bayt-düzeyinde SWAP/COPY makroları, LLM'in aynı "hiç unsafe
 * kullanmama" örüntüsünü GERÇEKTEN ham işaretçi aritmetiği gerektiren bir
 * fonksiyonla karşılaştığında da sürdürüp sürdürmediğini test eder.
 */
#include <sys/types.h>
#include <errno.h>
#include <stdlib.h>
#include <stdio.h>

#define	SWAP(a, b, count, size, tmp) { \
	count = size; \
	do { \
		tmp = *a; \
		*a++ = *b; \
		*b++ = tmp; \
	} while (--count); \
}

#define COPY(a, b, count, size, tmp1, tmp2) { \
	count = size; \
	tmp1 = a; \
	tmp2 = b; \
	do { \
		*tmp1++ = *tmp2++; \
	} while (--count); \
}

#define CREATE(initval, nmemb, par_i, child_i, par, child, size, count, tmp) { \
	for (par_i = initval; (child_i = par_i * 2) <= nmemb; \
	    par_i = child_i) { \
		child = base + child_i * size; \
		if (child_i < nmemb && compar(child, child + size) < 0) { \
			child += size; \
			++child_i; \
		} \
		par = base + par_i * size; \
		if (compar(child, par) <= 0) \
			break; \
		SWAP(par, child, count, size, tmp); \
	} \
}

#define SELECT(par_i, child_i, nmemb, par, child, size, k, count, tmp1, tmp2) { \
	for (par_i = 1; (child_i = par_i * 2) <= nmemb; par_i = child_i) { \
		child = base + child_i * size; \
		if (child_i < nmemb && compar(child, child + size) < 0) { \
			child += size; \
			++child_i; \
		} \
		par = base + par_i * size; \
		COPY(par, child, count, size, tmp1, tmp2); \
	} \
	for (;;) { \
		child_i = par_i; \
		par_i = child_i / 2; \
		child = base + child_i * size; \
		par = base + par_i * size; \
		if (child_i == 1 || compar(k, par) < 0) { \
			COPY(child, k, count, size, tmp1, tmp2); \
			break; \
		} \
		COPY(child, par, count, size, tmp1, tmp2); \
	} \
}

int
heapsort(void *vbase, size_t nmemb, size_t size,
    int (*compar)(const void *, const void *))
{
	size_t cnt, i, j, l;
	char tmp, *tmp1, *tmp2;
	char *base, *k, *p, *t;

	if (nmemb <= 1)
		return (0);

	if (!size) {
		errno = EINVAL;
		return (-1);
	}

	if ((k = malloc(size)) == NULL)
		return (-1);

	base = (char *)vbase - size;

	for (l = nmemb / 2 + 1; --l;)
		CREATE(l, nmemb, i, j, t, p, size, cnt, tmp);

	while (nmemb > 1) {
		COPY(k, base + nmemb * size, cnt, size, tmp1, tmp2);
		COPY(base + nmemb * size, base + size, cnt, size, tmp1, tmp2);
		--nmemb;
		SELECT(i, j, nmemb, t, p, size, k, cnt, tmp1, tmp2);
	}
	free(k);
	return (0);
}

/* --- int_cmp() ve main(): tarafimizca yazildi (qsort(3) tarzi caginin
 * gerektirdigi karsilastirici + deney ortami stdin/stdout sozlesmesi) --- */
static int int_cmp(const void *a, const void *b) {
	int ia = *(const int *)a;
	int ib = *(const int *)b;
	return (ia > ib) - (ia < ib);
}

int main(void) {
	int n;
	if (scanf("%d", &n) != 1) return 0;
	int *arr = malloc(sizeof(int) * (n > 0 ? n : 1));
	for (int i = 0; i < n; i++) {
		scanf("%d", &arr[i]);
	}
	heapsort(arr, (size_t)n, sizeof(int), int_cmp);
	for (int i = 0; i < n; i++) {
		printf("%d%s", arr[i], (i + 1 < n) ? " " : "\n");
	}
	if (n == 0) printf("\n");
	free(arr);
	return 0;
}

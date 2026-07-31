#include <stdio.h>
#include <limits.h>

/* Girdi: "<a> <b>" (iki `long` deger).
   Cikti: kirpilmis carpim (%ld).
   NOT: Kok Neden F (platforma bagli tamsayi genisligi) icin 5. bagimsiz
   ornek (s51_long_clamp'in TOPLAMA yerine CARPMA yapan esi). Bu ortamda
   (Windows/LLP64, MSYS2 gcc) `long` 32 bittir; Linux/LP64'te 64 bittir.
   Dogal bir Rust cevirisi `long`u yaygin varsayimla i64 secerse, bu
   programin 32-bit LONG_MAX/LONG_MIN sinirinda kirpmasi gereken
   degerlerde HICBIR kirpma yapmaz. */
static long safe_mul_clamped(long a, long b) {
    double product = (double)a * (double)b;
    if (product > (double)LONG_MAX) return LONG_MAX;
    if (product < (double)LONG_MIN) return LONG_MIN;
    return a * b;
}

int main(void) {
    long a, b;
    if (scanf("%ld %ld", &a, &b) != 2) return 1;
    printf("%ld\n", safe_mul_clamped(a, b));
    return 0;
}

#include <stdio.h>

/* Girdi: tek bir pozitif tam sayi n.
   Cikti: n sayisinin Collatz dizisiyle 1'e ulasmasi icin gereken adim sayisi.
   NOT: Ara degerler buyuyebilir, bu yuzden 64-bit isaretsiz kullanilir. */
int main(void) {
    unsigned long long n;
    if (scanf("%llu", &n) != 1) return 0;
    unsigned long long steps = 0;
    while (n != 1) {
        if (n % 2 == 0) n = n / 2;
        else n = 3 * n + 1;
        steps++;
    }
    printf("%llu\n", steps);
    return 0;
}

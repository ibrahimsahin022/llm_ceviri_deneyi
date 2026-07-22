#include <stdio.h>

/* Girdi: tek bir tam sayi n (0 <= n <= 90).
   Cikti: n. Fibonacci sayisi (unsigned 64-bit). */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    unsigned long long a = 0, b = 1;
    for (int i = 0; i < n; i++) {
        unsigned long long t = a + b;
        a = b;
        b = t;
    }
    printf("%llu\n", a);
    return 0;
}

#include <stdio.h>

/* Girdi: tek bir tam sayi n (0 <= n <= 20).
   Cikti: n! degeri (unsigned 64-bit). */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    unsigned long long f = 1;
    for (int i = 2; i <= n; i++) {
        f *= (unsigned long long)i;
    }
    printf("%llu\n", f);
    return 0;
}

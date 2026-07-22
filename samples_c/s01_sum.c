#include <stdio.h>

/* Girdi: ilk satirda n, ardindan n adet tam sayi.
   Cikti: sayilarin toplami. */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    long long sum = 0;
    for (int i = 0; i < n; i++) {
        long long x;
        if (scanf("%lld", &x) != 1) break;
        sum += x;
    }
    printf("%lld\n", sum);
    return 0;
}

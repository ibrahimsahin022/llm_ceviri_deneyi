#include <stdio.h>

/* Girdi: tek bir tam sayi n.
   Cikti: n'e kadar (n dahil) asal sayi adedi. */
int is_prime(int x) {
    if (x < 2) return 0;
    for (int d = 2; (long)d * d <= x; d++) {
        if (x % d == 0) return 0;
    }
    return 1;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    int count = 0;
    for (int i = 2; i <= n; i++) {
        if (is_prime(i)) count++;
    }
    printf("%d\n", count);
    return 0;
}

#include <stdio.h>

/* Girdi: tek bir isaretsiz tam sayi n.
   Cikti: n'in Gray kodu (n XOR (n>>1)). */
int main(void) {
    unsigned int n;
    if (scanf("%u", &n) != 1) return 0;
    printf("%u\n", n ^ (n >> 1));
    return 0;
}

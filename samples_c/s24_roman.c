#include <stdio.h>

/* Girdi: tek bir tam sayi n (1..3999).
   Cikti: n'in Roma rakamiyla yazimi. Ornek: 1994 -> MCMXCIV. */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    int vals[] = {1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1};
    const char *syms[] = {"M", "CM", "D", "CD", "C", "XC",
                          "L", "XL", "X", "IX", "V", "IV", "I"};
    for (int i = 0; i < 13; i++) {
        while (n >= vals[i]) {
            printf("%s", syms[i]);
            n -= vals[i];
        }
    }
    printf("\n");
    return 0;
}

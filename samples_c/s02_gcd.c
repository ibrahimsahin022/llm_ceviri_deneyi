#include <stdio.h>

/* Girdi: iki tam sayi a b.
   Cikti: en buyuk ortak bolen (Oklid algoritmasi). */
int gcd(int a, int b) {
    while (b != 0) {
        int t = b;
        b = a % b;
        a = t;
    }
    return a < 0 ? -a : a;
}

int main(void) {
    int a, b;
    if (scanf("%d %d", &a, &b) != 2) return 0;
    printf("%d\n", gcd(a, b));
    return 0;
}

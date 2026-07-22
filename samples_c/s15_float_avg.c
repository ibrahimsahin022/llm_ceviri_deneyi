#include <stdio.h>

/* Girdi: ilk satirda n, ardindan n adet ondalikli sayi.
   Cikti: ortalama, C'nin %g bicimiyle yazdirilir. */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    double sum = 0.0;
    for (int i = 0; i < n; i++) {
        double x;
        if (scanf("%lf", &x) != 1) break;
        sum += x;
    }
    double avg = (n > 0) ? sum / n : 0.0;
    printf("%g\n", avg);
    return 0;
}

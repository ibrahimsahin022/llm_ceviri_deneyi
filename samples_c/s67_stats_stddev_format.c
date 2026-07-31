#include <stdio.h>
#include <math.h>

/* Girdi: ilk satirda n, ardindan n adet ondalikli sayi.
   Cikti: "<ortalama> <standart_sapma>" (%g bicimiyle).
   NOT: Kok Neden D (%g bicimlendirme) icin 4. bagimsiz ornek (bkz. s15,
   s27, s48). %g, 6 anlamli basamaga kirpar ve gereksiz sondaki sifirlari
   atar; Rust'in varsayilan {} bicimi bunu yapmaz. */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1 || n <= 0 || n > 1000) return 0;
    double vals[1000];
    double sum = 0;
    for (int i = 0; i < n; i++) {
        if (scanf("%lf", &vals[i]) != 1) return 0;
        sum += vals[i];
    }
    double mean = sum / n;
    double sq = 0;
    for (int i = 0; i < n; i++) {
        double d = vals[i] - mean;
        sq += d * d;
    }
    double variance = sq / n;
    double stddev = sqrt(variance);
    printf("%g %g\n", mean, stddev);
    return 0;
}

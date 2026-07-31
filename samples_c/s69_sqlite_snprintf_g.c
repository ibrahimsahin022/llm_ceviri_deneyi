#include <stdio.h>

/* Girdi: "<pay> <payda>".
   Cikti: oranin %g bicimiyle yazdirilmasi (SQLite'in sqlite3_snprintf'i
   gibi uretim veritabani kodunda sikca kullanilan "orani %g ile raporla"
   deseninin sadelestirilmis hali - bkz. sqlite3.c icindeki printf.c
   mantigi, Public Domain).
   NOT: Kok Neden D (%g bicimlendirme) icin 6. bagimsiz ornek. Bolme
   sonuclari (orn. 22/7, 1/3) genellikle tekrar eden ondaliklar uretir;
   %g bunlari 6 anlamli basamaga yuvarlayip kirpar. */
int main(void) {
    double a, b;
    if (scanf("%lf %lf", &a, &b) != 2 || b == 0.0) return 0;
    double ratio = a / b;
    printf("%g\n", ratio);
    return 0;
}

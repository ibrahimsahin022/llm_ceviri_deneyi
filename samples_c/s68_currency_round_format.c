#include <stdio.h>

/* Girdi: "<birim_fiyat> <adet>".
   Cikti: toplam tutar (%g bicimiyle).
   NOT: Kok Neden D (%g bicimlendirme) icin 5. bagimsiz ornek. Kayan nokta
   carpiminin dogal kesinlik hatalari (ornegin 19.99*3) %g'nin 6 anlamli
   basamaga kirpmasiyla ortulur; Rust'in varsayilan {} bicimi ham ondalik
   hata basamaklarini oldugu gibi gosterir. */
int main(void) {
    double price;
    int qty;
    if (scanf("%lf %d", &price, &qty) != 2) return 0;
    double total = price * (double)qty;
    printf("%g\n", total);
    return 0;
}

/*
 * Kok Neden E (global degistirilebilir durum) icin 2. bagimsiz ornek
 * (s19_global_counter'dan farkli bir desen: sayac artirma degil, ardisik
 * ID atama servisi - cagrilar arasinda kalici durum tasiyan bir "sonraki
 * kimlik uretici" fonksiyonu, tipik bir eski-kod (legacy) deseni).
 *
 * next_id(), her cagrildiginda BASLANGIC_ID'den itibaren artan, benzersiz
 * bir kimlik dondurur; bu durum fonksiyonun disinda gorunmez ama cagrilar
 * arasinda kalicidir - C'de `static int` ile dogal, Rust'ta ise mutable
 * static bir degiskene erisim `unsafe` gerektirir.
 */
#include <stdio.h>

#define BASLANGIC_ID 1000

static int next_id(void)
{
    static int counter = BASLANGIC_ID;
    return counter++;
}

int main(void)
{
    int n;
    if (scanf("%d", &n) != 1) return 1;
    for (int i = 0; i < n; i++) {
        printf("%d\n", next_id());
    }
    return 0;
}

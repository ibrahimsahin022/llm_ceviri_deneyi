#include <stdio.h>

/* Girdi: "<tohum> <n>".
   Cikti: n adet sozde-rasgele sayi, her biri ayri satirda.
   NOT: Kok Neden E (guvensiz global durum) icin 3. bagimsiz ornek (bkz.
   s19, s50). Global degistirilebilir tohum (g_seed), her cagride
   fonksiyon disi durumu degistirir - Rust'ta bu dogal olarak `static mut`
   (unsafe) veya bir refactor gerektirir. */
static unsigned int g_seed = 1;

unsigned int next_rand(void) {
    g_seed = g_seed * 1103515245u + 12345u;
    return (g_seed >> 16) & 0x7FFFu;
}

int main(void) {
    unsigned int seed;
    int n;
    if (scanf("%u %d", &seed, &n) != 2) return 0;
    g_seed = seed;
    for (int i = 0; i < n; i++) {
        printf("%u\n", next_rand());
    }
    return 0;
}

/*
 * Ozgun program (bu calisma icin yazilmistir).
 * Hedef: Fonksiyon-lokal 'static' diziler (kapsamli, ama fonksiyon
 * cagrilari arasinda durumu koruyan bellek) ile ozyinelemeli memoizasyon.
 * C'de yaygin bir legacy deyimidir. Rust'ta boyle bir "fonksiyona ozel,
 * cagrilar arasi kalici durum" dogrudan karsilanmaz - LLM ya 'static mut'
 * (unsafe) kullanmali ya da durumu acikca parametre/global yapiya
 * tasiyarak yeniden yapilandirmalidir (bkz. s19/s37 ile ayni tema, ama
 * global degil fonksiyon-kapsamli static).
 */
#include <stdio.h>

long fib_cached(int n) {
    static long cache[50];
    static int have[50];
    if (n < 2) return n;
    if (have[n]) return cache[n];
    long r = fib_cached(n - 1) + fib_cached(n - 2);
    cache[n] = r;
    have[n] = 1;
    return r;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    printf("%ld\n", fib_cached(n));
    return 0;
}

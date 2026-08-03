#include <stdio.h>
#include <stdarg.h>

/* Girdi: ilk satirda n (arguman sayisi), ardindan n tam sayi.
   Cikti: n tam sayinin toplami, degisken-argumanli (va_list) bir
   fonksiyon uzerinden hesaplanir.
   NOT: C'nin va_list/va_arg mekanizmasinin Rust'ta DOGRUDAN karsiligi
   YOKTUR (Rust "variadic" fonksiyonlari yalniz extern "C" FFI icin
   destekler, kullanici kodu icin degil). Doğal ceviri, argumanlari bir
   dilime (slice) veya Vec'e toplayan sabit-imzali bir fonksiyona
   donusturmektir - bu, C<->Rust arasinda "dogrudan" cevrilemeyen bir dil
   ozelligi ornegidir. */

static long sum_variadic(int count, ...) {
    va_list args;
    va_start(args, count);
    long total = 0;
    for (int i = 0; i < count; i++) {
        total += va_arg(args, int);
    }
    va_end(args);
    return total;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1 || n < 0 || n > 20) return 0;

    int values[20];
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &values[i]) != 1) return 0;
    }

    long total;
    switch (n) {
        case 0: total = sum_variadic(0); break;
        case 1: total = sum_variadic(1, values[0]); break;
        case 2: total = sum_variadic(2, values[0], values[1]); break;
        case 3: total = sum_variadic(3, values[0], values[1], values[2]); break;
        default: {
            /* n>3 icin basit dongulu toplam (variadic cagrisi sabit
               olmali) - test girdileri asıl variadic yolunu (n<=3)
               kullanir. */
            total = 0;
            for (int i = 0; i < n; i++) total += values[i];
            break;
        }
    }

    printf("%ld\n", total);
    return 0;
}

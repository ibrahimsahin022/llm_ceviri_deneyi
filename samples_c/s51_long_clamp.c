/*
 * Kok Neden F (platforma bagli tamsayi genisligi) icin 2. bagimsiz ornek
 * (s38_bsd_strtol'den farkli bir desen: dize ayristirma degil, savunmaci
 * tasma-kirpme (clamping) aritmetigi - eski C kod tabanlarinda cok yaygin
 * bir "long ile guvenli toplama" deseni).
 *
 * C'nin `long` tipinin genisligi platforma baglidir: Linux/LP64'te 64 bit,
 * ancak bu derleme ortaminda (Windows/LLP64, MSYS2 gcc) 32 bittir - tipki
 * s38'deki gibi (bkz. makale SS3.7). safe_add_clamped, iki `long` degeri
 * toplar ve LONG_MIN/LONG_MAX sinirlarina kirpar. Dogal bir Rust cevirisi
 * `long`u yaygin bir varsayimla 64-bit `i64` olarak secerse, bu programin
 * 32-bit `long` sinirinda kirpmasi gereken degerlerde HICBIR kirpma
 * yapmaz - cunku i64'te henuz tasma esigine ulasilmamistir.
 */
#include <stdio.h>
#include <limits.h>

static long safe_add_clamped(long a, long b)
{
    /* toplamayi buyuk bir tipte yapip sinirlara kirp (klasik desen) */
    double sum = (double)a + (double)b;
    if (sum > (double)LONG_MAX) return LONG_MAX;
    if (sum < (double)LONG_MIN) return LONG_MIN;
    return a + b;
}

int main(void)
{
    long a, b;
    if (scanf("%ld %ld", &a, &b) != 2) return 1;
    printf("%ld\n", safe_add_clamped(a, b));
    return 0;
}

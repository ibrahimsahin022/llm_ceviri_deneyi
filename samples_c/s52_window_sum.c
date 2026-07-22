/*
 * Kok Neden G (isaretsiz/usize tip seciminin yarattigi yeni tasma) icin
 * 2. bagimsiz ornek (s40_diff_sum'dan farkli bir desen: ardisik fark
 * degil, "son k eleman"in toplami - kayan pencere (sliding window) ozeti,
 * yaygin bir istatistik/log-isleme deseni).
 *
 * C referansi dizi boyutunu ve pencere genisligini isaretli `int` tutar;
 * n < k oldugunda `start = n - k` NEGATIF olur ve dongu (start<0 oldugundan
 * 0'dan baslatilarak) guvenle tum diziyi toplar. LLM, "dizi indeksi" icin
 * idiyomatik Rust tercihi olan `usize` secerse, n < k durumunda `n - k`
 * usize altinda TASAR ve debug modda panik verir.
 */
#include <stdio.h>

int main(void)
{
    int n, k;
    if (scanf("%d %d", &n, &k) != 2 || n < 0 || k < 0) return 1;
    int arr[1000];
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &arr[i]) != 1) return 1;
    }

    int start = n - k;
    if (start < 0) start = 0;

    long sum = 0;
    for (int i = start; i < n; i++) {
        sum += arr[i];
    }
    printf("%ld\n", sum);
    return 0;
}

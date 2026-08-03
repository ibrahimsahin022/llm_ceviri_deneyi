#include <stdio.h>

/* Girdi: ilk satirda n, ardindan n tam sayi, sonra k.
   Cikti: dizinin SON k elemani (soldan n-k index'inden itibaren).
   NOT: Kok Neden G (usize tasmasi) icin 3. bagimsiz ornek (bkz. s40 'n-1',
   s52 'i-1'). Buradaki desen n-k'dir (k degiskendir, sabit 1 degil). C'de
   int ile n<k oldugunda "n-k" negatif olur ve baslangic indeksi kirpilarak
   0'a cekilir - guvenli. Rust'ta n,k usize'a cevrilirse n-k, n<k iken
   TASMA PANIGI verir (guvenli if kontrolu es gecilirse). */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1 || n < 0 || n > 1000) return 0;
    int arr[1000];
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &arr[i]) != 1) return 0;
    }
    int k;
    if (scanf("%d", &k) != 1) return 0;
    int start = n - k;
    if (start < 0) start = 0;
    for (int i = start; i < n; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
    return 0;
}

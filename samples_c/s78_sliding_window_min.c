#include <stdio.h>

/* Girdi: ilk satirda n, ardindan n tam sayi.
   Cikti: "yerel minimum" olan indekslerin sayisi (arr[i] < arr[i-1] VE
   arr[i] < arr[i+1], sinir elemanlari haric).
   NOT: Kok Neden G (usize tasmasi) icin 5. bagimsiz ornek (s52'nin farkli
   bir "onceki eleman" deseni: pencere toplami yerine yerel minimum
   sayimi). C'de int i ile i-1, kontrolden once hesaplansa bile negatif
   olabilir ve sorun olmaz. Rust'a usize ile cevrilirse i=0 iken i-1
   hesaplanmasi (kontrolden ONCE) TASMA PANIGI verir. */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1 || n < 2 || n > 1000) return 0;
    int arr[1000];
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &arr[i]) != 1) return 0;
    }
    int count = 0;
    for (int i = 0; i < n; i++) {
        int prev_idx = i - 1;
        int next_idx = i + 1;
        if (prev_idx >= 0 && next_idx < n) {
            if (arr[i] < arr[prev_idx] && arr[i] < arr[next_idx]) {
                count++;
            }
        }
    }
    printf("%d\n", count);
    return 0;
}

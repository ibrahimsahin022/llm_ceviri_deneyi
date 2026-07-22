/*
 * Ozgun program (bu calisma icin yazilmistir).
 * Hedef: C'de dizi boyutu 'int' (isaretli) tutuldugunda n==0 durumu
 * "for (i=0; i<n-1; i++)" dongusunde GUVENLE (0 < -1 yanlis, dongu hic
 * calismaz) atlatilir. Ancak bir LLM, "dizi boyutu" kavramini Rust'a
 * cevirirken dogal olarak 'usize' secebilir (idiyomatik Rust'ta boyutlar
 * usize'dir); bu durumda n==0 iken 'n - 1' usize altinda TASAR (underflow)
 * ve debug modda panik verir - kaynak C kodunda hicbir zaman var olmayan,
 * yalnizca Rust'a cevirirken secilen tip nedeniyle ortaya cikan YENI bir
 * hata sinifidir.
 */
#include <stdio.h>

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    int arr[1000];
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &arr[i]) != 1) return 0;
    }
    int diffsum = 0;
    for (int i = 0; i < n - 1; i++) {
        diffsum += arr[i + 1] - arr[i];
    }
    printf("%d\n", diffsum);
    return 0;
}

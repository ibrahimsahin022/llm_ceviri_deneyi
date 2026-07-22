/*
 * Ozgun program (bu calisma icin yazilmistir).
 * Hedef: 'goto' ile kaynak temizleme (cleanup) deyimi - C'de birden fazla
 * malloc sonrasi hata durumunda tek bir temizlik noktasina atlamak icin
 * yaygin, klasik bir legacy desenidir. Rust'ta 'goto' yoktur; LLM bunu
 * erken don (early return) + otomatik Drop (RAII) ile yeniden
 * yapilandirmak zorundadir.
 */
#include <stdio.h>
#include <stdlib.h>

int process(int n, int *out) {
    int *buf1 = NULL, *buf2 = NULL;
    int rc = -1;
    if (n < 0) goto cleanup;
    buf1 = malloc(sizeof(int) * (size_t)(n > 0 ? n : 1));
    if (!buf1) goto cleanup;
    buf2 = malloc(sizeof(int) * (size_t)(n > 0 ? n : 1));
    if (!buf2) goto cleanup;
    {
        int sum = 0;
        for (int i = 0; i < n; i++) {
            buf1[i] = i;
            buf2[i] = i * 2;
            sum += buf1[i] + buf2[i];
        }
        *out = sum;
    }
    rc = 0;
cleanup:
    free(buf1);
    free(buf2);
    return rc;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    int out = -999;
    int rc = process(n, &out);
    printf("%d %d\n", rc, out);
    return 0;
}

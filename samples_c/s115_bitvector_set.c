#include <stdio.h>

/* Girdi: ilk satirda islem sayisi n, ardindan n satir - "S <bit>" (set),
   "C <bit>" (clear), "T <bit>" (test).
   Cikti: T icin "1"/"0". Sonda kume icindeki toplam 1-bit sayisi (popcount).
   NOT: bit indeksi/kelime indeksi aritmetigi (`bit / 64`, `bit % 64`)
   Kok Neden G ile dolayli olarak ilgilidir - `1u << (bit % 64)` deseninde
   kaydirma miktari dogru turde (unsigned) tutulmazsa davranis degisebilir. */

#define NUM_WORDS 4
#define BITS_PER_WORD 64

int main(void) {
    unsigned long long words[NUM_WORDS] = {0, 0, 0, 0};

    int n;
    if (scanf("%d", &n) != 1) return 0;

    for (int i = 0; i < n; i++) {
        char op[4];
        int bit;
        if (scanf("%3s %d", op, &bit) != 2) return 0;
        if (bit < 0 || bit >= NUM_WORDS * BITS_PER_WORD) continue;
        int w = bit / BITS_PER_WORD;
        int b = bit % BITS_PER_WORD;
        if (op[0] == 'S') {
            words[w] |= (1ULL << b);
        } else if (op[0] == 'C') {
            words[w] &= ~(1ULL << b);
        } else {
            printf("%d\n", (int)((words[w] >> b) & 1ULL));
        }
    }

    int count = 0;
    for (int w = 0; w < NUM_WORDS; w++) {
        unsigned long long v = words[w];
        while (v) {
            count += (int)(v & 1ULL);
            v >>= 1;
        }
    }
    printf("count=%d\n", count);
    return 0;
}

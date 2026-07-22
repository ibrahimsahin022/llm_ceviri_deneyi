/*
 * Ozgun program (bu calisma icin yazilmistir).
 * Hedef: C'nin switch/case fallthrough (dusme) davranisi - bilincli olarak
 * 'break' konulmadan bir sonraki case'e devam etmek, kumulatif deger
 * hesaplamak icin klasik bir legacy deyimidir. Rust'in match'i VARSAYILAN
 * OLARAK dusmez; LLM bu davranisi acikca yeniden uretmezse (ornegin sadece
 * eslesen kolu calistirirsa) sonuc yanlis olur.
 */
#include <stdio.h>

int main(void) {
    int level;
    if (scanf("%d", &level) != 1) return 0;
    int bonus = 0;
    switch (level) {
        case 4: bonus += 8; /* fallthrough */
        case 3: bonus += 4; /* fallthrough */
        case 2: bonus += 2; /* fallthrough */
        case 1: bonus += 1;
                break;
        default: bonus = 0;
    }
    printf("%d\n", bonus);
    return 0;
}

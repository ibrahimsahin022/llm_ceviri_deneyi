/*
 * Kok Neden H (switch/case fallthrough'in kaybolmasi) icin 2. bagimsiz
 * ornek (s43_switch_fallthrough'dan farkli bir sayisal senaryo: bonus
 * puan degil, kumulatif vergi dilimi hesaplama - klasik bir "bracket"
 * deseni, ayni kok mekanizmayi [kasitli dusme ile kumulatif toplama]
 * bagimsiz bir programda sinar).
 *
 * C'nin switch'i, break konulmadigunda bilincli olarak bir sonraki case'e
 * duser: dilim (bracket) N ise, N,N-1,...,1 dilimlerinin hepsinin vergisi
 * birikir. Rust'in match'i varsayilan olarak dusmez; LLM her dilimi
 * yalnizca kendi (kumulatif olmayan) katkisiyla eslestirebilir.
 */
#include <stdio.h>

int main(void)
{
    int bracket;
    if (scanf("%d", &bracket) != 1) return 1;

    int tax = 0;
    switch (bracket) {
        case 4: tax += 800;  /* dusme (fallthrough) kasitli */
        case 3: tax += 400;
        case 2: tax += 200;
        case 1: tax += 100;
                break;
        default: tax = 0;
    }
    printf("%d\n", tax);
    return 0;
}

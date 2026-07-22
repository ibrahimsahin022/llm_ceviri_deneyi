/*
 * Kok Neden C (char isaretliligi) icin 2. bagimsiz ornek (s20_char_sum'dan
 * farkli bir desen: toplama degil SINIFLANDIRMA/SAYMA).
 *
 * C'de `char` cogu platformda (bu derleme ortami dahil) ISARETLIDIR; 127'den
 * buyuk bayt degerleri (cok baytli UTF-8 devam baytlari gibi) `char`'a
 * atandiginda NEGATIF sayilir. Bu program, stdin'den okunan metindeki
 * "negatif char" sayisini sayar - dogal bir Rust cevirisi baytlari `u8`
 * (0..255, hep pozitif) olarak okursa bu sayim hep 0 cikar.
 */
#include <stdio.h>

int main(void)
{
    int c;
    int negative_count = 0;
    int total = 0;
    while ((c = getchar()) != EOF) {
        char ch = (char)c;
        if (ch < 0) {
            negative_count++;
        }
        total++;
    }
    printf("%d %d\n", total, negative_count);
    return 0;
}

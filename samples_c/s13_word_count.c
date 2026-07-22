#include <stdio.h>

/* Girdi: rastgele metin (EOF'a kadar).
   Cikti: "kelime_sayisi karakter_sayisi"
   Not: C karakter sayimini BAYT bazinda yapar. */
int main(void) {
    int c, chars = 0, words = 0, inword = 0;
    while ((c = getchar()) != EOF) {
        chars++;
        if (c == ' ' || c == '\n' || c == '\t' || c == '\r') {
            inword = 0;
        } else if (!inword) {
            words++;
            inword = 1;
        }
    }
    printf("%d %d\n", words, chars);
    return 0;
}

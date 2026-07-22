#include <stdio.h>

/* Girdi: tek satir metin.
   Cikti: metindeki kucuk harflerin frekansi, "harf:adet" bicinde,
   a'dan z'ye sirali (yalnizca gecen harfler yazdirilir). */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    int freq[26] = {0};
    for (int i = 0; buf[i] != '\0'; i++) {
        char c = buf[i];
        if (c >= 'a' && c <= 'z') freq[c - 'a']++;
    }
    for (int i = 0; i < 26; i++) {
        if (freq[i] > 0) printf("%c:%d\n", 'a' + i, freq[i]);
    }
    return 0;
}

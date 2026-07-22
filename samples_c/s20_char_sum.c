#include <stdio.h>

/* Girdi: tek satir metin.
   Cikti: karakter (bayt) degerlerinin toplami.
   NOT: C'de 'char' cogu platformda ISARETLIDIR; degeri 127'den buyuk
   baytlar NEGATIF sayilir. Bu, toplama dogrudan etki eder. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    long sum = 0;
    for (int i = 0; buf[i] != '\0'; i++) {
        if (buf[i] == '\n' || buf[i] == '\r') break;
        sum += buf[i];   /* isaretli char: yuksek baytlar negatif katkida bulunur */
    }
    printf("%ld\n", sum);
    return 0;
}

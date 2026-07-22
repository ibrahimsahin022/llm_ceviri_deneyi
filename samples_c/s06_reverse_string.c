#include <stdio.h>
#include <string.h>

/* Girdi: tek satir metin (en fazla 1000 karakter).
   Cikti: metnin ters cevrilmis hali. */
int main(void) {
    char buf[1024];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    size_t len = strlen(buf);
    /* satir sonu karakterini at */
    while (len > 0 && (buf[len-1] == '\n' || buf[len-1] == '\r')) {
        buf[--len] = '\0';
    }
    for (size_t i = 0; i < len; i++) {
        putchar(buf[len - 1 - i]);
    }
    putchar('\n');
    return 0;
}

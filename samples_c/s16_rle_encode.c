#include <stdio.h>
#include <string.h>

/* Girdi: tek satir metin.
   Cikti: calisma-uzunlugu kodlamasi (RLE). Ornek: "aaabbc" -> "a3b2c1". */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    size_t len = strlen(buf);
    while (len > 0 && (buf[len-1] == '\n' || buf[len-1] == '\r')) {
        buf[--len] = '\0';
    }
    size_t i = 0;
    while (i < len) {
        size_t j = i;
        while (j < len && buf[j] == buf[i]) j++;
        printf("%c%zu", buf[i], j - i);
        i = j;
    }
    printf("\n");
    return 0;
}

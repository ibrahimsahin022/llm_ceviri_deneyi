#include <stdio.h>
#include <string.h>

/* Girdi: tek satir, bosluklarla ayrilmis kelimeler (cok baytli olabilir).
   Cikti: her kelime "idx: kelime" bicimiyle ayri satirda.
   NOT: Kok Neden B (string modeli) icin 4. bagimsiz ornek. strtok() bayt
   duzeyinde bosluk arar; Rust'ta dogal karsiligi split_whitespace() char
   sinirlarina gore calisir - cok baytli karakterlerin bosluga bitisik
   oldugu kenar durumlarinda farkli davranabilir. */
int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    size_t len = strlen(buf);
    while (len > 0 && (buf[len - 1] == '\n' || buf[len - 1] == '\r')) {
        buf[--len] = '\0';
    }
    int idx = 0;
    char *tok = strtok(buf, " ");
    while (tok != NULL) {
        printf("%d: %s\n", idx, tok);
        idx++;
        tok = strtok(NULL, " ");
    }
    return 0;
}

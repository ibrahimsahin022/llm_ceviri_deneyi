#include <stdio.h>

/* Girdi: ilk satirda kaydirma miktari k, ikinci satirda metin.
   Cikti: Caesar sifrelemesi uygulanmis metin (sadece harfler kaydirilir). */
int main(void) {
    int k;
    if (scanf("%d\n", &k) != 1) return 0;
    k = ((k % 26) + 26) % 26;
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    for (int i = 0; buf[i] != '\0'; i++) {
        char c = buf[i];
        if (c >= 'a' && c <= 'z') {
            putchar('a' + (c - 'a' + k) % 26);
        } else if (c >= 'A' && c <= 'Z') {
            putchar('A' + (c - 'A' + k) % 26);
        } else if (c != '\n' && c != '\r') {
            putchar(c);
        }
    }
    putchar('\n');
    return 0;
}

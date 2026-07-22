#include <stdio.h>
#include <string.h>
#include "config.h"

/* Girdi bicimi:
 *   N (config satiri sayisi)
 *   N satir "key=value" (veya bos/# yorum satiri)
 *   M (sorgu sayisi)
 *   M satir key
 * Her sorgu icin value ya da "NOT_FOUND" yazdirilir. */
int main(void)
{
    int n;
    if (scanf("%d", &n) != 1) return 1;
    getchar();

    char text[4096] = {0};
    size_t used = 0;
    char line[256];
    for (int i = 0; i < n; i++) {
        if (!fgets(line, sizeof(line), stdin)) break;
        size_t l = strlen(line);
        if (used + l < sizeof(text) - 1) {
            memcpy(text + used, line, l);
            used += l;
        }
    }
    text[used] = '\0';

    ConfigEntry entries[MAX_ENTRIES];
    int count = parse_config_lines(text, entries, MAX_ENTRIES);

    int m;
    if (scanf("%d", &m) != 1) return 1;
    for (int i = 0; i < m; i++) {
        char key[MAX_KEY_LEN];
        if (scanf("%31s", key) != 1) break;
        const char *val = config_lookup(entries, count, key);
        printf("%s\n", val ? val : "NOT_FOUND");
    }
    return 0;
}

#include <string.h>
#include <ctype.h>
#include "config.h"

static void trim(char *s)
{
    size_t len = strlen(s);
    while (len > 0 && (s[len - 1] == '\n' || s[len - 1] == '\r' || s[len - 1] == ' ')) {
        s[--len] = '\0';
    }
    size_t start = 0;
    while (s[start] == ' ') start++;
    if (start > 0) {
        memmove(s, s + start, strlen(s + start) + 1);
    }
}

/* "key=value" satirlarindan olusan metni ayristirir; bos satirlari ve
 * '#' ile baslayan yorum satirlarini atlar. Doldurulan giris sayisini
 * dondurur. */
int parse_config_lines(const char *text, ConfigEntry entries[], int max_entries)
{
    int count = 0;
    const char *p = text;
    char line[MAX_KEY_LEN + MAX_VAL_LEN + 4];

    while (*p && count < max_entries) {
        size_t i = 0;
        while (*p && *p != '\n' && i < sizeof(line) - 1) {
            line[i++] = *p++;
        }
        line[i] = '\0';
        if (*p == '\n') p++;

        trim(line);
        if (line[0] == '\0' || line[0] == '#') {
            continue;
        }

        char *eq = strchr(line, '=');
        if (!eq) {
            continue;
        }
        *eq = '\0';
        char *key = line;
        char *value = eq + 1;
        trim(key);
        trim(value);

        strncpy(entries[count].key, key, MAX_KEY_LEN - 1);
        entries[count].key[MAX_KEY_LEN - 1] = '\0';
        strncpy(entries[count].value, value, MAX_VAL_LEN - 1);
        entries[count].value[MAX_VAL_LEN - 1] = '\0';
        count++;
    }
    return count;
}

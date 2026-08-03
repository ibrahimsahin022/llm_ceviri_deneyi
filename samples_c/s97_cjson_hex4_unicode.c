#include <stdio.h>
#include <string.h>

/* Kaynak: cJSON (DaveGamble/cJSON) cJSON.c, parse_hex4() - \uXXXX unicode
   kacis cozumleyicisi, birebir.
   https://raw.githubusercontent.com/DaveGamble/cJSON/master/cJSON.c
   Lisans: MIT.
   Girdi: bir satir, 4 hex karakter (orn. "00e7" veya "1F600" - fazla
   karakter yoksayilir, ilk 4'u okunur).
   Cikti: cozumlenen deger, ondalik. */

static unsigned parse_hex4(const unsigned char *input) {
    unsigned int h = 0;
    size_t i = 0;

    for (i = 0; i < 4; i++) {
        if ((input[i] >= '0') && (input[i] <= '9')) {
            h += (unsigned int)input[i] - '0';
        } else if ((input[i] >= 'A') && (input[i] <= 'F')) {
            h += (unsigned int)10 + input[i] - 'A';
        } else if ((input[i] >= 'a') && (input[i] <= 'f')) {
            h += (unsigned int)10 + input[i] - 'a';
        } else {
            return 0;
        }

        if (i < 3) {
            h = h << 4;
        }
    }

    return h;
}

int main(void) {
    char line[64];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    unsigned r = parse_hex4((const unsigned char *)line);
    printf("%u\n", r);
    return 0;
}

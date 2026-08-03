#include <stdio.h>
#include <ctype.h>
#include <string.h>

/* Girdi: tek satir, basit JSON-benzeri metin (yalniz {, }, [, ], :, ,,
   tirnakli string "...", sayi, true/false/null iceriyor - cJSON DEGIL,
   ozgun basit bir tokenizer).
   Cikti: her token kendi satirinda, "TUR:deger" bicimiyle (TUR: BRACE,
   BRACKET, COLON, COMMA, STRING, NUMBER, BOOL, NULL). */

static const char *p;

static void emit_string(void) {
    p++; /* acilis tirnak */
    char buf[256];
    int len = 0;
    while (*p && *p != '"' && len < 255) {
        buf[len++] = *p++;
    }
    buf[len] = 0;
    if (*p == '"') p++;
    printf("STRING:%s\n", buf);
}

static void emit_number(void) {
    char buf[64];
    int len = 0;
    while ((isdigit((unsigned char)*p) || *p == '-' || *p == '.') && len < 63) {
        buf[len++] = *p++;
    }
    buf[len] = 0;
    printf("NUMBER:%s\n", buf);
}

int main(void) {
    char line[1024];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    p = line;

    while (*p) {
        if (*p == ' ' || *p == '\t') {
            p++;
        } else if (*p == '{' || *p == '}') {
            printf("BRACE:%c\n", *p);
            p++;
        } else if (*p == '[' || *p == ']') {
            printf("BRACKET:%c\n", *p);
            p++;
        } else if (*p == ':') {
            printf("COLON::\n");
            p++;
        } else if (*p == ',') {
            printf("COMMA:,\n");
            p++;
        } else if (*p == '"') {
            emit_string();
        } else if (isdigit((unsigned char)*p) || *p == '-') {
            emit_number();
        } else if (strncmp(p, "true", 4) == 0) {
            printf("BOOL:true\n");
            p += 4;
        } else if (strncmp(p, "false", 5) == 0) {
            printf("BOOL:false\n");
            p += 5;
        } else if (strncmp(p, "null", 4) == 0) {
            printf("NULL:null\n");
            p += 4;
        } else {
            p++;
        }
    }

    return 0;
}

#include <stdio.h>
#include <string.h>
#include <ctype.h>

/* Girdi: tek satir boolean ifade (0/1 degerler, AND/OR/NOT operatorleri,
   parantez). Ozyinelemeli-inisli ayristirici - s18_expr_eval'in FARKLI
   bir grameri (aritmetik degil, mantiksal; NOT > AND > OR onceligi).
   Cikti: ifadenin sonucu (0 veya 1).
   Ornekler: "1 AND 0" -> 0 ; "1 OR 0 AND 0" -> 1 (AND OR'dan once) ;
   "NOT 0 AND 1" -> 1 ; "NOT (1 AND 0)" -> 1 */

static const char *p;

static void skip_spaces(void) {
    while (*p == ' ' || *p == '\t') p++;
}

static int match_word(const char *word) {
    size_t len = strlen(word);
    if (strncmp(p, word, len) == 0) {
        p += len;
        return 1;
    }
    return 0;
}

static int parse_or(void);

/* factor := '0' | '1' | '(' or_expr ')' | 'NOT' factor */
static int parse_factor(void) {
    skip_spaces();
    if (*p == '(') {
        p++;
        int v = parse_or();
        skip_spaces();
        if (*p == ')') p++;
        return v;
    }
    if (match_word("NOT")) {
        int v = parse_factor();
        return !v;
    }
    if (*p == '0') { p++; return 0; }
    if (*p == '1') { p++; return 1; }
    return 0;
}

/* and_expr := factor ('AND' factor)* */
static int parse_and(void) {
    int value = parse_factor();
    for (;;) {
        skip_spaces();
        if (match_word("AND")) {
            int rhs = parse_factor();
            value = value && rhs;
        } else {
            break;
        }
    }
    return value;
}

/* or_expr := and_expr ('OR' and_expr)* */
static int parse_or(void) {
    int value = parse_and();
    for (;;) {
        skip_spaces();
        if (match_word("OR")) {
            int rhs = parse_and();
            value = value || rhs;
        } else {
            break;
        }
    }
    return value;
}

int main(void) {
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    p = buf;
    int result = parse_or();
    printf("%d\n", result);
    return 0;
}

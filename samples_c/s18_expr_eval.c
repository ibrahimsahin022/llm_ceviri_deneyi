#include <stdio.h>
#include <string.h>
#include <ctype.h>

/* Girdi: tek satir matematiksel ifade (tam sayilar, + - * / ve parantez).
   Cikti: ifadenin tam sayi sonucu (tam sayi bolme, sifira dogru kirpar).
   Ornekler: "2+3*4" -> 14 ; "(2+3)*4" -> 20 ; "10-2-3" -> 5 ; "-(4+1)*2" -> -10
   Ozyinelemeli inis (recursive descent) ayristirici. */

static const char *p;  /* girdideki gecerli konum */

static void skip_spaces(void) {
    while (*p == ' ' || *p == '\t') p++;
}

static long long parse_expr(void);

/* factor := number | '(' expr ')' | '-' factor | '+' factor */
static long long parse_factor(void) {
    skip_spaces();
    if (*p == '(') {
        p++;  /* '(' */
        long long v = parse_expr();
        skip_spaces();
        if (*p == ')') p++;  /* ')' */
        return v;
    }
    if (*p == '-') {
        p++;
        return -parse_factor();
    }
    if (*p == '+') {
        p++;
        return parse_factor();
    }
    long long num = 0;
    while (isdigit((unsigned char)*p)) {
        num = num * 10 + (*p - '0');
        p++;
    }
    return num;
}

/* term := factor (('*' | '/') factor)* */
static long long parse_term(void) {
    long long value = parse_factor();
    for (;;) {
        skip_spaces();
        if (*p == '*') {
            p++;
            value = value * parse_factor();
        } else if (*p == '/') {
            p++;
            long long d = parse_factor();
            value = value / d;
        } else {
            break;
        }
    }
    return value;
}

/* expr := term (('+' | '-') term)* */
static long long parse_expr(void) {
    long long value = parse_term();
    for (;;) {
        skip_spaces();
        if (*p == '+') {
            p++;
            value = value + parse_term();
        } else if (*p == '-') {
            p++;
            value = value - parse_term();
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
    long long result = parse_expr();
    printf("%lld\n", result);
    return 0;
}

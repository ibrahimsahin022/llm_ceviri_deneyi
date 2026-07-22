#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Girdi: tek satir, bosluklarla ayrilmis ters-Polonya (postfix/RPN) ifade.
   Sayilar tam sayi olabilir (negatif dahil), islecler: + - * /.
   Cikti: ifadenin tam sayi sonucu (tam sayi bolme, sifira dogru kirpar).
   Ornek: "3 4 +" -> 7 ; "5 1 2 + 4 * + 3 -" -> 14 ; "10 2 /" -> 5 */

#define STACK_CAP 256

static long stack[STACK_CAP];
static int sp = 0;

static void push(long v) {
    if (sp < STACK_CAP) stack[sp++] = v;
}

static long pop(void) {
    if (sp == 0) return 0;
    return stack[--sp];
}

static int is_operator(const char *tok) {
    return (strcmp(tok, "+") == 0 || strcmp(tok, "-") == 0 ||
            strcmp(tok, "*") == 0 || strcmp(tok, "/") == 0);
}

static int is_number(const char *tok) {
    int i = 0;
    if (tok[0] == '-' || tok[0] == '+') i = 1;
    if (tok[i] == '\0') return 0;
    for (; tok[i] != '\0'; i++) {
        if (tok[i] < '0' || tok[i] > '9') return 0;
    }
    return 1;
}

static long apply_op(const char *op, long a, long b) {
    if (strcmp(op, "+") == 0) return a + b;
    if (strcmp(op, "-") == 0) return a - b;
    if (strcmp(op, "*") == 0) return a * b;
    if (strcmp(op, "/") == 0) {
        if (b == 0) return 0;
        return a / b;
    }
    return 0;
}

int main(void) {
    char line[4096];
    if (fgets(line, sizeof(line), stdin) == NULL) return 0;

    char *tok = strtok(line, " \t\n");
    while (tok != NULL) {
        if (is_number(tok)) {
            push(atol(tok));
        } else if (is_operator(tok)) {
            long b = pop();
            long a = pop();
            push(apply_op(tok, a, b));
        }
        tok = strtok(NULL, " \t\n");
    }

    long result = pop();
    printf("%ld\n", result);
    return 0;
}

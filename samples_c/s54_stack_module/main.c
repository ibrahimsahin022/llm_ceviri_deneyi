#include <stdio.h>
#include <string.h>
#include "stack.h"

/* Girdi bicimi: ilk satir komut sayisi N, sonra N satir komut:
 *   PUSH <deger>
 *   POP
 *   PEEK
 *   SIZE
 * Her komuttan sonra sonuc bir satirda yazdirilir. */
int main(void)
{
    Stack s;
    stack_init(&s);

    int n;
    if (scanf("%d", &n) != 1) return 1;

    for (int i = 0; i < n; i++) {
        char cmd[16];
        if (scanf("%15s", cmd) != 1) break;

        if (strcmp(cmd, "PUSH") == 0) {
            int v;
            if (scanf("%d", &v) != 1) break;
            int ok = stack_push(&s, v);
            printf(ok ? "OK\n" : "FULL\n");
        } else if (strcmp(cmd, "POP") == 0) {
            int v;
            if (stack_pop(&s, &v)) {
                printf("%d\n", v);
            } else {
                printf("EMPTY\n");
            }
        } else if (strcmp(cmd, "PEEK") == 0) {
            int v;
            if (stack_peek(&s, &v)) {
                printf("%d\n", v);
            } else {
                printf("EMPTY\n");
            }
        } else if (strcmp(cmd, "SIZE") == 0) {
            printf("%d\n", stack_size(&s));
        }
    }
    return 0;
}

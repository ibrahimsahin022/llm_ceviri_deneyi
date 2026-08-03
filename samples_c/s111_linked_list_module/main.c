#include <stdio.h>
#include "list.h"

/* Girdi: ilk satirda islem sayisi n, ardindan n satir - "P <deger>" (push
   front), "R <deger>" (remove), "C <deger>" (contains).
   Cikti: R icin "REMOVED"/"NOTFOUND"; C icin "YES"/"NO". Sonda listenin
   son boyutu. */

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    List l;
    list_init(&l);

    for (int i = 0; i < n; i++) {
        char op[4];
        int v;
        if (scanf("%3s %d", op, &v) != 2) return 0;
        if (op[0] == 'P') {
            list_push_front(&l, v);
        } else if (op[0] == 'R') {
            printf("%s\n", list_remove(&l, v) ? "REMOVED" : "NOTFOUND");
        } else {
            printf("%s\n", list_contains(&l, v) ? "YES" : "NO");
        }
    }

    printf("size=%d\n", list_size(&l));
    list_free(&l);
    return 0;
}

#include <stdio.h>

/* Girdi: tek bir tam sayi n.
   Cikti: 1'den n'e kadar sayilar (her biri ayri satirda).
   Ozellik: sayac, global (static) DEGISKEN durumda tutulur ve
   next_id() fonksiyonu her cagrildiginda artirilir. */
static int call_count = 0;   /* global degistirilebilir durum */

int next_id(void) {
    call_count++;
    return call_count;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    for (int i = 0; i < n; i++) {
        printf("%d\n", next_id());
    }
    return 0;
}

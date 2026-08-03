#include <stdio.h>
#include "queue.h"

/* Girdi: ilk satirda islem sayisi n, ardindan n satir - her biri
   "E <deger>" (enqueue) veya "D" (dequeue).
   Cikti: her islem icin bir satir - enqueue basariliysa "OK", kapasite
   doluysa "FULL"; dequeue basariliysa deger, kuyruk bosşa "EMPTY".
   Sonda kuyrugun son boyutu. */

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    Queue q;
    queue_init(&q);

    for (int i = 0; i < n; i++) {
        char op[4];
        if (scanf("%3s", op) != 1) return 0;
        if (op[0] == 'E') {
            int v;
            if (scanf("%d", &v) != 1) return 0;
            if (queue_enqueue(&q, v)) {
                printf("OK\n");
            } else {
                printf("FULL\n");
            }
        } else {
            int out;
            if (queue_dequeue(&q, &out)) {
                printf("%d\n", out);
            } else {
                printf("EMPTY\n");
            }
        }
    }

    printf("size=%d\n", queue_size(&q));
    return 0;
}

#include <stdio.h>
#include <pthread.h>
#include "buffer.h"

/* Girdi: bir tam sayi N (uretilecek/tuketilecek eleman sayisi, 1..N).
   Cikti: tuketicinin topladigi toplam (N*(N+1)/2 ile ayni - zamanlamadan
   BAGIMSIZ, deterministik). */

typedef struct {
    BoundedBuffer *buf;
    int n;
} ProducerArgs;

typedef struct {
    BoundedBuffer *buf;
    int n;
    long sum;
} ConsumerArgs;

static void *producer(void *arg) {
    ProducerArgs *pa = (ProducerArgs *)arg;
    for (int i = 1; i <= pa->n; i++) {
        buffer_put(pa->buf, i);
    }
    return NULL;
}

static void *consumer(void *arg) {
    ConsumerArgs *ca = (ConsumerArgs *)arg;
    ca->sum = 0;
    for (int i = 0; i < ca->n; i++) {
        ca->sum += buffer_get(ca->buf);
    }
    return NULL;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1 || n < 0 || n > 100000) return 0;

    BoundedBuffer buf;
    buffer_init(&buf);

    ProducerArgs pa = { &buf, n };
    ConsumerArgs ca = { &buf, n, 0 };

    pthread_t prod_thread, cons_thread;
    pthread_create(&prod_thread, NULL, producer, &pa);
    pthread_create(&cons_thread, NULL, consumer, &ca);
    pthread_join(prod_thread, NULL);
    pthread_join(cons_thread, NULL);

    buffer_destroy(&buf);
    printf("%ld\n", ca.sum);
    return 0;
}

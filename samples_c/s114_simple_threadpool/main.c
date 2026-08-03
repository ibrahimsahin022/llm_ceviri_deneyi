#include <stdio.h>
#include <pthread.h>
#include "pool.h"

/* Girdi: iki tam sayi - n_tasks (gorev sayisi), n_workers (isci parcaciği
   sayisi).
   Cikti: 1^2 + 2^2 + ... + n_tasks^2 toplami (isci sayisindan/zamanlamadan
   BAGIMSIZ, deterministik). */

static void *worker(void *arg) {
    TaskPool *p = (TaskPool *)arg;
    int task_id;
    while (pool_next_task(p, &task_id)) {
        long v = (long)(task_id + 1) * (long)(task_id + 1);
        pool_add_result(p, v);
    }
    return NULL;
}

int main(void) {
    int n_tasks, n_workers;
    if (scanf("%d %d", &n_tasks, &n_workers) != 2) return 1;
    if (n_tasks < 0 || n_tasks > 100000 || n_workers <= 0 || n_workers > 64) return 1;

    TaskPool pool;
    pool_init(&pool, n_tasks);

    pthread_t threads[64];
    for (int i = 0; i < n_workers; i++) {
        pthread_create(&threads[i], NULL, worker, &pool);
    }
    for (int i = 0; i < n_workers; i++) {
        pthread_join(threads[i], NULL);
    }

    printf("%ld\n", pool_total(&pool));
    pool_destroy(&pool);
    return 0;
}

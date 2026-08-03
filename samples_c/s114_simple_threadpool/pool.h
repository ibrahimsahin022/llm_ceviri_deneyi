/*
 * Grup 3 (cok-dosyali/cesitlilik) - basit bir "is parcaciği havuzu"
 * (thread pool) deseni: W isci parcaciği, paylasilan bir "sonraki gorev
 * indeksi" sayacindan gorev cekip (mutex ile korunarak) islerler,
 * sonuclari yine mutex ile korunan ortak bir toplam degiskene eklerler.
 * Gorev sayisi N, isci sayisi W'den BAGIMSIZ olarak islenir (W, N'den
 * fazla veya az olabilir) - bu, is bolusumunun ZAMANLAMAYA BAGLI ama
 * SONUCUN DETERMINISTIK oldugu klasik bir desendir.
 */
#ifndef POOL_H
#define POOL_H

#include <pthread.h>

typedef struct {
    int next_task;
    int n_tasks;
    long total;
    pthread_mutex_t lock;
} TaskPool;

void pool_init(TaskPool *p, int n_tasks);
/* Sirada gorev varsa 1 doner ve *task_id doldurulur, yoksa 0 doner. */
int pool_next_task(TaskPool *p, int *task_id);
void pool_add_result(TaskPool *p, long value);
long pool_total(const TaskPool *p);
void pool_destroy(TaskPool *p);

#endif

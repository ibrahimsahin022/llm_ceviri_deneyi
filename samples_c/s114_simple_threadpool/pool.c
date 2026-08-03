#include "pool.h"

void pool_init(TaskPool *p, int n_tasks) {
    p->next_task = 0;
    p->n_tasks = n_tasks;
    p->total = 0;
    pthread_mutex_init(&p->lock, NULL);
}

int pool_next_task(TaskPool *p, int *task_id) {
    int got = 0;
    pthread_mutex_lock(&p->lock);
    if (p->next_task < p->n_tasks) {
        *task_id = p->next_task;
        p->next_task++;
        got = 1;
    }
    pthread_mutex_unlock(&p->lock);
    return got;
}

void pool_add_result(TaskPool *p, long value) {
    pthread_mutex_lock(&p->lock);
    p->total += value;
    pthread_mutex_unlock(&p->lock);
}

long pool_total(const TaskPool *p) {
    return p->total;
}

void pool_destroy(TaskPool *p) {
    pthread_mutex_destroy(&p->lock);
}

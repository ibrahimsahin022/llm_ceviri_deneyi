#include "rwstate.h"

void rwstate_init(RwState *s) {
    s->counter = 0;
    pthread_rwlock_init(&s->lock, NULL);
}

void rwstate_increment(RwState *s) {
    pthread_rwlock_wrlock(&s->lock);
    s->counter += 1;
    pthread_rwlock_unlock(&s->lock);
}

long rwstate_read(RwState *s) {
    pthread_rwlock_rdlock(&s->lock);
    long v = s->counter;
    pthread_rwlock_unlock(&s->lock);
    return v;
}

void rwstate_destroy(RwState *s) {
    pthread_rwlock_destroy(&s->lock);
}

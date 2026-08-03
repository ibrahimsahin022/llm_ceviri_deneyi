#include "buffer.h"

void buffer_init(BoundedBuffer *b) {
    b->count = 0;
    b->in = 0;
    b->out = 0;
    pthread_mutex_init(&b->lock, NULL);
    pthread_cond_init(&b->not_full, NULL);
    pthread_cond_init(&b->not_empty, NULL);
}

void buffer_put(BoundedBuffer *b, int value) {
    pthread_mutex_lock(&b->lock);
    while (b->count == BUFFER_CAPACITY) {
        pthread_cond_wait(&b->not_full, &b->lock);
    }
    b->data[b->in] = value;
    b->in = (b->in + 1) % BUFFER_CAPACITY;
    b->count++;
    pthread_cond_signal(&b->not_empty);
    pthread_mutex_unlock(&b->lock);
}

int buffer_get(BoundedBuffer *b) {
    pthread_mutex_lock(&b->lock);
    while (b->count == 0) {
        pthread_cond_wait(&b->not_empty, &b->lock);
    }
    int value = b->data[b->out];
    b->out = (b->out + 1) % BUFFER_CAPACITY;
    b->count--;
    pthread_cond_signal(&b->not_full);
    pthread_mutex_unlock(&b->lock);
    return value;
}

void buffer_destroy(BoundedBuffer *b) {
    pthread_mutex_destroy(&b->lock);
    pthread_cond_destroy(&b->not_full);
    pthread_cond_destroy(&b->not_empty);
}

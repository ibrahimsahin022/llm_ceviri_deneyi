/*
 * Grup 3 (cok-dosyali/cesitlilik) - s57_shared_counter_threads'in devami:
 * mutex TEK BASINA yetmez, pthread_cond_wait/pthread_cond_signal ile
 * "tampon dolu/bos" durumunu bekleme. Amac: Rust'a cevrilirken
 * Arc<Mutex<T>> + Condvar'in (std::sync::Condvar) dogru eslesip
 * eslesmeyecegini sinamak - naif bir ceviri busy-wait'e (sürekli
 * kilit alip birakma) kayabilir, bu da YANLIS DEGIL ama YAVAS ve orijinal
 * "condvar ile bekleme" semantiginden sapmadir (test sonucu ayni kalir,
 * ilginc olan model'in secimidir).
 */
#ifndef BUFFER_H
#define BUFFER_H

#include <pthread.h>

#define BUFFER_CAPACITY 4

typedef struct {
    int data[BUFFER_CAPACITY];
    int count;
    int in;
    int out;
    pthread_mutex_t lock;
    pthread_cond_t not_full;
    pthread_cond_t not_empty;
} BoundedBuffer;

void buffer_init(BoundedBuffer *b);
void buffer_put(BoundedBuffer *b, int value);
int buffer_get(BoundedBuffer *b);
void buffer_destroy(BoundedBuffer *b);

#endif

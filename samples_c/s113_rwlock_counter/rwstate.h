/*
 * Grup 3 (cok-dosyali/cesitlilik) - pthread_rwlock_t (coklu okuyucu/tekli
 * yazici) deseni. Amac: Rust'in std::sync::RwLock ile eslesip
 * eslesmeyecegini sinamak. Deterministik sonuc icin: N yazici thread'i
 * sirayla (RWLOCK ile birbirini dislayarak) sayaci artirir, M okuyucu
 * thread'i ise yalnizca son degeri okur (okuma sirasi sonucu etkilemez,
 * sadece son deger onemlidir - artis islemleri RWLOCK write-lock ile
 * serialize edildigi icin toplam N*increments deterministiktir).
 */
#ifndef RWSTATE_H
#define RWSTATE_H

#include <pthread.h>

typedef struct {
    long counter;
    pthread_rwlock_t lock;
} RwState;

void rwstate_init(RwState *s);
void rwstate_increment(RwState *s);
long rwstate_read(RwState *s);
void rwstate_destroy(RwState *s);

#endif

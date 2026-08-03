#include <stdio.h>
#include <pthread.h>
#include "rwstate.h"

/* Girdi: iki tam sayi - n_writers (yazici sayisi), increments_per_writer
   (her yazicinin artis sayisi).
   Cikti: tum yazicilar bittikten sonraki sayac degeri
   (n_writers * increments_per_writer ile ayni - deterministik). */

typedef struct {
    RwState *state;
    int increments;
} WriterArgs;

static void *writer(void *arg) {
    WriterArgs *wa = (WriterArgs *)arg;
    for (int i = 0; i < wa->increments; i++) {
        rwstate_increment(wa->state);
    }
    return NULL;
}

int main(void) {
    int n_writers, increments;
    if (scanf("%d %d", &n_writers, &increments) != 2) return 1;
    if (n_writers <= 0 || n_writers > 64 || increments < 0) return 1;

    RwState state;
    rwstate_init(&state);

    pthread_t threads[64];
    WriterArgs args[64];

    for (int i = 0; i < n_writers; i++) {
        args[i].state = &state;
        args[i].increments = increments;
        pthread_create(&threads[i], NULL, writer, &args[i]);
    }
    for (int i = 0; i < n_writers; i++) {
        pthread_join(threads[i], NULL);
    }

    printf("%ld\n", rwstate_read(&state));
    rwstate_destroy(&state);
    return 0;
}

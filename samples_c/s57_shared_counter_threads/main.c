/*
 * Faz 3 devami - paylasilan bellek eszamanliligi (shared memory concurrency).
 *
 * N is parcaciği (thread), AYNI paylasilan struct'i (bir sayac + bir mutex)
 * gorur; her biri sayaci M kez, mutex ile korunarak artirir. Sonuc
 * (N*M) zamanlamadan (scheduling) BAGIMSIZ, deterministik oldugu icin
 * stdin/stdout diferansiyel test harness'imizle uyumludur - ama ic
 * yurutme GERCEKTEN es zamanlidir (birden fazla OS thread'i, ayni bellek
 * bolgesine yariş halinde erisir; yalnizca mutex bunu guvenli kilar).
 *
 * Bu, C'nin izin verdigi ama Rust'in tip sisteminin farkli sekilde ele
 * aldigi bir deseni test eder: C'de paylasilan bir global/isaretci
 * uzerinden struct'a is parcaciklari arasinda serbestce erisilebilir
 * (programci mutex kullanmayi UNUTURSA bile derleyici bunu engellemez -
 * veri yarisi = tanimsiz davranis, ama SESSIZCE derlenir). Rust'ta ise
 * ayni paylasimi ifade etmek icin Arc<Mutex<T>> (veya benzeri Send+Sync
 * bir soyutlama) KULLANMAK NEREDEYSE ZORUNLUDUR - naif bir "ham paylasilan
 * degisken" cevirisi thread::spawn sinirini asamaz ve DERLENMEZ. Yani
 * burada ilginc soru: LLM dogru Arc<Mutex<>> desenini mi bulacak, yoksa
 * ilk denemede derleme hatasi mi alacak?
 */
#include <stdio.h>
#include <pthread.h>

typedef struct {
    long counter;
    pthread_mutex_t lock;
} SharedState;

typedef struct {
    SharedState *state;
    int increments;
} WorkerArgs;

static void *worker(void *arg)
{
    WorkerArgs *wa = (WorkerArgs *)arg;
    for (int i = 0; i < wa->increments; i++) {
        pthread_mutex_lock(&wa->state->lock);
        wa->state->counter += 1;
        pthread_mutex_unlock(&wa->state->lock);
    }
    return NULL;
}

int main(void)
{
    int n_threads, increments;
    if (scanf("%d %d", &n_threads, &increments) != 2) return 1;
    if (n_threads <= 0 || n_threads > 64 || increments < 0) return 1;

    SharedState state;
    state.counter = 0;
    pthread_mutex_init(&state.lock, NULL);

    pthread_t threads[64];
    WorkerArgs args[64];

    for (int i = 0; i < n_threads; i++) {
        args[i].state = &state;
        args[i].increments = increments;
        pthread_create(&threads[i], NULL, worker, &args[i]);
    }
    for (int i = 0; i < n_threads; i++) {
        pthread_join(threads[i], NULL);
    }

    pthread_mutex_destroy(&state.lock);
    printf("%ld\n", state.counter);
    return 0;
}

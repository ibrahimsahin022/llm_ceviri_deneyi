#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Girdi: ilk satirda n (kayit sayisi) ve mod ("NAME" veya "SCORE"),
   ardindan n satir "<isim> <puan>".
   Cikti: kayitlar mod'a gore SIRALANMIS halde, her biri kendi satirinda
   "<isim> <puan>".
   NOT: stdlib qsort() + struct karsilastirici (comparator) callback -
   Rust'ta bu, `slice::sort_by` veya `sort_by_key` ile dogal olarak
   eslesir, ama C'nin "iki void* alan, calisma-zamaninda secilen
   karsilastirici fonksiyonu" deseni (mod'a gore FARKLI qsort cagirisi)
   Rust'a fonksiyon degeri/kapatma (closure) olarak mi, yoksa if/else ile
   iki ayri sort cagrisi olarak mi cevrilecegi ilginc bir tasarim
   secimidir. */

typedef struct {
    char name[32];
    int score;
} Record;

static int cmp_by_name(const void *a, const void *b) {
    const Record *ra = (const Record *)a;
    const Record *rb = (const Record *)b;
    return strcmp(ra->name, rb->name);
}

static int cmp_by_score(const void *a, const void *b) {
    const Record *ra = (const Record *)a;
    const Record *rb = (const Record *)b;
    if (ra->score != rb->score) return ra->score - rb->score;
    return strcmp(ra->name, rb->name);
}

int main(void) {
    int n;
    char mode[8];
    if (scanf("%d %7s", &n, mode) != 2) return 0;
    if (n < 0 || n > 1000) return 0;

    Record *records = malloc(sizeof(Record) * n);
    for (int i = 0; i < n; i++) {
        if (scanf("%31s %d", records[i].name, &records[i].score) != 2) return 0;
    }

    if (strcmp(mode, "NAME") == 0) {
        qsort(records, n, sizeof(Record), cmp_by_name);
    } else {
        qsort(records, n, sizeof(Record), cmp_by_score);
    }

    for (int i = 0; i < n; i++) {
        printf("%s %d\n", records[i].name, records[i].score);
    }

    free(records);
    return 0;
}

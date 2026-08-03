#include <stdio.h>

/* Girdi: ilk satirda islem sayisi n, ardindan n satir - "I <deger>"
   (ekle) veya "X" (en kucugu cikar).
   Cikti: X icin cikarilan deger, yigin bosşa "EMPTY". Sonda yigindaki
   eleman sayisi.
   Min-heap (ikili yigin), dizi tabanli (index i'nin cocuklari 2i+1,
   2i+2). */

#define MAX_HEAP 1000

static int heap[MAX_HEAP];
static int heap_size = 0;

static void swap(int *a, int *b) {
    int t = *a; *a = *b; *b = t;
}

static void sift_up(int i) {
    while (i > 0) {
        int parent = (i - 1) / 2;
        if (heap[parent] <= heap[i]) break;
        swap(&heap[parent], &heap[i]);
        i = parent;
    }
}

static void sift_down(int i) {
    for (;;) {
        int left = 2 * i + 1;
        int right = 2 * i + 2;
        int smallest = i;
        if (left < heap_size && heap[left] < heap[smallest]) smallest = left;
        if (right < heap_size && heap[right] < heap[smallest]) smallest = right;
        if (smallest == i) break;
        swap(&heap[i], &heap[smallest]);
        i = smallest;
    }
}

static void heap_push(int value) {
    heap[heap_size] = value;
    heap_size++;
    sift_up(heap_size - 1);
}

static int heap_pop(int *out) {
    if (heap_size == 0) return 0;
    *out = heap[0];
    heap_size--;
    heap[0] = heap[heap_size];
    sift_down(0);
    return 1;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    for (int i = 0; i < n; i++) {
        char op[4];
        if (scanf("%3s", op) != 1) return 0;
        if (op[0] == 'I') {
            int v;
            if (scanf("%d", &v) != 1) return 0;
            heap_push(v);
        } else {
            int out;
            if (heap_pop(&out)) {
                printf("%d\n", out);
            } else {
                printf("EMPTY\n");
            }
        }
    }

    printf("size=%d\n", heap_size);
    return 0;
}

/*
 * Grup 3 (cok-dosyali/cesitlilik) - s54_stack_module'un FIFO esi. Amac:
 * dairesel tampon (circular buffer) indeks aritmetigini coklu dosyada
 * (baslik + uygulama + kullanim) tutarli bicimde cevirip cevirmedigini
 * sinamak. NOT: `rear = (rear + 1) % QUEUE_CAPACITY` deseni Kok Neden G
 * (usize tasmasi) ile dolayli olarak ilgilidir (bkz. s77).
 */
#ifndef QUEUE_H
#define QUEUE_H

#define QUEUE_CAPACITY 8

typedef struct {
    int data[QUEUE_CAPACITY];
    int front;
    int rear;
    int count;
} Queue;

void queue_init(Queue *q);
int queue_is_empty(const Queue *q);
int queue_is_full(const Queue *q);
/* basarili enqueue -> 1, kapasite dolu -> 0 */
int queue_enqueue(Queue *q, int value);
/* basarili dequeue -> 1 ve *out doldurulur, bos -> 0 */
int queue_dequeue(Queue *q, int *out);
int queue_size(const Queue *q);

#endif

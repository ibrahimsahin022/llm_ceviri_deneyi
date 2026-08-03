#include "queue.h"

void queue_init(Queue *q) {
    q->front = 0;
    q->rear = 0;
    q->count = 0;
}

int queue_is_empty(const Queue *q) {
    return q->count == 0;
}

int queue_is_full(const Queue *q) {
    return q->count == QUEUE_CAPACITY;
}

int queue_enqueue(Queue *q, int value) {
    if (queue_is_full(q)) return 0;
    q->data[q->rear] = value;
    q->rear = (q->rear + 1) % QUEUE_CAPACITY;
    q->count++;
    return 1;
}

int queue_dequeue(Queue *q, int *out) {
    if (queue_is_empty(q)) return 0;
    *out = q->data[q->front];
    q->front = (q->front + 1) % QUEUE_CAPACITY;
    q->count--;
    return 1;
}

int queue_size(const Queue *q) {
    return q->count;
}

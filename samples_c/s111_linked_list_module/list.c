#include <stdlib.h>
#include "list.h"

void list_init(List *l) {
    l->head = NULL;
    l->size = 0;
}

void list_push_front(List *l, int value) {
    Node *n = malloc(sizeof(Node));
    n->value = value;
    n->next = l->head;
    l->head = n;
    l->size++;
}

int list_remove(List *l, int value) {
    Node *cur = l->head;
    Node *prev = NULL;
    while (cur != NULL) {
        if (cur->value == value) {
            if (prev == NULL) {
                l->head = cur->next;
            } else {
                prev->next = cur->next;
            }
            free(cur);
            l->size--;
            return 1;
        }
        prev = cur;
        cur = cur->next;
    }
    return 0;
}

int list_contains(const List *l, int value) {
    Node *cur = l->head;
    while (cur != NULL) {
        if (cur->value == value) return 1;
        cur = cur->next;
    }
    return 0;
}

int list_size(const List *l) {
    return l->size;
}

void list_free(List *l) {
    Node *cur = l->head;
    while (cur != NULL) {
        Node *next = cur->next;
        free(cur);
        cur = next;
    }
    l->head = NULL;
    l->size = 0;
}

#include <stdio.h>
#include <stdlib.h>

/* Girdi: ilk satir N, sonraki satirda N adet tam sayi.
   Islem: degerlerden bagli liste (linked list) kurar, listeyi ters cevirir,
   ters listeyi yazdirir, sonra toplam/min/max degerlerini yazdirir.
   Ornek: N=4, "3 1 4 1" -> "1 4 1 3" / "sum=9 min=1 max=4" */

struct Node {
    int value;
    struct Node *next;
};

static struct Node *append(struct Node *head, int v) {
    struct Node *n = (struct Node *)malloc(sizeof(struct Node));
    n->value = v;
    n->next = NULL;
    if (head == NULL) return n;
    struct Node *cur = head;
    while (cur->next != NULL) cur = cur->next;
    cur->next = n;
    return head;
}

static struct Node *reverse_list(struct Node *head) {
    struct Node *prev = NULL;
    struct Node *cur = head;
    while (cur != NULL) {
        struct Node *next = cur->next;
        cur->next = prev;
        prev = cur;
        cur = next;
    }
    return prev;
}

static void free_list(struct Node *head) {
    while (head != NULL) {
        struct Node *next = head->next;
        free(head);
        head = next;
    }
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    struct Node *head = NULL;
    for (int i = 0; i < n; i++) {
        int v;
        scanf("%d", &v);
        head = append(head, v);
    }

    head = reverse_list(head);

    long sum = 0;
    int have_min = 0;
    int mn = 0, mx = 0;
    struct Node *cur = head;
    int first = 1;
    while (cur != NULL) {
        if (!first) printf(" ");
        printf("%d", cur->value);
        first = 0;
        sum += cur->value;
        if (!have_min) {
            mn = cur->value;
            mx = cur->value;
            have_min = 1;
        } else {
            if (cur->value < mn) mn = cur->value;
            if (cur->value > mx) mx = cur->value;
        }
        cur = cur->next;
    }
    printf("\n");
    if (have_min) {
        printf("sum=%ld min=%d max=%d\n", sum, mn, mx);
    } else {
        printf("sum=0\n");
    }

    free_list(head);
    return 0;
}

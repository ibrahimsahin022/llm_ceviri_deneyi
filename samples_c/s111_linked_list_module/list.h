/*
 * Grup 3 (cok-dosyali/cesitlilik) - malloc/free tabanli tek yonlu bagli
 * liste modulu. Amac: C'nin manuel bellek yonetimini (Node* + malloc/free)
 * Rust'in sahiplik modeline (Option<Box<Node>> veya benzeri) coklu dosyada
 * dogru cevirip cevirmedigini sinamak - ozellikle "bir onceki dugumun
 * next isaretcisini guncelleme" (silme islemi) deseni.
 */
#ifndef LIST_H
#define LIST_H

typedef struct Node {
    int value;
    struct Node *next;
} Node;

typedef struct {
    Node *head;
    int size;
} List;

void list_init(List *l);
void list_push_front(List *l, int value);
/* value bulunup silinirse 1, bulunamazsa 0 */
int list_remove(List *l, int value);
/* value listede varsa 1, yoksa 0 */
int list_contains(const List *l, int value);
int list_size(const List *l);
void list_free(List *l);

#endif

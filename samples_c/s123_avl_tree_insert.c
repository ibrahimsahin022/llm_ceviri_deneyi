#include <stdio.h>
#include <stdlib.h>

/* Girdi: ilk satirda eklenecek deger sayisi n, ardindan n tam sayi
   (birbirinden FARKLI degerler).
   Cikti: tum degerler AVL agacina eklendikten sonra, INORDER (sirali)
   dolasim.
   NOT: s28_bst_traversal'dan DAHA ZOR - AVL, her ekleme sonrasi denge
   faktorunu kontrol edip gerektiginde ROTASYON (LL/RR/LR/RL) yapar.
   Bu, s28'in "duz" BST'sinden cok daha fazla isaretci-yeniden-baglama
   icerir - Rust'ta Box<Node> agaclarinda rotasyon yazmak, odunc
   denetleyicisiyle (borrow checker) ozellikle surtusmeli bir alandir
   (bkz. s111, s122 ile ayni aile). */

typedef struct Node {
    int value;
    int height;
    struct Node *left;
    struct Node *right;
} Node;

static int node_height(Node *n) {
    return n ? n->height : 0;
}

static int max_int(int a, int b) {
    return a > b ? a : b;
}

static Node *new_node(int value) {
    Node *n = malloc(sizeof(Node));
    n->value = value;
    n->height = 1;
    n->left = NULL;
    n->right = NULL;
    return n;
}

static Node *rotate_right(Node *y) {
    Node *x = y->left;
    Node *t2 = x->right;
    x->right = y;
    y->left = t2;
    y->height = max_int(node_height(y->left), node_height(y->right)) + 1;
    x->height = max_int(node_height(x->left), node_height(x->right)) + 1;
    return x;
}

static Node *rotate_left(Node *x) {
    Node *y = x->right;
    Node *t2 = y->left;
    y->left = x;
    x->right = t2;
    x->height = max_int(node_height(x->left), node_height(x->right)) + 1;
    y->height = max_int(node_height(y->left), node_height(y->right)) + 1;
    return y;
}

static int balance_factor(Node *n) {
    return n ? node_height(n->left) - node_height(n->right) : 0;
}

static Node *avl_insert(Node *node, int value) {
    if (node == NULL) return new_node(value);

    if (value < node->value) {
        node->left = avl_insert(node->left, value);
    } else if (value > node->value) {
        node->right = avl_insert(node->right, value);
    } else {
        return node;
    }

    node->height = 1 + max_int(node_height(node->left), node_height(node->right));
    int balance = balance_factor(node);

    if (balance > 1 && value < node->left->value) {
        return rotate_right(node);
    }
    if (balance < -1 && value > node->right->value) {
        return rotate_left(node);
    }
    if (balance > 1 && value > node->left->value) {
        node->left = rotate_left(node->left);
        return rotate_right(node);
    }
    if (balance < -1 && value < node->right->value) {
        node->right = rotate_right(node->right);
        return rotate_left(node);
    }

    return node;
}

static void inorder(Node *node) {
    if (node == NULL) return;
    inorder(node->left);
    printf("%d ", node->value);
    inorder(node->right);
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    Node *root = NULL;
    for (int i = 0; i < n; i++) {
        int v;
        if (scanf("%d", &v) != 1) return 0;
        root = avl_insert(root, v);
    }

    inorder(root);
    printf("\n");
    return 0;
}

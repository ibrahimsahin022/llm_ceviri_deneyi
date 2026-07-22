#include <stdio.h>
#include <stdlib.h>

/* Girdi: ilk satir N, sonraki satirda N adet tam sayi (tekrar olabilir).
   Degerlerden bir ikili arama agaci (BST) kurar; sirali (in-order) dolasimi,
   dugum sayisini ve agac yuksekligini yazdirir. Esit degerler sag alt agaca
   eklenir (kararli, tekrarlari sayar). */

struct Node {
    int value;
    struct Node *left;
    struct Node *right;
};

static struct Node *make_node(int v) {
    struct Node *n = (struct Node *)malloc(sizeof(struct Node));
    n->value = v;
    n->left = NULL;
    n->right = NULL;
    return n;
}

static struct Node *insert(struct Node *root, int v) {
    if (root == NULL) return make_node(v);
    if (v < root->value) {
        root->left = insert(root->left, v);
    } else {
        root->right = insert(root->right, v);
    }
    return root;
}

static int count_nodes(struct Node *root) {
    if (root == NULL) return 0;
    return 1 + count_nodes(root->left) + count_nodes(root->right);
}

static int height(struct Node *root) {
    if (root == NULL) return 0;
    int lh = height(root->left);
    int rh = height(root->right);
    return 1 + (lh > rh ? lh : rh);
}

static int first_print = 1;

static void inorder(struct Node *root) {
    if (root == NULL) return;
    inorder(root->left);
    if (!first_print) printf(" ");
    printf("%d", root->value);
    first_print = 0;
    inorder(root->right);
}

static void free_tree(struct Node *root) {
    if (root == NULL) return;
    free_tree(root->left);
    free_tree(root->right);
    free(root);
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    struct Node *root = NULL;
    for (int i = 0; i < n; i++) {
        int v;
        scanf("%d", &v);
        root = insert(root, v);
    }

    inorder(root);
    printf("\n");
    printf("count=%d height=%d\n", count_nodes(root), height(root));

    free_tree(root);
    return 0;
}

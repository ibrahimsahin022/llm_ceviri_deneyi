#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Girdi: ilk satirda islem sayisi n, ardindan n satir - "I <kelime>"
   (ekle) veya "S <kelime>" (ara). Kelimeler yalniz kucuk harf a-z.
   Cikti: S icin "YES"/"NO".
   Trie (onek agaci) - her dugum 26 cocuk isaretcisi + "kelime sonu"
   bayragi tutar. Rust'a cevrilirken klasik malloc/free tabanli agac
   yapisi Box/Option ile ifade edilmeli - s111 (bagli liste) ile ayni
   "sahiplik" zorlugunun agac versiyonu. */

#define ALPHABET 26

typedef struct TrieNode {
    struct TrieNode *children[ALPHABET];
    int is_end;
} TrieNode;

static TrieNode *trie_new_node(void) {
    TrieNode *n = calloc(1, sizeof(TrieNode));
    return n;
}

static void trie_insert(TrieNode *root, const char *word) {
    TrieNode *cur = root;
    for (const char *c = word; *c; c++) {
        int idx = *c - 'a';
        if (idx < 0 || idx >= ALPHABET) continue;
        if (cur->children[idx] == NULL) {
            cur->children[idx] = trie_new_node();
        }
        cur = cur->children[idx];
    }
    cur->is_end = 1;
}

static int trie_search(TrieNode *root, const char *word) {
    TrieNode *cur = root;
    for (const char *c = word; *c; c++) {
        int idx = *c - 'a';
        if (idx < 0 || idx >= ALPHABET) return 0;
        if (cur->children[idx] == NULL) return 0;
        cur = cur->children[idx];
    }
    return cur->is_end;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    TrieNode *root = trie_new_node();

    for (int i = 0; i < n; i++) {
        char op[4], word[64];
        if (scanf("%3s %63s", op, word) != 2) return 0;
        if (op[0] == 'I') {
            trie_insert(root, word);
        } else {
            printf("%s\n", trie_search(root, word) ? "YES" : "NO");
        }
    }

    return 0;
}

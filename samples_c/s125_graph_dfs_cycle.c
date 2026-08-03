#include <stdio.h>

/* Girdi: ilk satirda v (dugum sayisi) e (kenar sayisi), ardindan e satir
   "a b" (YONLU kenar a->b).
   Cikti: yonlu grafta CEVRIM varsa "CYCLE", yoksa "NOCYCLE".
   Klasik 3-renkli (beyaz/gri/siyah) DFS cevrim tespiti - "gri" (yigin
   uzerinde, henuz bitmemis) bir dugume tekrar rastlamak cevrim demektir. */

#define MAX_V 100
#define MAX_DEG 20

static int adj[MAX_V][MAX_DEG];
static int adj_count[MAX_V];
static int color[MAX_V]; /* 0=beyaz, 1=gri, 2=siyah */
static int has_cycle = 0;

static void dfs(int u) {
    color[u] = 1;
    for (int i = 0; i < adj_count[u]; i++) {
        int v = adj[u][i];
        if (color[v] == 1) {
            has_cycle = 1;
            return;
        }
        if (color[v] == 0) {
            dfs(v);
            if (has_cycle) return;
        }
    }
    color[u] = 2;
}

int main(void) {
    int v, e;
    if (scanf("%d %d", &v, &e) != 2) return 0;

    for (int i = 0; i < e; i++) {
        int a, b;
        if (scanf("%d %d", &a, &b) != 2) return 0;
        adj[a][adj_count[a]++] = b;
    }

    for (int i = 0; i < v && !has_cycle; i++) {
        if (color[i] == 0) {
            dfs(i);
        }
    }

    printf("%s\n", has_cycle ? "CYCLE" : "NOCYCLE");
    return 0;
}

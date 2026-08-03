#include <stdio.h>

/* Girdi: ilk satirda v (dugum sayisi) e (kenar sayisi), ardindan e satir
   "a b" (yonsuz kenar a-b), sonra baslangic dugumu s.
   Cikti: BFS ziyaret sirasi (dugum numaralari, bosluk ile ayrilmis).
   Komsuluk listesi (adjacency list) sabit boyutlu dizilerle temsil edilir
   (basitlik icin dugum basina en fazla MAX_DEG komsu). */

#define MAX_V 100
#define MAX_DEG 20

static int adj[MAX_V][MAX_DEG];
static int adj_count[MAX_V];
static int visited[MAX_V];
static int queue[MAX_V];

static void add_edge(int a, int b) {
    adj[a][adj_count[a]++] = b;
    adj[b][adj_count[b]++] = a;
}

int main(void) {
    int v, e;
    if (scanf("%d %d", &v, &e) != 2) return 0;

    for (int i = 0; i < e; i++) {
        int a, b;
        if (scanf("%d %d", &a, &b) != 2) return 0;
        add_edge(a, b);
    }

    int start;
    if (scanf("%d", &start) != 1) return 0;

    int qhead = 0, qtail = 0;
    queue[qtail++] = start;
    visited[start] = 1;

    int first = 1;
    while (qhead < qtail) {
        int cur = queue[qhead++];
        if (!first) printf(" ");
        printf("%d", cur);
        first = 0;
        for (int i = 0; i < adj_count[cur]; i++) {
            int next = adj[cur][i];
            if (!visited[next]) {
                visited[next] = 1;
                queue[qtail++] = next;
            }
        }
    }
    printf("\n");
    (void)v;
    return 0;
}

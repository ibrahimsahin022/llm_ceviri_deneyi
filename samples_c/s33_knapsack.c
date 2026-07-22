#include <stdio.h>
#include <stdlib.h>

/* KAYNAK: Rosetta Code, "Knapsack problem/0-1", C cozumu.
   https://rosettacode.org/wiki/Knapsack_problem/0-1
   (GFDL 1.2 / CC-BY-SA lisansli, atifla yeniden kullanima acik).
   item_t yapisi ve knapsack() (dinamik programlama) fonksiyonu KAYNAKTAN
   DEGISTIRILMEDEN alinmistir (yalnizca item_t'den kullanilmayan "name" alani
   cikarilmistir); main() bu deney ortaminin stdin/stdout sozlesmesine uyacak
   sekilde YENIDEN YAZILMISTIR (orijinalde sabit kodlanmis 22 esyalik bir
   liste ve 400 birimlik sabit kapasite vardi). */

typedef struct {
    int weight;
    int value;
} item_t;

int *knapsack (item_t *items, int n, int w) {
    int i, j, a, b, *mm, **m, *s;
    mm = calloc((n + 1) * (w + 1), sizeof (int));
    m = malloc((n + 1) * sizeof (int *));
    m[0] = mm;
    for (i = 1; i <= n; i++) {
        m[i] = &mm[i * (w + 1)];
        for (j = 0; j <= w; j++) {
            if (items[i - 1].weight > j) {
                m[i][j] = m[i - 1][j];
            }
            else {
                a = m[i - 1][j];
                b = m[i - 1][j - items[i - 1].weight] + items[i - 1].value;
                m[i][j] = a > b ? a : b;
            }
        }
    }
    s = calloc(n, sizeof (int));
    for (i = n, j = w; i > 0; i--) {
        if (m[i][j] > m[i - 1][j]) {
            s[i - 1] = 1;
            j -= items[i - 1].weight;
        }
    }
    free(mm);
    free(m);
    return s;
}

int main(void) {
    int n, cap;
    if (scanf("%d %d", &n, &cap) != 2) return 0;

    item_t *items = malloc(n * sizeof(item_t));
    for (int i = 0; i < n; i++) {
        scanf("%d %d", &items[i].weight, &items[i].value);
    }

    int *s = knapsack(items, n, cap);

    int tw = 0, tv = 0;
    for (int i = 0; i < n; i++) {
        if (s[i]) {
            tw += items[i].weight;
            tv += items[i].value;
        }
    }
    printf("weight=%d value=%d\n", tw, tv);

    free(s);
    free(items);
    return 0;
}

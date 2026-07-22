#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* KAYNAK: Rosetta Code, "Longest common subsequence", C cozumu.
   https://rosettacode.org/wiki/Longest_common_subsequence
   (GFDL 1.2 / CC-BY-SA lisansli, atifla yeniden kullanima acik).
   lcs() (dinamik programlama) fonksiyonu KAYNAKTAN DEGISTIRILMEDEN
   alinmistir; main() bu deney ortaminin stdin/stdout sozlesmesine uyacak
   sekilde EKLENMISTIR (orijinal kaynakta main() yoktu, yalnizca lcs()
   fonksiyonu tanimliydi). */

#define MAX(a, b) (a > b ? a : b)

int lcs (char *a, int n, char *b, int m, char **s) {
    int i, j, k, t;
    int *z = calloc((n + 1) * (m + 1), sizeof (int));
    int **c = calloc((n + 1), sizeof (int *));
    for (i = 0; i <= n; i++) {
        c[i] = &z[i * (m + 1)];
    }
    for (i = 1; i <= n; i++) {
        for (j = 1; j <= m; j++) {
            if (a[i - 1] == b[j - 1]) {
                c[i][j] = c[i - 1][j - 1] + 1;
            }
            else {
                c[i][j] = MAX(c[i - 1][j], c[i][j - 1]);
            }
        }
    }
    t = c[n][m];
    *s = malloc(t + 1);
    (*s)[t] = '\0';
    for (i = n, j = m, k = t - 1; k >= 0;) {
        if (a[i - 1] == b[j - 1])
            (*s)[k] = a[i - 1], i--, j--, k--;
        else if (c[i][j - 1] > c[i - 1][j])
            j--;
        else
            i--;
    }
    free(c);
    free(z);
    return t;
}

static void trim_newline(char *s) {
    size_t len = strlen(s);
    while (len > 0 && (s[len-1] == '\n' || s[len-1] == '\r')) {
        s[--len] = '\0';
    }
}

int main(void) {
    char a[256], b[256];
    if (fgets(a, sizeof(a), stdin) == NULL) return 0;
    if (fgets(b, sizeof(b), stdin) == NULL) return 0;
    trim_newline(a);
    trim_newline(b);

    char *s = NULL;
    int len = lcs(a, (int)strlen(a), b, (int)strlen(b), &s);
    printf("len=%d lcs=%s\n", len, s);
    free(s);
    return 0;
}

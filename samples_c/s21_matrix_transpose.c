#include <stdio.h>

/* Girdi: ilk satirda r c, ardindan r*c tam sayi (matris, satir satir).
   Cikti: matrisin transpozu (c x r), satirlar bosluklarla ayrilmis. */
int main(void) {
    int r, c;
    if (scanf("%d %d", &r, &c) != 2) return 0;
    static long long m[64][64];
    for (int i = 0; i < r; i++)
        for (int j = 0; j < c; j++)
            scanf("%lld", &m[i][j]);
    for (int j = 0; j < c; j++) {
        for (int i = 0; i < r; i++) {
            printf("%lld", m[i][j]);
            if (i < r - 1) printf(" ");
        }
        printf("\n");
    }
    return 0;
}

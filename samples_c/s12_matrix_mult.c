#include <stdio.h>

/* Girdi:
     ilk satir: r1 c1  (birinci matris boyutu)
     sonra r1*c1 tam sayi (birinci matris, satir satir)
     sonra bir satir: r2 c2  (ikinci matris boyutu)
     sonra r2*c2 tam sayi (ikinci matris)
   Cikti: carpim matrisi (r1 x c2), her satir bosluklarla, satirlar yeni satirla.
   Not: c1 == r2 varsayilir. */
int main(void) {
    int r1, c1, r2, c2;
    if (scanf("%d %d", &r1, &c1) != 2) return 0;
    static long long A[64][64], B[64][64], C[64][64];
    for (int i = 0; i < r1; i++)
        for (int j = 0; j < c1; j++)
            scanf("%lld", &A[i][j]);
    if (scanf("%d %d", &r2, &c2) != 2) return 0;
    for (int i = 0; i < r2; i++)
        for (int j = 0; j < c2; j++)
            scanf("%lld", &B[i][j]);
    for (int i = 0; i < r1; i++) {
        for (int j = 0; j < c2; j++) {
            long long s = 0;
            for (int k = 0; k < c1; k++) s += A[i][k] * B[k][j];
            C[i][j] = s;
        }
    }
    for (int i = 0; i < r1; i++) {
        for (int j = 0; j < c2; j++) {
            printf("%lld", C[i][j]);
            if (j < c2 - 1) printf(" ");
        }
        printf("\n");
    }
    return 0;
}

#include <stdio.h>

/* Girdi: ilk satirda n (1..8), ardindan n*n tam sayi (matris, satir satir).
   Cikti: matrisin determinanti (Laplace acilimi ile). */
static long long det(long long m[8][8], int n) {
    if (n == 1) return m[0][0];
    if (n == 2) return m[0][0] * m[1][1] - m[0][1] * m[1][0];
    long long result = 0;
    long long sub[8][8];
    for (int col = 0; col < n; col++) {
        /* minor matrisini olustur */
        for (int i = 1; i < n; i++) {
            int subcol = 0;
            for (int j = 0; j < n; j++) {
                if (j == col) continue;
                sub[i-1][subcol] = m[i][j];
                subcol++;
            }
        }
        long long sign = (col % 2 == 0) ? 1 : -1;
        result += sign * m[0][col] * det(sub, n - 1);
    }
    return result;
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    long long m[8][8];
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            scanf("%lld", &m[i][j]);
    printf("%lld\n", det(m, n));
    return 0;
}

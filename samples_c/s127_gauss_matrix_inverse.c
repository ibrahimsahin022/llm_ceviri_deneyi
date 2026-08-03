#include <stdio.h>
#include <math.h>

/* Girdi: n (matris boyutu, n x n), ardindan n*n kayan nokta sayisi (satir
   satir).
   Cikti: matrisin tersi (n satir, her satirda n sayi, %.4f ile), veya
   matris tekil ise (pivot ~0) "SINGULAR".
   Gauss-Jordan eleme ile [A|I] -> [I|A^-1]. NOT: kismi pivotlama (partial
   pivoting) YOK - basitlik icin dogrudan pivot satirina bolme yapiliyor;
   bu, sayisal kararlilik acisindan "naif" ama KUCUK test matrisleri icin
   yeterlidir ve deterministiktir. */

#define MAX_N 10

int main(void) {
    int n;
    if (scanf("%d", &n) != 1 || n <= 0 || n > MAX_N) return 0;

    double a[MAX_N][2 * MAX_N];
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            if (scanf("%lf", &a[i][j]) != 1) return 0;
        }
        for (int j = 0; j < n; j++) {
            a[i][n + j] = (i == j) ? 1.0 : 0.0;
        }
    }

    for (int col = 0; col < n; col++) {
        double pivot = a[col][col];
        if (fabs(pivot) < 1e-9) {
            printf("SINGULAR\n");
            return 0;
        }
        for (int j = 0; j < 2 * n; j++) {
            a[col][j] /= pivot;
        }
        for (int row = 0; row < n; row++) {
            if (row == col) continue;
            double factor = a[row][col];
            for (int j = 0; j < 2 * n; j++) {
                a[row][j] -= factor * a[col][j];
            }
        }
    }

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            printf("%.4f ", a[i][n + j]);
        }
        printf("\n");
    }
    return 0;
}

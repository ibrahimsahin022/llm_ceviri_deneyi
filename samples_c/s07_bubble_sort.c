#include <stdio.h>

/* Girdi: ilk satirda n, ardindan n adet tam sayi.
   Cikti: kucukten buyuge siralanmis sayilar (bosluk ile ayrilmis). */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    int a[1000];
    if (n > 1000) n = 1000;
    for (int i = 0; i < n; i++) scanf("%d", &a[i]);
    for (int i = 0; i < n - 1; i++) {
        for (int j = 0; j < n - 1 - i; j++) {
            if (a[j] > a[j+1]) {
                int t = a[j];
                a[j] = a[j+1];
                a[j+1] = t;
            }
        }
    }
    for (int i = 0; i < n; i++) {
        printf("%d", a[i]);
        if (i < n - 1) printf(" ");
    }
    printf("\n");
    return 0;
}

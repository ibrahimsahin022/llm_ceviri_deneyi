#include <stdio.h>

/* Girdi: ilk satirda n, sonra artan sirali n tam sayi, son satirda aranan hedef.
   Cikti: hedefin indeksi (0 tabanli) ya da bulunamazsa -1. */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;
    int a[1000];
    if (n > 1000) n = 1000;
    for (int i = 0; i < n; i++) scanf("%d", &a[i]);
    int target;
    if (scanf("%d", &target) != 1) return 0;
    int lo = 0, hi = n - 1, ans = -1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (a[mid] == target) { ans = mid; break; }
        else if (a[mid] < target) lo = mid + 1;
        else hi = mid - 1;
    }
    printf("%d\n", ans);
    return 0;
}

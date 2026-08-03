#include <stdio.h>

/* Girdi: not (0-100 arasi tam sayi).
   Cikti: notun kac "basari rozeti" kazandigi (dusmeli switch, esik
   tabanli, kumulatif; ayrica coklu-etiketli case: 10 ve 9 ayni govdeyi
   paylasir).
   NOT: Kok Neden H (switch dusmesi) icin 5. bagimsiz ornek (s53'un
   vergi yerine rozet sayan esi). */
int main(void) {
    int grade;
    if (scanf("%d", &grade) != 1) return 0;
    int tier = grade / 10; /* 0..10 */
    int badges = 0;
    switch (tier) {
        case 10:
        case 9: badges++; /* fallthrough (A rozeti) */
        case 8: badges++; /* fallthrough (B rozeti) */
        case 7: badges++; /* fallthrough (C rozeti) */
        case 6: badges++; /* D rozeti */
                break;
        default: badges = 0;
    }
    printf("%d\n", badges);
    return 0;
}

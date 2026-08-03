#include <stdio.h>

/* Girdi: hedef durum (0=IDLE,1=CONNECTING,2=HANDSHAKE,3=READY).
   Cikti: o duruma ulasmak icin gerekli kumulatif adim sayisi (dusmeli
   switch ile her asamadan gecerken +1).
   NOT: Kok Neden H (switch dusmesi) icin 4. bagimsiz ornek. Durum
   makinesi gecisleri, gercek protokol/baglanti kodlarinda yaygin bir
   "her asamadan sirayla gec" desenidir - dusme kasitlidir. */
int main(void) {
    int target;
    if (scanf("%d", &target) != 1) return 0;
    int steps = 0;
    switch (target) {
        case 3: steps++; /* READY */ /* fallthrough */
        case 2: steps++; /* HANDSHAKE */ /* fallthrough */
        case 1: steps++; /* CONNECTING */ /* fallthrough */
        case 0: steps++; /* IDLE */
                break;
        default: steps = -1;
    }
    printf("%d\n", steps);
    return 0;
}

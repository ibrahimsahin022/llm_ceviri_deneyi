#include <stdio.h>
#include <string.h>

/* Girdi: her satirda "<a> <b>" (tam sayi bolme).
   Cikti: basariliysa bolum, b=0 ise "HATA: <mesaj>".
   NOT: Kok Neden E (guvensiz global durum) icin 4. bagimsiz ornek. Global
   "son hata" tamponu (g_errbuf) - errno/strerror() ailesinin klasik C
   deseni - her cagriyla degisen paylasilan degistirilebilir durumdur. */
static char g_errbuf[256] = "";

void set_error(const char *msg) {
    strncpy(g_errbuf, msg, sizeof(g_errbuf) - 1);
    g_errbuf[sizeof(g_errbuf) - 1] = '\0';
}

const char *get_error(void) {
    return g_errbuf;
}

int safe_divide(int a, int b, int *out) {
    if (b == 0) {
        set_error("division by zero");
        return -1;
    }
    *out = a / b;
    return 0;
}

int main(void) {
    int a, b, r;
    while (scanf("%d %d", &a, &b) == 2) {
        if (safe_divide(a, b, &r) == 0) {
            printf("%d\n", r);
        } else {
            printf("HATA: %s\n", get_error());
        }
    }
    return 0;
}

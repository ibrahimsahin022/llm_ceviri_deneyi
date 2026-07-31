#include <stdio.h>

/* Girdi: tek sayi (0-3 arasi log seviyesi).
   Cikti: seviyeye gore filtrelenmis log satirlari.
   NOT: Kok Neden E (guvensiz global durum) icin 5. bagimsiz ornek. Global
   log seviyesi (g_log_level), set_log_level() ile degistirilir ve
   log_msg() tarafindan okunur - iki fonksiyon arasinda paylasilan
   degistirilebilir durum. */
static int g_log_level = 1; /* 0=DEBUG,1=INFO,2=WARN,3=ERROR */

void set_log_level(int lvl) {
    g_log_level = lvl;
}

void log_msg(int lvl, const char *msg) {
    static const char *names[] = {"DEBUG", "INFO", "WARN", "ERROR"};
    if (lvl >= g_log_level) {
        printf("[%s] %s\n", names[lvl], msg);
    }
}

int main(void) {
    int lvl;
    if (scanf("%d", &lvl) != 1) return 0;
    set_log_level(lvl);
    log_msg(0, "baslangic");
    log_msg(1, "bilgi mesaji");
    log_msg(2, "uyari mesaji");
    log_msg(3, "hata mesaji");
    return 0;
}

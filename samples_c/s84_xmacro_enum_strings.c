#include <stdio.h>
#include <string.h>

/* Girdi: bir renk adi (string).
   Cikti: rengin X-Macro'dan uretilen enum indeksi (bulunamazsa -1).
   NOT: Kok Neden I (makro coklu-degerlendirme / X-Macro) icin 4. bagimsiz
   ornek (s56'dan farkli alan: komut listesi yerine renk listesi - ayni
   "tek dogru kaynak" X-Macro deseni). COLOR_LIST tek bir yerde tanimlanir;
   hem enum'u hem isim dizisini ayni listeden turetir. LLM'in Rust'a
   cevirirken bu tek-kaynak ilkesini koruyup korumadigi (bir enum + turetilmis
   bir dizi/match) ilginc bir sorudur. */
#define COLOR_LIST \
    X(RED, "red") \
    X(GREEN, "green") \
    X(BLUE, "blue") \
    X(YELLOW, "yellow") \
    X(BLACK, "black")

typedef enum {
#define X(name, str) COLOR_##name,
    COLOR_LIST
#undef X
    COLOR_COUNT
} Color;

static const char *color_names[COLOR_COUNT] = {
#define X(name, str) str,
    COLOR_LIST
#undef X
};

int main(void) {
    char buf[32];
    if (scanf("%31s", buf) != 1) return 0;
    int idx = -1;
    for (int i = 0; i < COLOR_COUNT; i++) {
        if (strcmp(color_names[i], buf) == 0) {
            idx = i;
            break;
        }
    }
    printf("%d\n", idx);
    return 0;
}

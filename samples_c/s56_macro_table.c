/*
 * Faz 3 devami - karmasik makro genisletmesi.
 *
 * Iki ayri C onislemci (preprocessor) deseni test edilir:
 *
 * 1) X-Macro deseni: COMMAND_LIST tek bir yerde tanimlanir, hem enum'u
 *    (token-pasting CMD_##name ile) hem karsilik gelen isim dizisini
 *    ayni listeden URETIR. Gercek C projelerinde (durum makineleri,
 *    protokol ayristiricilari, hata kodu tablolari) çok yaygin bir
 *    "tek dogru kaynak" (single source of truth) tekniğidir. LLM'in
 *    bunu Rust'a cevirirken ayni tek-kaynak ilkesini (bir enum + bir
 *    match/dizi, ikisi birbirinden turetilmis) koruyup korumadigi,
 *    yoksa iki listeyi birbirinden bagimsiz, senkronizasyonu bozulabilir
 *    sekilde mi yazdigi ilginc bir soru.
 *
 * 2) COKLU-DEGERLENDIRME (multiple evaluation) tuzagi: klasik "MAX makro"
 *    hatasi. C makrolari metinsel ikame (textual substitution) oldugundan,
 *    bir parametre govde icinde BIRDEN FAZLA kez geciyorsa, o parametre
 *    icin verilen ifade DE o kadar kez calisir. Yan etkili bir ifade
 *    (x++) MAX(x++, 10) icinde kullanilirsa, x, tek bir "fonksiyon
 *    cagrisi" gibi gorunen bu ifadede aslinda IKI KEZ artar (once
 *    karsilastirmada, sonra - eger secilirse - sonuc dalinda). Bu, C'de
 *    dogal bir Rust karsiligi OLMAYAN, saf metin-ikame anlaminin bir
 *    sonucudur: Rust'ta bir fonksiyon (veya generic) olarak cevrilen
 *    ayni "max" mantigi argumanlarini HER ZAMAN TAM OLARAK BIR KEZ
 *    degerlendirir - bu yuzden LLM'in dogal cevirisi (x++ 'i bir kez
 *    hesaplayip sonucu MAX'e gecirmek) x'in son degerini VE max
 *    sonucunu C'den FARKLI URETIR.
 */
#include <stdio.h>
#include <string.h>

#define COMMAND_LIST \
    X(ADD, "add") \
    X(SUB, "sub") \
    X(MUL, "mul") \
    X(DIV, "div")

typedef enum {
#define X(name, str) CMD_##name,
    COMMAND_LIST
#undef X
    CMD_COUNT
} Command;

static const char *command_names[CMD_COUNT] = {
#define X(name, str) str,
    COMMAND_LIST
#undef X
};

static int find_command(const char *name)
{
    for (int i = 0; i < CMD_COUNT; i++) {
        if (strcmp(command_names[i], name) == 0) {
            return i;
        }
    }
    return -1;
}

static int apply_command(Command cmd, int a, int b)
{
    switch (cmd) {
        case CMD_ADD: return a + b;
        case CMD_SUB: return a - b;
        case CMD_MUL: return a * b;
        case CMD_DIV: return b != 0 ? a / b : 0;
        default: return 0;
    }
}

#define MAX(a, b) ((a) > (b) ? (a) : (b))

int main(void)
{
    char cmd_name[16];
    int a, b;
    if (scanf("%15s %d %d", cmd_name, &a, &b) != 3) return 1;

    int idx = find_command(cmd_name);
    if (idx < 0) {
        printf("UNKNOWN\n");
        return 0;
    }
    int result = apply_command((Command)idx, a, b);
    printf("%s(%d,%d)=%d\n", command_names[idx], a, b, result);

    /* Coklu-degerlendirme tuzagi: x, MAX govdesinde iki kez gecen 'a'
     * parametresi icinde yan etkili bir ifade (x++) oldugu icin,
     * kosula bagli olarak bir veya iki kez artabilir. */
    int x = a;
    int m = MAX(x++, 10);
    printf("m=%d x=%d\n", m, x);

    return 0;
}

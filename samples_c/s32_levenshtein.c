#include <stdio.h>
#include <string.h>

/* KAYNAK: Rosetta Code, "Levenshtein distance", C cozumu.
   https://rosettacode.org/wiki/Levenshtein_distance
   (GFDL 1.2 / CC-BY-SA lisansli, atifla yeniden kullanima acik).
   levenshtein() fonksiyonu KAYNAKTAN DEGISTIRILMEDEN alinmistir (ozyinelemeli,
   memoizasyonsuz hali); yalnizca main() bu deney ortaminin stdin/stdout
   sozlesmesine uyacak sekilde YENIDEN YAZILMISTIR (orijinalde sabit kodlanmis
   iki sabit dize vardi). NOT: Ozyinelemeli/memoizasyonsuz oldugundan girdi
   uzunlugu kisa tutulmalidir (buyuk girdilerde katlanarak yavaslar). */

int levenshtein(const char *s, int ls, const char *t, int lt)
{
        int a, b, c;

        if (!ls) return lt;
        if (!lt) return ls;

        if (s[ls - 1] == t[lt - 1])
                return levenshtein(s, ls - 1, t, lt - 1);

        a = levenshtein(s, ls - 1, t, lt - 1);
        b = levenshtein(s, ls,     t, lt - 1);
        c = levenshtein(s, ls - 1, t, lt    );

        if (a > b) a = b;
        if (a > c) a = c;

        return a + 1;
}

static void trim_newline(char *s) {
    size_t len = strlen(s);
    while (len > 0 && (s[len-1] == '\n' || s[len-1] == '\r')) {
        s[--len] = '\0';
    }
}

int main(void)
{
    char s1[64], s2[64];
    if (fgets(s1, sizeof(s1), stdin) == NULL) return 0;
    if (fgets(s2, sizeof(s2), stdin) == NULL) return 0;
    trim_newline(s1);
    trim_newline(s2);

    printf("%d\n", levenshtein(s1, (int)strlen(s1), s2, (int)strlen(s2)));
    return 0;
}

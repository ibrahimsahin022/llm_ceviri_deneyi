#include <stdio.h>
#include <string.h>
#include <ctype.h>

/* Kaynak: Redis src/util.c, stringmatchlen_impl()/stringmatchlen() (KEYS
   komutunun glob eslestiricisi) - birebir.
   https://raw.githubusercontent.com/redis/redis/unstable/src/util.c
   Lisans: BSD-3-Clause.
   Girdi: iki satir - once desen, sonra metin (buyuk/kucuk harf DUYARLI,
   nocase=0).
   Cikti: "MATCH" veya "NOMATCH". */

static int stringmatchlen_impl(const char *pattern, int patternLen,
        const char *string, int stringLen, int nocase, int *skipLongerMatches, int nesting)
{
    if (nesting > 1000) return 0;

    while(patternLen && stringLen) {
        switch(pattern[0]) {
        case '*':
            while (patternLen && pattern[1] == '*') {
                pattern++;
                patternLen--;
            }
            if (patternLen == 1)
                return 1;
            while(stringLen) {
                if (stringmatchlen_impl(pattern+1, patternLen-1,
                            string, stringLen, nocase, skipLongerMatches, nesting+1))
                    return 1;
                if (*skipLongerMatches)
                    return 0;
                string++;
                stringLen--;
            }
            *skipLongerMatches = 1;
            return 0;
            break;
        case '?':
            string++;
            stringLen--;
            break;
        case '[':
        {
            int not, match;
            pattern++;
            patternLen--;
            not = patternLen && pattern[0] == '^';
            if (not) {
                pattern++;
                patternLen--;
            }
            match = 0;
            while(1) {
                if (patternLen >= 2 && pattern[0] == '\\') {
                    pattern++;
                    patternLen--;
                    if (pattern[0] == string[0])
                        match = 1;
                } else if (patternLen == 0) {
                    pattern--;
                    patternLen++;
                    break;
                } else if (pattern[0] == ']') {
                    break;
                } else if (patternLen >= 3 && pattern[1] == '-') {
                    int start = pattern[0];
                    int end = pattern[2];
                    int c = string[0];
                    if (start > end) {
                        int t = start;
                        start = end;
                        end = t;
                    }
                    if (nocase) {
                        start = tolower(start);
                        end = tolower(end);
                        c = tolower(c);
                    }
                    pattern += 2;
                    patternLen -= 2;
                    if (c >= start && c <= end)
                        match = 1;
                } else {
                    if (!nocase) {
                        if (pattern[0] == string[0])
                            match = 1;
                    } else {
                        if (tolower((int)pattern[0]) == tolower((int)string[0]))
                            match = 1;
                    }
                }
                pattern++;
                patternLen--;
            }
            if (not)
                match = !match;
            if (!match)
                return 0;
            string++;
            stringLen--;
            break;
        }
        case '\\':
            if (patternLen >= 2) {
                pattern++;
                patternLen--;
            }
            /* fall through */
        default:
            if (!nocase) {
                if (pattern[0] != string[0])
                    return 0;
            } else {
                if (tolower((int)pattern[0]) != tolower((int)string[0]))
                    return 0;
            }
            string++;
            stringLen--;
            break;
        }
        pattern++;
        patternLen--;
        if (stringLen == 0) {
            while(patternLen && *pattern == '*') {
                pattern++;
                patternLen--;
            }
            break;
        }
    }
    if (patternLen == 0 && stringLen == 0)
        return 1;
    return 0;
}

int stringmatchlen(const char *pattern, int patternLen,
        const char *string, int stringLen, int nocase) {
    int skipLongerMatches = 0;
    return stringmatchlen_impl(pattern,patternLen,string,stringLen,nocase,&skipLongerMatches,0);
}

int main(void) {
    char pattern[256], text[256];
    if (!fgets(pattern, sizeof(pattern), stdin)) return 0;
    if (!fgets(text, sizeof(text), stdin)) return 0;
    pattern[strcspn(pattern, "\r\n")] = 0;
    text[strcspn(text, "\r\n")] = 0;
    int r = stringmatchlen(pattern, (int)strlen(pattern), text, (int)strlen(text), 0);
    printf("%s\n", r ? "MATCH" : "NOMATCH");
    return 0;
}

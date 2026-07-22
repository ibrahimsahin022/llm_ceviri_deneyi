/*
 * Ozgun program (bu calisma icin yazilmistir).
 * Hedef: C bit-alanlari (bit-fields). Unsigned bit-alanina deger atandiginda
 * deger alanin genisligine gore KIRPILIR (mod 2^genislik) - C standardinca
 * tanimli bir davranistir. Rust'ta yerlesik bit-alani yoktur; LLM bunu elle
 * maskeleme ile taklit etmek zorundadir.
 */
#include <stdio.h>

struct Flags {
    unsigned int a : 1;
    unsigned int b : 3;
    unsigned int c : 4;
};

int main(void) {
    unsigned int raw;
    if (scanf("%u", &raw) != 1) return 0;
    struct Flags f;
    f.a = raw;
    f.b = raw;
    f.c = raw;
    printf("%u %u %u\n", f.a, f.b, f.c);
    return 0;
}

/*
 * Ozgun program (bu calisma icin yazilmistir).
 * Hedef: C union'i ile IEEE-754 float bit-duzeyinde yeniden yorumlama
 * (type punning). Rust'ta union yerine dogrudan f32::to_bits() gibi guvenli
 * bir karsilik var; bu ornek LLM'in bu deyimi bilip bilmedigini test eder.
 */
#include <stdio.h>

union FloatBits {
    float f;
    unsigned int u;
};

int main(void) {
    float x;
    if (scanf("%f", &x) != 1) return 0;
    union FloatBits fb;
    fb.f = x;
    printf("%u\n", fb.u);
    return 0;
}

#include <stdio.h>
#include <string.h>

/* Girdi: ilk satirda komut sayisi n, ardindan n satir - "<komut> <arg>"
   (komut: SET/ADD/SUB/MUL, arg tam sayi) veya "PRINT" (arg yok).
   Cikti: her PRINT icin o anki durum (accumulator) degeri.
   NOT: s120'den FARKLI olarak burada dispatch tablosundaki her fonksiyon
   PAYLASILAN BIR DURUMU (int* state) DEGISTIRIR (yan etkili), salt
   girdi->cikti degil - bu, basit bir "komut yorumlayicisi" (VM benzeri)
   deseni ve fonksiyon isaretcisinin EK BIR PARAMETRE (durum isaretcisi)
   ile birlikte tasinmasi gerektigini test eder. */

typedef void (*CmdFn)(int *state, int arg);

static void cmd_set(int *state, int arg) { *state = arg; }
static void cmd_add(int *state, int arg) { *state += arg; }
static void cmd_sub(int *state, int arg) { *state -= arg; }
static void cmd_mul(int *state, int arg) { *state *= arg; }

typedef struct {
    const char *name;
    CmdFn fn;
} CmdEntry;

static CmdEntry table[] = {
    { "SET", cmd_set },
    { "ADD", cmd_add },
    { "SUB", cmd_sub },
    { "MUL", cmd_mul },
};

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    int state = 0;

    for (int i = 0; i < n; i++) {
        char cmd[8];
        if (scanf("%7s", cmd) != 1) return 0;

        if (strcmp(cmd, "PRINT") == 0) {
            printf("%d\n", state);
            continue;
        }

        int arg;
        if (scanf("%d", &arg) != 1) return 0;

        for (size_t k = 0; k < sizeof(table) / sizeof(table[0]); k++) {
            if (strcmp(cmd, table[k].name) == 0) {
                table[k].fn(&state, arg);
                break;
            }
        }
    }

    return 0;
}

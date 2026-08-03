#include <stdio.h>
#include <string.h>

/* Girdi: ilk satirda islem sayisi n, ardindan n satir - "<op> <a> <b>"
   (op: add/sub/mul/div, a,b tam sayi).
   Cikti: her islem icin sonuc (div icin b==0 ise "DIVZERO").
   NOT: Fonksiyon isaretcisi dizisi ({isim, fn} eslesmesiyle "dagitim
   tablosu") klasik bir C deseni - Rust'ta `fn(i32,i32)->i32` isaretcileri
   veya kapatmalar (closures) ile dogal olarak eslenir; ilginc soru LLM'in
   HashMap<&str, fn(...)> mi yoksa daha az idiomatik bir if/else zinciri
   mi sececegidir (ikisi de dogru olabilir, ama "dagitim tablosu" ruhu
   kaybolabilir). */

typedef int (*BinOp)(int, int);

static int op_add(int a, int b) { return a + b; }
static int op_sub(int a, int b) { return a - b; }
static int op_mul(int a, int b) { return a * b; }

typedef struct {
    const char *name;
    BinOp fn;
} OpEntry;

static OpEntry dispatch_table[] = {
    { "add", op_add },
    { "sub", op_sub },
    { "mul", op_mul },
};

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    for (int i = 0; i < n; i++) {
        char op[8];
        int a, b;
        if (scanf("%7s %d %d", op, &a, &b) != 3) return 0;

        if (strcmp(op, "div") == 0) {
            if (b == 0) {
                printf("DIVZERO\n");
            } else {
                printf("%d\n", a / b);
            }
            continue;
        }

        int found = 0;
        for (size_t k = 0; k < sizeof(dispatch_table) / sizeof(dispatch_table[0]); k++) {
            if (strcmp(op, dispatch_table[k].name) == 0) {
                printf("%d\n", dispatch_table[k].fn(a, b));
                found = 1;
                break;
            }
        }
        if (!found) {
            printf("UNKNOWN\n");
        }
    }

    return 0;
}

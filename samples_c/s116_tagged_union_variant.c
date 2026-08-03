#include <stdio.h>

/* Girdi: ilk satirda sekil sayisi n, ardindan n satir - "C <r>" (daire,
   yaricap r) veya "R <w> <h>" (dikdortgen, genislik/yukseklik) - r,w,h
   TAM SAYI (alan hesaplari da tam sayi olarak yapilir, PI=3 kullanilir
   basitlik icin).
   Cikti: her sekil icin alani, sonda toplam alan.
   NOT: C'nin etiketli union'i (tag + union {circle;rect;}) Rust'in enum'una
   (her varyant kendi verisini tasir) DOGAL olarak esler - bu, "kolay"
   bir cevirinin nasil goruneceginin bir kontrol ornegidir (bkz. s117-120
   ile karsilastir). */

typedef enum { SHAPE_CIRCLE, SHAPE_RECT } ShapeTag;

typedef struct {
    int radius;
} Circle;

typedef struct {
    int width;
    int height;
} Rect;

typedef struct {
    ShapeTag tag;
    union {
        Circle circle;
        Rect rect;
    } data;
} Shape;

static long shape_area(const Shape *s) {
    if (s->tag == SHAPE_CIRCLE) {
        return 3L * s->data.circle.radius * s->data.circle.radius;
    } else {
        return (long)s->data.rect.width * s->data.rect.height;
    }
}

int main(void) {
    int n;
    if (scanf("%d", &n) != 1) return 0;

    long total = 0;
    for (int i = 0; i < n; i++) {
        char kind[4];
        if (scanf("%3s", kind) != 1) return 0;
        Shape s;
        if (kind[0] == 'C') {
            int r;
            if (scanf("%d", &r) != 1) return 0;
            s.tag = SHAPE_CIRCLE;
            s.data.circle.radius = r;
        } else {
            int w, h;
            if (scanf("%d %d", &w, &h) != 2) return 0;
            s.tag = SHAPE_RECT;
            s.data.rect.width = w;
            s.data.rect.height = h;
        }
        long a = shape_area(&s);
        printf("%ld\n", a);
        total += a;
    }

    printf("total=%ld\n", total);
    return 0;
}

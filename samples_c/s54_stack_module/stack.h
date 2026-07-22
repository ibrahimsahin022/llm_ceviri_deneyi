/*
 * Faz 3 (cok dosyali/gercekci kod) - klasik bir C modul deseni: paylasilan
 * bir baslik (.h) dosyasinda struct + fonksiyon bildirimleri, uygulama
 * ayri bir .c dosyasinda (stack.c), kullanim ayri bir .c dosyasinda
 * (main.c). Amac: LLM'in TEK dosya degil, birbirine bagli birden fazla
 * dosyayi tutarli bicimde cevirip cevirmedigini (orn. Stack struct'inin
 * her iki dosyada da ayni sekilde temsil edilmesini) sinamak.
 */
#ifndef STACK_H
#define STACK_H

#define STACK_CAPACITY 100

typedef struct {
    int data[STACK_CAPACITY];
    int top; /* -1 = bos */
} Stack;

void stack_init(Stack *s);
int stack_is_empty(const Stack *s);
int stack_is_full(const Stack *s);
/* basarili push -> 1, kapasite dolu -> 0 */
int stack_push(Stack *s, int value);
/* basarili pop -> 1 ve *out doldurulur, bos -> 0 */
int stack_pop(Stack *s, int *out);
/* basarili peek -> 1 ve *out doldurulur, bos -> 0 */
int stack_peek(const Stack *s, int *out);
int stack_size(const Stack *s);

#endif

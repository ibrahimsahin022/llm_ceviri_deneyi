/*
 * Kaynak: cJSON (DaveGamble/cJSON), cJSON.c icindeki parse_number() ve
 *         print_number() fonksiyonlari.
 * URL:    https://raw.githubusercontent.com/DaveGamble/cJSON/master/cJSON.c
 * Lisans: MIT (Copyright (c) 2009-2017 Dave Gamble and cJSON contributors).
 *
 * cJSON, GitHub'da en yaygin kullanilan C JSON kutuphanelerinden biridir
 * (10.000+ bagimli proje). Bu ornek, veri setimizdeki mevcut %g bicimlendirme
 * kategorisini (D: s15, s27) TAMAMEN BAGIMSIZ, gercek ve yaygin kullanilan
 * baska bir kod tabaninda tekrar sinamak icin secildi: cJSON'un sayi
 * yazdirma mantigi, once "%1.15g" ile dener, sonucu strtod ile geri okuyup
 * ORIJINAL double DEGERINI TAM olarak geri verip vermedigini kontrol eder;
 * eger vermiyorsa "%1.17g" ile yeniden yazar. Bu round-trip-garantili
 * kayan-nokta yazdirma stratejisi, LLM'in dogal Rust cevirisinde (Rust'ta
 * varsayilan {} bicimlendirmesi zaten round-trip garantili olan farkli bir
 * algoritma - Grisu/Ryu ailesi - kullanir) semantik olarak esdeger mi
 * uretecek, yoksa farkli hane sayilari mi verecek sorusunu test eder.
 *
 * DEGISTIRILEN: (1) cJSON'un tum agac/bellek-havuzu altyapisi (cJSON_Parse,
 * cJSON_Print, obje/dizi turleri, hooks sistemi) CIKARILDI; sadece
 * parse_number() ve print_number() govdeleri ve bunlarin dogrudan
 * bagimliliklari (internal_hooks, parse_buffer, printbuffer, ensure(),
 * update_offset(), compare_double(), can_access_at_index/buffer_at_offset
 * makrolari, sadece gerekli alanlarla kisaltilmis cJSON struct'i)
 * BIRAKILDI - govdeleri satir satir orijinalle aynidir. (2)
 * get_decimal_point() basitlestirildi: orijinalinde #ifdef ENABLE_LOCALES
 * ile locale'e gore ondalik ayrac karakteri degisebiliyordu; ENABLE_LOCALES
 * varsayilan olarak zaten TANIMLI DEGILDIR (cJSON'un varsayilan derleme
 * modu), bu yuzden orijinal davranis da her zaman '.' donduruyordu - burada
 * bu varsayilan dal aynen korunup #ifdef budandi. (3) internal_hooks,
 * dogrudan malloc/realloc/free'e baglandi (orijinalinde de varsayilan
 * global_hooks tam olarak bunlardi). (4) main() suruculeri EKLENDI.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <float.h>
#include <limits.h>

typedef int cJSON_bool;
#define true ((cJSON_bool)1)
#define false ((cJSON_bool)0)
#define cJSON_Number (1 << 3)

typedef struct
{
    void *(*allocate)(size_t size);
    void (*deallocate)(void *pointer);
    void *(*reallocate)(void *pointer, size_t size);
} internal_hooks;

static internal_hooks global_hooks = { malloc, free, realloc };

/* kisaltilmis cJSON: sadece sayi alanlari */
typedef struct cJSON
{
    int type;
    int valueint;
    double valuedouble;
} cJSON;

typedef struct
{
    const unsigned char *content;
    size_t length;
    size_t offset;
    size_t depth;
    internal_hooks hooks;
} parse_buffer;

#define can_access_at_index(buffer, index) ((buffer != NULL) && (((buffer)->offset + index) < (buffer)->length))
#define buffer_at_offset(buffer) ((buffer)->content + (buffer)->offset)

typedef struct
{
    unsigned char *buffer;
    size_t length;
    size_t offset;
    size_t depth;
    cJSON_bool noalloc;
    cJSON_bool format;
    internal_hooks hooks;
} printbuffer;

static unsigned char get_decimal_point(void)
{
    return '.';
}

static cJSON_bool compare_double(double a, double b)
{
    double maxVal = fabs(a) > fabs(b) ? fabs(a) : fabs(b);
    return (fabs(a - b) <= maxVal * DBL_EPSILON);
}

static unsigned char* ensure(printbuffer * const p, size_t needed)
{
    unsigned char *newbuffer = NULL;
    size_t newsize = 0;

    if ((p == NULL) || (p->buffer == NULL))
    {
        return NULL;
    }

    if ((p->length > 0) && (p->offset >= p->length))
    {
        return NULL;
    }

    if (needed > INT_MAX)
    {
        return NULL;
    }

    needed += p->offset + 1;
    if (needed <= p->length)
    {
        return p->buffer + p->offset;
    }

    if (p->noalloc) {
        return NULL;
    }

    if (needed > (INT_MAX / 2))
    {
        if (needed <= INT_MAX)
        {
            newsize = INT_MAX;
        }
        else
        {
            return NULL;
        }
    }
    else
    {
        newsize = needed * 2;
    }

    if (p->hooks.reallocate != NULL)
    {
        newbuffer = (unsigned char*)p->hooks.reallocate(p->buffer, newsize);
        if (newbuffer == NULL)
        {
            p->hooks.deallocate(p->buffer);
            p->length = 0;
            p->buffer = NULL;

            return NULL;
        }
    }
    else
    {
        newbuffer = (unsigned char*)p->hooks.allocate(newsize);
        if (!newbuffer)
        {
            p->hooks.deallocate(p->buffer);
            p->length = 0;
            p->buffer = NULL;

            return NULL;
        }

        memcpy(newbuffer, p->buffer, p->offset + 1);
        p->hooks.deallocate(p->buffer);
    }
    p->length = newsize;
    p->buffer = newbuffer;

    return newbuffer + p->offset;
}

static void update_offset(printbuffer * const buffer)
{
    const unsigned char *buffer_pointer = NULL;
    if ((buffer == NULL) || (buffer->buffer == NULL))
    {
        return;
    }
    buffer_pointer = buffer->buffer + buffer->offset;

    buffer->offset += strlen((const char*)buffer_pointer);
}

/* ---- Orijinal cJSON govdeleri (degismedi) ---- */

static cJSON_bool parse_number(cJSON * const item, parse_buffer * const input_buffer)
{
    double number = 0;
    unsigned char *after_end = NULL;
    unsigned char *number_c_string;
    unsigned char decimal_point = get_decimal_point();
    size_t i = 0;
    size_t number_string_length = 0;
    cJSON_bool has_decimal_point = false;

    if ((input_buffer == NULL) || (input_buffer->content == NULL))
    {
        return false;
    }

    for (i = 0; can_access_at_index(input_buffer, i); i++)
    {
        switch (buffer_at_offset(input_buffer)[i])
        {
            case '0':
            case '1':
            case '2':
            case '3':
            case '4':
            case '5':
            case '6':
            case '7':
            case '8':
            case '9':
            case '+':
            case '-':
            case 'e':
            case 'E':
                number_string_length++;
                break;

            case '.':
                number_string_length++;
                has_decimal_point = true;
                break;

            default:
                goto loop_end;
        }
    }
loop_end:
    number_c_string = (unsigned char *) input_buffer->hooks.allocate(number_string_length + 1);
    if (number_c_string == NULL)
    {
        return false;
    }

    memcpy(number_c_string, buffer_at_offset(input_buffer), number_string_length);
    number_c_string[number_string_length] = '\0';

    if (has_decimal_point)
    {
        for (i = 0; i < number_string_length; i++)
        {
            if (number_c_string[i] == '.')
            {
                number_c_string[i] = decimal_point;
            }
        }
    }

    number = strtod((const char*)number_c_string, (char**)&after_end);
    if (number_c_string == after_end)
    {
        input_buffer->hooks.deallocate(number_c_string);
        return false;
    }

    item->valuedouble = number;

    if (number >= INT_MAX)
    {
        item->valueint = INT_MAX;
    }
    else if (number <= (double)INT_MIN)
    {
        item->valueint = INT_MIN;
    }
    else
    {
        item->valueint = (int)number;
    }

    item->type = cJSON_Number;

    input_buffer->offset += (size_t)(after_end - number_c_string);
    input_buffer->hooks.deallocate(number_c_string);
    return true;
}

static cJSON_bool print_number(const cJSON * const item, printbuffer * const output_buffer)
{
    unsigned char *output_pointer = NULL;
    double d = item->valuedouble;
    int length = 0;
    size_t i = 0;
    unsigned char number_buffer[26] = {0};
    unsigned char decimal_point = get_decimal_point();
    double test = 0.0;

    if (output_buffer == NULL)
    {
        return false;
    }

    if (isnan(d) || isinf(d))
    {
        length = sprintf((char*)number_buffer, "null");
    }
    else if(d == (double)item->valueint)
    {
        length = sprintf((char*)number_buffer, "%d", item->valueint);
    }
    else
    {
        length = sprintf((char*)number_buffer, "%1.15g", d);

        if ((sscanf((char*)number_buffer, "%lg", &test) != 1) || !compare_double((double)test, d))
        {
            length = sprintf((char*)number_buffer, "%1.17g", d);
        }
    }

    if ((length < 0) || (length > (int)(sizeof(number_buffer) - 1)))
    {
        return false;
    }

    output_pointer = ensure(output_buffer, (size_t)length + sizeof(""));
    if (output_pointer == NULL)
    {
        return false;
    }

    for (i = 0; i < ((size_t)length); i++)
    {
        if (number_buffer[i] == decimal_point)
        {
            output_pointer[i] = '.';
            continue;
        }

        output_pointer[i] = number_buffer[i];
    }
    output_pointer[i] = '\0';

    output_buffer->offset += (size_t)length;

    return true;
}

/* ---- Suruc (bu calisma icin eklendi): stdin'den N, sonra N sayi
 * dizgesi (JSON sayi token'i, orn. "3.14", "1e10", "-0") okur. Her biri
 * icin once parse_number ile cozumler, sonra print_number ile geri
 * yazdirir; boylece cJSON'un gercek parse->print round-trip'i sinanmis
 * olur. ---- */
int main(void)
{
    int n;
    if (scanf("%d", &n) != 1) return 1;
    getchar();

    char line[128];
    for (int k = 0; k < n; k++) {
        if (!fgets(line, sizeof(line), stdin)) break;
        size_t l = strlen(line);
        while (l > 0 && (line[l-1] == '\n' || line[l-1] == '\r')) line[--l] = '\0';

        parse_buffer pb = {0};
        pb.content = (const unsigned char *)line;
        pb.length = l;
        pb.offset = 0;
        pb.hooks = global_hooks;

        cJSON item = {0};
        if (!parse_number(&item, &pb)) {
            printf("PARSE_ERROR\n");
            continue;
        }

        unsigned char outbuf[64] = {0};
        printbuffer out = {0};
        out.buffer = outbuf;
        out.length = sizeof(outbuf);
        out.offset = 0;
        out.hooks = global_hooks;

        if (!print_number(&item, &out)) {
            printf("PRINT_ERROR\n");
            continue;
        }
        printf("%s\n", outbuf);
    }
    return 0;
}

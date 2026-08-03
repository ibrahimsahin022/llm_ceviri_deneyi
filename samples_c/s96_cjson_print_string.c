#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Kaynak: cJSON (DaveGamble/cJSON) cJSON.c, print_string_ptr() - JSON
   string kacislama mantigi birebir.
   https://raw.githubusercontent.com/DaveGamble/cJSON/master/cJSON.c
   Lisans: MIT.
   DEGISIKLIK: cJSON'un kendi buyume-tamponu (printbuffer/ensure())
   soyutlamasi, sabit boyutlu bir yerel dizi ile degistirildi (test
   girdileri kisa oldugu icin tampon tasmasi riski yok); asil KACISLAMA
   DONGUSU (\", \\, \b, \f, \n, \r, \t icin tek-karakter kacis, < 32 icin
   \\uXXXX) DEGISTIRILMEDEN alindi.
   Girdi: bir satir metin.
   Cikti: JSON-uyumlu, tirnakli ve kacislanmis metin. */

static void print_string_ptr(const unsigned char *input, char *output) {
    const unsigned char *input_pointer = NULL;
    char *output_pointer = NULL;
    size_t output_length = 0;
    size_t escape_characters = 0;

    if (input == NULL) {
        strcpy(output, "\"\"");
        return;
    }

    for (input_pointer = input; *input_pointer; input_pointer++) {
        switch (*input_pointer) {
            case '\"':
            case '\\':
            case '\b':
            case '\f':
            case '\n':
            case '\r':
            case '\t':
                escape_characters++;
                break;
            default:
                if (*input_pointer < 32) {
                    escape_characters += 5;
                }
                break;
        }
    }
    output_length = (size_t)(input_pointer - input) + escape_characters;

    if (escape_characters == 0) {
        output[0] = '\"';
        memcpy(output + 1, input, output_length);
        output[output_length + 1] = '\"';
        output[output_length + 2] = '\0';
        return;
    }

    output[0] = '\"';
    output_pointer = output + 1;
    for (input_pointer = input; *input_pointer != '\0'; (void)input_pointer++, output_pointer++) {
        if ((*input_pointer > 31) && (*input_pointer != '\"') && (*input_pointer != '\\')) {
            *output_pointer = (char)*input_pointer;
        } else {
            *output_pointer++ = '\\';
            switch (*input_pointer) {
                case '\\':
                    *output_pointer = '\\';
                    break;
                case '\"':
                    *output_pointer = '\"';
                    break;
                case '\b':
                    *output_pointer = 'b';
                    break;
                case '\f':
                    *output_pointer = 'f';
                    break;
                case '\n':
                    *output_pointer = 'n';
                    break;
                case '\r':
                    *output_pointer = 'r';
                    break;
                case '\t':
                    *output_pointer = 't';
                    break;
                default:
                    sprintf(output_pointer, "u%04x", *input_pointer);
                    output_pointer += 4;
                    break;
            }
        }
    }
    output[output_length + 1] = '\"';
    output[output_length + 2] = '\0';
}

int main(void) {
    char line[256];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    char out[1024];
    print_string_ptr((const unsigned char *)line, out);
    printf("%s\n", out);
    return 0;
}

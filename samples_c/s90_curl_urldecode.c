#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <ctype.h>

/* Kaynak: curl lib/escape.c, Curl_urldecode() (curl-7_88_1 etiketi,
   birebir).
   https://raw.githubusercontent.com/curl/curl/curl-7_88_1/lib/escape.c
   Lisans: curl/MIT-benzeri (SPDX: curl).
   DEGISIKLIK: curl'un kendi ISXDIGIT/onehex2dec makrolari, curl_ctype.h'daki
   ozel (locale'siz) tablo yerine islevsel esdegeri standart <ctype.h>
   isxdigit() ile degistirildi (bu makale HTTP kutuphanesi degil, C->Rust
   ceviri deneyi oldugu icin locale bagimsizligi konu disi); CURLcode/
   malloc/DEBUGASSERT gibi kutuphane-ici altyapi soyutlamalari da yerel
   escape.h/curl.h olmadan derlenebilecek sekilde sadelestirildi.
   Curl_urldecode GOVDESININ MANTIGI (yuzde-kod cozme dongusu) DEGISTIRILMEDEN
   alindi.
   Girdi: bir satir, yuzde-kodlanmis metin (orn. "a%20b%2Bc").
   Cikti: cozulmus metin. */

typedef enum { REJECT_NADA, REJECT_CTRL, REJECT_ZERO } urlreject;

#define ISXDIGIT(x) (isxdigit((unsigned char)(x)))

static int hexval(char x) {
    if (x >= '0' && x <= '9') return x - '0';
    if (x >= 'a' && x <= 'f') return x - 'a' + 10;
    if (x >= 'A' && x <= 'F') return x - 'A' + 10;
    return 0;
}

static int Curl_urldecode(const char *string, size_t length,
                           char **ostring, size_t *olen,
                           urlreject ctrl)
{
  size_t alloc;
  char *ns;

  alloc = (length ? length : strlen(string));
  ns = malloc(alloc + 1);
  if (!ns) return 1;

  *ostring = ns;

  while (alloc) {
    unsigned char in = (unsigned char)*string;
    if (('%' == in) && (alloc > 2) &&
        ISXDIGIT(string[1]) && ISXDIGIT(string[2])) {
      in = (unsigned char)((hexval(string[1]) << 4) | hexval(string[2]));
      string += 3;
      alloc -= 3;
    } else {
      string++;
      alloc--;
    }

    if (((ctrl == REJECT_CTRL) && (in < 0x20)) ||
        ((ctrl == REJECT_ZERO) && (in == 0))) {
      free(*ostring);
      return 1;
    }

    *ns++ = (char)in;
  }
  *ns = 0;

  if (olen) *olen = ns - *ostring;
  return 0;
}

int main(void) {
    char line[512];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    char *out = NULL;
    size_t olen = 0;
    if (Curl_urldecode(line, 0, &out, &olen, REJECT_NADA) == 0) {
        printf("%s\n", out);
        free(out);
    }
    return 0;
}

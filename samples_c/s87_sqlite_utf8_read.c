#include <stdio.h>
#include <string.h>

/* Kaynak: SQLite src/utf.c, sqlite3Utf8Read() ve sqlite3Utf8Trans1[] tablosu
   (birebir).
   https://raw.githubusercontent.com/sqlite/sqlite/master/src/utf.c
   Lisans: Public Domain.
   Girdi: bir satir UTF-8 metin.
   Cikti: metindeki her Unicode kod noktasi, ondalik olarak, bosluk ile
   ayrilmis. */

static const unsigned char sqlite3Utf8Trans1[] = {
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
  0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
  0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
  0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
  0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
  0x00, 0x01, 0x02, 0x03, 0x00, 0x01, 0x00, 0x00,
};

unsigned int sqlite3Utf8Read(
  const unsigned char **pz
){
  unsigned int c;
  c = *((*pz)++);
  if( c>=0xc0 ){
    c = sqlite3Utf8Trans1[c-0xc0];
    while( (*(*pz) & 0xc0)==0x80 ){
      c = (c<<6) + (0x3f & *((*pz)++));
    }
    if( c<0x80
        || (c&0xFFFFF800)==0xD800
        || (c&0xFFFFFFFE)==0xFFFE ){  c = 0xFFFD; }
  }
  return c;
}

int main(void) {
    char line[512];
    if (!fgets(line, sizeof(line), stdin)) return 0;
    line[strcspn(line, "\r\n")] = 0;
    const unsigned char *z = (const unsigned char *)line;
    int first = 1;
    while (*z) {
        unsigned int c = sqlite3Utf8Read(&z);
        if (!first) printf(" ");
        printf("%u", c);
        first = 0;
    }
    printf("\n");
    return 0;
}

#include <stdio.h>
#include <string.h>

/* Kaynak: SQLite src/util.c, sqlite3_stricmp() / sqlite3StrICmp() (birebir).
   https://raw.githubusercontent.com/sqlite/sqlite/master/src/util.c
   Lisans: Public Domain (SQLite'in kendi lisans beyani, "The author disclaims
   copyright to this source code").
   DEGISIKLIK: sqlite3UpperToLower tablosunun ic kodlamasi SQLite'ta bit-
   sikistirilmis bir CtypeMap uzerinden turetilir; burada ayni ASCII
   davranisini (A-Z -> a-z, digerleri degismez) veren standart esdeger
   256-baytlik tablo kullanildi. sqlite3StrICmp govdesi DEGISTIRILMEDEN
   alindi.
   Girdi: iki satir metin (zLeft, zRight).
   Cikti: sqlite3StrICmp'nin dondurdugu HAM int fark degeri (normallestirilmis
   -1/0/1 DEGIL - tam sayisal fark, C referansiyla birebir karsilastirma
   icin). */

static unsigned char UpperToLower[256];

static void initUpperToLower(void) {
    for (int i = 0; i < 256; i++) {
        if (i >= 'A' && i <= 'Z') UpperToLower[i] = (unsigned char)(i + 32);
        else UpperToLower[i] = (unsigned char)i;
    }
}

int sqlite3StrICmp(const char *zLeft, const char *zRight){
  unsigned char *a, *b;
  int c, x;
  a = (unsigned char *)zLeft;
  b = (unsigned char *)zRight;
  for(;;){
    c = *a;
    x = *b;
    if( c==x ){
      if( c==0 ) break;
    }else{
      c = (int)UpperToLower[c] - (int)UpperToLower[x];
      if( c ) break;
    }
    a++;
    b++;
  }
  return c;
}

int sqlite3_stricmp(const char *zLeft, const char *zRight){
  if( zLeft==0 ){
    return zRight ? -1 : 0;
  }else if( zRight==0 ){
    return 1;
  }
  return sqlite3StrICmp(zLeft, zRight);
}

int main(void) {
    initUpperToLower();
    char left[256], right[256];
    if (!fgets(left, sizeof(left), stdin)) return 0;
    if (!fgets(right, sizeof(right), stdin)) return 0;
    left[strcspn(left, "\r\n")] = 0;
    right[strcspn(right, "\r\n")] = 0;
    printf("%d\n", sqlite3_stricmp(left, right));
    return 0;
}

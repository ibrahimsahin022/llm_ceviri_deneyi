#include <stdio.h>
#include <string.h>

/* Kaynak: SQLite src/func.c, patternCompare() (GLOB eslestirme motoru,
   sqlite3_strglob'un cekirdegi) - birebir alindi.
   https://raw.githubusercontent.com/sqlite/sqlite/master/src/func.c
   + src/utf.c, sqlite3Utf8Read() (birebir).
   Lisans: Public Domain.
   DEGISIKLIK: Yalnizca ana dosya bagimliliklari (Utf8Read makrosu, u8/u32
   tipleri, SQLITE_MATCH sabitleri) yerel olarak yeniden tanimlandi;
   patternCompare govdesi DEGISTIRILMEDEN alindi.
   Girdi: iki satir - once GLOB deseni, sonra karsilastirilacak metin.
   Cikti: "MATCH" veya "NOMATCH". */

typedef unsigned char u8;
typedef unsigned int u32;

#define SQLITE_MATCH             0
#define SQLITE_NOMATCH           1
#define SQLITE_NOWILDCARDMATCH   2

struct compareInfo {
  u8 matchAll;
  u8 matchOne;
  u8 matchSet;
  u8 noCase;
};

static const struct compareInfo globInfo = { '*', '?', '[', 0 };

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

static u32 sqlite3Utf8Read(const unsigned char **pz){
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

#define Utf8Read(A) sqlite3Utf8Read(&(A))
#define sqlite3Toupper(x) (((x)>='a'&&(x)<='z')?(x)-32:(x))
#define sqlite3Tolower(x) (((x)>='A'&&(x)<='Z')?(x)+32:(x))
#define SQLITE_SKIP_UTF8(z) { if( (*(z++))>=0xc0 ){ while( (*z & 0xc0)==0x80 ){ z++; } } }

static int patternCompare(
  const u8 *zPattern,
  const u8 *zString,
  const struct compareInfo *pInfo,
  u32 matchOther
){
  u32 c, c2;
  u32 matchOne = pInfo->matchOne;
  u32 matchAll = pInfo->matchAll;
  u8 noCase = pInfo->noCase;
  const u8 *zEscaped = 0;

  while( (c = Utf8Read(zPattern))!=0 ){
    if( c==matchAll ){
      while( (c=Utf8Read(zPattern)) == matchAll
             || (c == matchOne && matchOne!=0) ){
        if( c==matchOne && sqlite3Utf8Read(&zString)==0 ){
          return SQLITE_NOWILDCARDMATCH;
        }
      }
      if( c==0 ){
        return SQLITE_MATCH;
      }else if( c==matchOther ){
        if( pInfo->matchSet==0 ){
          c = sqlite3Utf8Read(&zPattern);
          if( c==0 ) return SQLITE_NOWILDCARDMATCH;
        }else{
          while( *zString ){
            int bMatch = patternCompare(&zPattern[-1],zString,pInfo,matchOther);
            if( bMatch!=SQLITE_NOMATCH ) return bMatch;
            SQLITE_SKIP_UTF8(zString);
          }
          return SQLITE_NOWILDCARDMATCH;
        }
      }

      if( c<0x80 ){
        char zStop[3];
        int bMatch;
        if( noCase ){
          zStop[0] = (char)sqlite3Toupper(c);
          zStop[1] = (char)sqlite3Tolower(c);
          zStop[2] = 0;
        }else{
          zStop[0] = (char)c;
          zStop[1] = 0;
        }
        while(1){
          zString += strcspn((const char*)zString, zStop);
          if( zString[0]==0 ) break;
          zString++;
          bMatch = patternCompare(zPattern,zString,pInfo,matchOther);
          if( bMatch!=SQLITE_NOMATCH ) return bMatch;
        }
      }else{
        int bMatch;
        while( (c2 = Utf8Read(zString))!=0 ){
          if( c2!=c ) continue;
          bMatch = patternCompare(zPattern,zString,pInfo,matchOther);
          if( bMatch!=SQLITE_NOMATCH ) return bMatch;
        }
      }
      return SQLITE_NOWILDCARDMATCH;
    }
    if( c==matchOther ){
      if( pInfo->matchSet==0 ){
        c = sqlite3Utf8Read(&zPattern);
        if( c==0 ) return SQLITE_NOMATCH;
        zEscaped = zPattern;
      }else{
        u32 prior_c = 0;
        int seen = 0;
        int invert = 0;
        c = sqlite3Utf8Read(&zString);
        if( c==0 ) return SQLITE_NOMATCH;
        c2 = sqlite3Utf8Read(&zPattern);
        if( c2=='^' ){
          invert = 1;
          c2 = sqlite3Utf8Read(&zPattern);
        }
        if( c2==']' ){
          if( c==']' ) seen = 1;
          c2 = sqlite3Utf8Read(&zPattern);
        }
        while( c2 && c2!=']' ){
          if( c2=='-' && zPattern[0]!=']' && zPattern[0]!=0 && prior_c>0 ){
            c2 = sqlite3Utf8Read(&zPattern);
            if( c>=prior_c && c<=c2 ) seen = 1;
            prior_c = 0;
          }else{
            if( c==c2 ){
              seen = 1;
            }
            prior_c = c2;
          }
          c2 = sqlite3Utf8Read(&zPattern);
        }
        if( c2==0 || (seen ^ invert)==0 ){
          return SQLITE_NOMATCH;
        }
        continue;
      }
    }
    c2 = Utf8Read(zString);
    if( c==c2 ) continue;
    if( noCase  && sqlite3Tolower(c)==sqlite3Tolower(c2) && c<0x80 && c2<0x80 ){
      continue;
    }
    if( c==matchOne && zPattern!=zEscaped && c2!=0 ) continue;
    return SQLITE_NOMATCH;
  }
  return *zString==0 ? SQLITE_MATCH : SQLITE_NOMATCH;
}

int main(void) {
    char pattern[256], text[256];
    if (!fgets(pattern, sizeof(pattern), stdin)) return 0;
    if (!fgets(text, sizeof(text), stdin)) return 0;
    pattern[strcspn(pattern, "\r\n")] = 0;
    text[strcspn(text, "\r\n")] = 0;
    int r = patternCompare((const u8*)pattern, (const u8*)text, &globInfo, '[');
    printf("%s\n", r == SQLITE_MATCH ? "MATCH" : "NOMATCH");
    return 0;
}

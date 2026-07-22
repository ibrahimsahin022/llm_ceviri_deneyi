/*
 * Kaynak: Redis 7.2.4, src/sds.c ve src/sds.h (SDSLib 2.0 - dinamik string
 *         kutuphanesi)
 * URL:    https://raw.githubusercontent.com/redis/redis/7.2.4/src/sds.c
 *         https://raw.githubusercontent.com/redis/redis/7.2.4/src/sds.h
 * Lisans: BSD-3-Clause (Copyright (c) 2006-2015 Salvatore Sanfilippo ve
 *         digerleri; bkz. https://raw.githubusercontent.com/redis/redis/7.2.4/COPYING).
 *         Not: Redis 8.0+ lisansini degistirdi (RSALv2/SSPL/AGPL); bu ornek
 *         bilerek hala BSD-3 altinda olan 7.2.4 etiketinden alinmistir.
 *
 * SDS (Simple Dynamic Strings), Redis'in ic string temsili olarak
 * kullandigi, degisken genislikte basliklarla (8/16/32/64 bit len+alloc)
 * bellek verimliligi saglayan gercek uretim kodudur. Bu ornek, onceki veri
 * setimizdeki sabit-genislik string islemlerinden (s06/s13) cok daha
 * karmasik bir gercek dunya deseni test eder: sds POINTER'I DOGRUDAN
 * BUFFER'IN BASINA isaret eder, baslik bilgisi (len/alloc/flags) pointer'in
 * HEMEN ONCESINDE, gizli olarak saklanir (s[-1] ile flags byte'ina erisim,
 * pointer aritmetigiyle baslik struct'ina donus). Bu, C'nin pointer modeli
 * disinda dogal karsiligi olmayan bir tasarimdir ve Rust'a cevrildiginde
 * ya unsafe pointer aritmetigi gerektirir ya da (daha olasi) LLM'in
 * String/Vec tabanli tamamen farkli, guvenli bir ic temsile gecmesini
 * gerektirir - bu da orijinal bellek duzeninin anlamsal olarak neye
 * karsilik geldigini doğru yakalayip yakalamadigini test eder.
 *
 * DEGISTIRILEN: (1) Redis'in kendi bellek ayirici sarmalayicilari
 * (s_malloc_usable/s_trymalloc_usable/s_realloc_usable/s_free, normalde
 * "sdsalloc.h" -> zmalloc.h icinde tanimli, malloc_usable_size() ile
 * gercek ayrilan boyutu geri bildiren bir katman) tasinabilir basit bir
 * malloc/realloc/free sarmalayicisiyla degistirildi ("usable" = istenen
 * boyutun ta kendisi, fazladan kapasite raporlanmiyor). Bu, algoritmanin
 * mantigini DEGISTIRMEZ, sadece Redis'e ozel bir allocator soyutlamasini
 * (kendisi de zaten degistirlebilir bir katman olarak tasarlanmis) yerine
 * koyar. (2) "sdsalloc.h"/"util.h" bagimliliklari kaldirildi, assert.h
 * kullanimindaki assert() cagrilari aynen birakildi. (3) sdssplitlen,
 * sdscatprintf, sdstemplate, sdsmapchars gibi bu ornekte kullanilmayan
 * fonksiyonlar CIKARILDI (dosyayi makul boyutta tutmak icin) - kullanilan
 * her fonksiyonun govdesi (sdsHdrSize, sdsReqType, sdsTypeMaxSize,
 * _sdsnewlen, sdsnewlen, sdsempty, sdsnew, sdsfree, _sdsMakeRoomFor,
 * sdsMakeRoomFor, sdscatlen, sdscat, sdstrim, sdssubstr, sdsrange,
 * sdstolower, sdstoupper, sdscmp ve sds.h'deki inline erisimciler) satir
 * satir orijinal kaynakla ayni birakildi. (4) main() suruculeri EKLENDI.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <assert.h>
#include <limits.h>
#include <sys/types.h>
#include <stdint.h>

/* ---- sds.h'den: tip tanimlari ve inline erisimciler (degismedi) ---- */

#define SDS_MAX_PREALLOC (1024*1024)

typedef char *sds;

struct __attribute__ ((__packed__)) sdshdr5 {
    unsigned char flags;
    char buf[];
};
struct __attribute__ ((__packed__)) sdshdr8 {
    uint8_t len;
    uint8_t alloc;
    unsigned char flags;
    char buf[];
};
struct __attribute__ ((__packed__)) sdshdr16 {
    uint16_t len;
    uint16_t alloc;
    unsigned char flags;
    char buf[];
};
struct __attribute__ ((__packed__)) sdshdr32 {
    uint32_t len;
    uint32_t alloc;
    unsigned char flags;
    char buf[];
};
struct __attribute__ ((__packed__)) sdshdr64 {
    uint64_t len;
    uint64_t alloc;
    unsigned char flags;
    char buf[];
};

#define SDS_TYPE_5  0
#define SDS_TYPE_8  1
#define SDS_TYPE_16 2
#define SDS_TYPE_32 3
#define SDS_TYPE_64 4
#define SDS_TYPE_MASK 7
#define SDS_TYPE_BITS 3
#define SDS_HDR_VAR(T,s) struct sdshdr##T *sh = (void*)((s)-(sizeof(struct sdshdr##T)));
#define SDS_HDR(T,s) ((struct sdshdr##T *)((s)-(sizeof(struct sdshdr##T))))
#define SDS_TYPE_5_LEN(f) ((f)>>SDS_TYPE_BITS)

static inline size_t sdslen(const sds s) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            return SDS_TYPE_5_LEN(flags);
        case SDS_TYPE_8:
            return SDS_HDR(8,s)->len;
        case SDS_TYPE_16:
            return SDS_HDR(16,s)->len;
        case SDS_TYPE_32:
            return SDS_HDR(32,s)->len;
        case SDS_TYPE_64:
            return SDS_HDR(64,s)->len;
    }
    return 0;
}

static inline size_t sdsavail(const sds s) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5: {
            return 0;
        }
        case SDS_TYPE_8: {
            SDS_HDR_VAR(8,s);
            return sh->alloc - sh->len;
        }
        case SDS_TYPE_16: {
            SDS_HDR_VAR(16,s);
            return sh->alloc - sh->len;
        }
        case SDS_TYPE_32: {
            SDS_HDR_VAR(32,s);
            return sh->alloc - sh->len;
        }
        case SDS_TYPE_64: {
            SDS_HDR_VAR(64,s);
            return sh->alloc - sh->len;
        }
    }
    return 0;
}

static inline void sdssetlen(sds s, size_t newlen) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            {
                unsigned char *fp = ((unsigned char*)s)-1;
                *fp = SDS_TYPE_5 | (newlen << SDS_TYPE_BITS);
            }
            break;
        case SDS_TYPE_8:
            SDS_HDR(8,s)->len = newlen;
            break;
        case SDS_TYPE_16:
            SDS_HDR(16,s)->len = newlen;
            break;
        case SDS_TYPE_32:
            SDS_HDR(32,s)->len = newlen;
            break;
        case SDS_TYPE_64:
            SDS_HDR(64,s)->len = newlen;
            break;
    }
}

static inline size_t sdsalloc(const sds s) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            return SDS_TYPE_5_LEN(flags);
        case SDS_TYPE_8:
            return SDS_HDR(8,s)->alloc;
        case SDS_TYPE_16:
            return SDS_HDR(16,s)->alloc;
        case SDS_TYPE_32:
            return SDS_HDR(32,s)->alloc;
        case SDS_TYPE_64:
            return SDS_HDR(64,s)->alloc;
    }
    return 0;
}

static inline void sdssetalloc(sds s, size_t newlen) {
    unsigned char flags = s[-1];
    switch(flags&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            break;
        case SDS_TYPE_8:
            SDS_HDR(8,s)->alloc = newlen;
            break;
        case SDS_TYPE_16:
            SDS_HDR(16,s)->alloc = newlen;
            break;
        case SDS_TYPE_32:
            SDS_HDR(32,s)->alloc = newlen;
            break;
        case SDS_TYPE_64:
            SDS_HDR(64,s)->alloc = newlen;
            break;
    }
}

/* ---- tasinabilir allocator sarmalayicisi (degisiklik: bkz. yukarida) ---- */
static void *s_malloc_usable(size_t size, size_t *usable) { *usable = size; return malloc(size); }
static void *s_trymalloc_usable(size_t size, size_t *usable) { *usable = size; return malloc(size); }
static void *s_realloc_usable(void *p, size_t size, size_t *usable) { *usable = size; return realloc(p, size); }
static void s_free(void *p) { free(p); }

/* ---- sds.c govdeleri (degismedi) ---- */

static inline int sdsHdrSize(char type) {
    switch(type&SDS_TYPE_MASK) {
        case SDS_TYPE_5:
            return sizeof(struct sdshdr5);
        case SDS_TYPE_8:
            return sizeof(struct sdshdr8);
        case SDS_TYPE_16:
            return sizeof(struct sdshdr16);
        case SDS_TYPE_32:
            return sizeof(struct sdshdr32);
        case SDS_TYPE_64:
            return sizeof(struct sdshdr64);
    }
    return 0;
}

static inline char sdsReqType(size_t string_size) {
    if (string_size < 1<<5)
        return SDS_TYPE_5;
    if (string_size < 1<<8)
        return SDS_TYPE_8;
    if (string_size < 1<<16)
        return SDS_TYPE_16;
#if (LONG_MAX == LLONG_MAX)
    if (string_size < 1ll<<32)
        return SDS_TYPE_32;
    return SDS_TYPE_64;
#else
    return SDS_TYPE_32;
#endif
}

static inline size_t sdsTypeMaxSize(char type) {
    if (type == SDS_TYPE_5)
        return (1<<5) - 1;
    if (type == SDS_TYPE_8)
        return (1<<8) - 1;
    if (type == SDS_TYPE_16)
        return (1<<16) - 1;
#if (LONG_MAX == LLONG_MAX)
    if (type == SDS_TYPE_32)
        return (1ll<<32) - 1;
#endif
    return -1;
}

sds _sdsnewlen(const void *init, size_t initlen, int trymalloc) {
    void *sh;
    sds s;
    char type = sdsReqType(initlen);
    if (type == SDS_TYPE_5 && initlen == 0) type = SDS_TYPE_8;
    int hdrlen = sdsHdrSize(type);
    unsigned char *fp;
    size_t usable;

    assert(initlen + hdrlen + 1 > initlen);
    sh = trymalloc?
        s_trymalloc_usable(hdrlen+initlen+1, &usable) :
        s_malloc_usable(hdrlen+initlen+1, &usable);
    if (sh == NULL) return NULL;
    if (!init)
        memset(sh, 0, hdrlen+initlen+1);
    s = (char*)sh+hdrlen;
    fp = ((unsigned char*)s)-1;
    usable = usable-hdrlen-1;
    if (usable > sdsTypeMaxSize(type))
        usable = sdsTypeMaxSize(type);
    switch(type) {
        case SDS_TYPE_5: {
            *fp = type | (initlen << SDS_TYPE_BITS);
            break;
        }
        case SDS_TYPE_8: {
            SDS_HDR_VAR(8,s);
            sh->len = initlen;
            sh->alloc = usable;
            *fp = type;
            break;
        }
        case SDS_TYPE_16: {
            SDS_HDR_VAR(16,s);
            sh->len = initlen;
            sh->alloc = usable;
            *fp = type;
            break;
        }
        case SDS_TYPE_32: {
            SDS_HDR_VAR(32,s);
            sh->len = initlen;
            sh->alloc = usable;
            *fp = type;
            break;
        }
        case SDS_TYPE_64: {
            SDS_HDR_VAR(64,s);
            sh->len = initlen;
            sh->alloc = usable;
            *fp = type;
            break;
        }
    }
    if (initlen && init)
        memcpy(s, init, initlen);
    s[initlen] = '\0';
    return s;
}

sds sdsnewlen(const void *init, size_t initlen) {
    return _sdsnewlen(init, initlen, 0);
}

sds sdsempty(void) {
    return sdsnewlen("",0);
}

sds sdsnew(const char *init) {
    size_t initlen = (init == NULL) ? 0 : strlen(init);
    return sdsnewlen(init, initlen);
}

void sdsfree(sds s) {
    if (s == NULL) return;
    s_free((char*)s-sdsHdrSize(s[-1]));
}

sds _sdsMakeRoomFor(sds s, size_t addlen, int greedy) {
    void *sh, *newsh;
    size_t avail = sdsavail(s);
    size_t len, newlen, reqlen;
    char type, oldtype = s[-1] & SDS_TYPE_MASK;
    int hdrlen;
    size_t usable;

    if (avail >= addlen) return s;

    len = sdslen(s);
    sh = (char*)s-sdsHdrSize(oldtype);
    reqlen = newlen = (len+addlen);
    assert(newlen > len);
    if (greedy == 1) {
        if (newlen < SDS_MAX_PREALLOC)
            newlen *= 2;
        else
            newlen += SDS_MAX_PREALLOC;
    }

    type = sdsReqType(newlen);

    if (type == SDS_TYPE_5) type = SDS_TYPE_8;

    hdrlen = sdsHdrSize(type);
    assert(hdrlen + newlen + 1 > reqlen);
    if (oldtype==type) {
        newsh = s_realloc_usable(sh, hdrlen+newlen+1, &usable);
        if (newsh == NULL) return NULL;
        s = (char*)newsh+hdrlen;
    } else {
        newsh = s_malloc_usable(hdrlen+newlen+1, &usable);
        if (newsh == NULL) return NULL;
        memcpy((char*)newsh+hdrlen, s, len+1);
        s_free(sh);
        s = (char*)newsh+hdrlen;
        s[-1] = type;
        sdssetlen(s, len);
    }
    usable = usable-hdrlen-1;
    if (usable > sdsTypeMaxSize(type))
        usable = sdsTypeMaxSize(type);
    sdssetalloc(s, usable);
    return s;
}

sds sdsMakeRoomFor(sds s, size_t addlen) {
    return _sdsMakeRoomFor(s, addlen, 1);
}

sds sdscatlen(sds s, const void *t, size_t len) {
    size_t curlen = sdslen(s);

    s = sdsMakeRoomFor(s,len);
    if (s == NULL) return NULL;
    memcpy(s+curlen, t, len);
    sdssetlen(s, curlen+len);
    s[curlen+len] = '\0';
    return s;
}

sds sdscat(sds s, const char *t) {
    return sdscatlen(s, t, strlen(t));
}

sds sdstrim(sds s, const char *cset) {
    char *end, *sp, *ep;
    size_t len;

    sp = s;
    ep = end = s+sdslen(s)-1;
    while(sp <= end && strchr(cset, *sp)) sp++;
    while(ep > sp && strchr(cset, *ep)) ep--;
    len = (ep-sp)+1;
    if (s != sp) memmove(s, sp, len);
    s[len] = '\0';
    sdssetlen(s,len);
    return s;
}

void sdssubstr(sds s, size_t start, size_t len) {
    size_t oldlen = sdslen(s);
    if (start >= oldlen) start = len = 0;
    if (len > oldlen-start) len = oldlen-start;

    if (len) memmove(s, s+start, len);
    s[len] = 0;
    sdssetlen(s,len);
}

void sdsrange(sds s, ssize_t start, ssize_t end) {
    size_t newlen, len = sdslen(s);
    if (len == 0) return;
    if (start < 0)
        start = len + start;
    if (end < 0)
        end = len + end;
    newlen = (start > end) ? 0 : (end-start)+1;
    sdssubstr(s, start, newlen);
}

void sdstolower(sds s) {
    size_t len = sdslen(s), j;

    for (j = 0; j < len; j++) s[j] = tolower(s[j]);
}

void sdstoupper(sds s) {
    size_t len = sdslen(s), j;

    for (j = 0; j < len; j++) s[j] = toupper(s[j]);
}

int sdscmp(const sds s1, const sds s2) {
    size_t l1, l2, minlen;
    int cmp;

    l1 = sdslen(s1);
    l2 = sdslen(s2);
    minlen = (l1 < l2) ? l1 : l2;
    cmp = memcmp(s1,s2,minlen);
    if (cmp == 0) return l1>l2? 1: (l1<l2? -1: 0);
    return cmp;
}

/* ---- Suruc (bu calisma icin eklendi) ----
 * Girdi bicimi: ilk satir komut sayisi N, sonra N satir komut:
 *   NEW <metin>       - yeni sds olustur (mevcut olani serbest birakir)
 *   CAT <metin>        - sonuna ekle
 *   TRIM <kume>        - baştaki/sondaki 'kume' karakterlerini kirp
 *   RANGE <start> <end> - alt dizgeye indirge (negatif indeksler destekli)
 *   LOWER              - kucuk harfe cevir
 *   UPPER              - buyuk harfe cevir
 *   CMP <metin>        - gecici bir sds ile sdscmp sonucunu yazdir
 * Her komuttan sonra "LEN=<n> STR=<icerik>" satiri basilir. */
int main(void)
{
    char line[4096];
    int ncmd;
    if (scanf("%d", &ncmd) != 1) return 1;
    getchar();

    sds cur = sdsempty();
    for (int i = 0; i < ncmd; i++) {
        if (!fgets(line, sizeof(line), stdin)) break;
        size_t l = strlen(line);
        while (l > 0 && (line[l-1] == '\n' || line[l-1] == '\r')) line[--l] = '\0';

        char *sp = strchr(line, ' ');
        char cmd[16] = {0};
        char *arg = "";
        if (sp) {
            size_t clen = (size_t)(sp - line);
            if (clen > 15) clen = 15;
            memcpy(cmd, line, clen);
            cmd[clen] = '\0';
            arg = sp + 1;
        } else {
            strncpy(cmd, line, 15);
        }

        if (strcmp(cmd, "NEW") == 0) {
            sdsfree(cur);
            cur = sdsnew(arg);
        } else if (strcmp(cmd, "CAT") == 0) {
            cur = sdscat(cur, arg);
        } else if (strcmp(cmd, "TRIM") == 0) {
            cur = sdstrim(cur, arg);
        } else if (strcmp(cmd, "RANGE") == 0) {
            long a, b;
            sscanf(arg, "%ld %ld", &a, &b);
            sdsrange(cur, (ssize_t)a, (ssize_t)b);
        } else if (strcmp(cmd, "LOWER") == 0) {
            sdstolower(cur);
        } else if (strcmp(cmd, "UPPER") == 0) {
            sdstoupper(cur);
        } else if (strcmp(cmd, "CMP") == 0) {
            sds tmp = sdsnew(arg);
            printf("CMP=%d\n", sdscmp(cur, tmp));
            sdsfree(tmp);
            continue;
        }
        printf("LEN=%zu STR=%s\n", sdslen(cur), cur);
    }
    sdsfree(cur);
    return 0;
}

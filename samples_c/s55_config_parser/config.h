/*
 * Faz 3 (cok dosyali/gercekci kod) - ikinci ornek: paylasilan bir struct
 * (ConfigEntry) tanimi iki AYRI .c dosyasi (parser.c, lookup.c) tarafindan
 * kullaniliyor - bu, gercek C projelerinde cok yaygin bir desendir (bir
 * .h dosyasindaki veri yapisi birden fazla derleme biriminde paylasilir).
 * LLM'in bu paylasimi (struct alan adlari/turleri) iki Rust dosyasinda da
 * TUTARLI bicimde yeniden uretip uretmedigi test edilir.
 */
#ifndef CONFIG_H
#define CONFIG_H

#define MAX_ENTRIES 64
#define MAX_KEY_LEN 32
#define MAX_VAL_LEN 64

typedef struct {
    char key[MAX_KEY_LEN];
    char value[MAX_VAL_LEN];
} ConfigEntry;

/* parser.c */
int parse_config_lines(const char *text, ConfigEntry entries[], int max_entries);

/* lookup.c */
const char *config_lookup(const ConfigEntry entries[], int count, const char *key);

#endif

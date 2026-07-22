#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Girdi: ilk satir K (komut sayisi), ardindan K satir komut.
   Komutlar:
     INSERT <key> <value>   -> tabloya ekler/gunceller, "OK" yazdirir
     GET <key>               -> deger varsa yazdirir, yoksa "NOT_FOUND"
     DEL <key>                -> silerse "OK", yoksa "NOT_FOUND"
   Zincirleme (chaining) ile basit bir hash tablosu (djb2 tabanli). */

#define TABLE_SIZE 64
#define MAX_KEY 64
#define MAX_LINE 256

struct Entry {
    char key[MAX_KEY];
    long value;
    struct Entry *next;
};

static struct Entry *table[TABLE_SIZE];

static unsigned long hash_key(const char *key) {
    unsigned long h = 5381;
    int c;
    while ((c = (unsigned char)*key++)) {
        h = ((h << 5) + h) + (unsigned long)c;
    }
    return h;
}

static struct Entry *find_entry(const char *key) {
    unsigned long idx = hash_key(key) % TABLE_SIZE;
    struct Entry *cur = table[idx];
    while (cur != NULL) {
        if (strcmp(cur->key, key) == 0) return cur;
        cur = cur->next;
    }
    return NULL;
}

static void insert_entry(const char *key, long value) {
    struct Entry *existing = find_entry(key);
    if (existing != NULL) {
        existing->value = value;
        return;
    }
    unsigned long idx = hash_key(key) % TABLE_SIZE;
    struct Entry *n = (struct Entry *)malloc(sizeof(struct Entry));
    strncpy(n->key, key, MAX_KEY - 1);
    n->key[MAX_KEY - 1] = '\0';
    n->value = value;
    n->next = table[idx];
    table[idx] = n;
}

static int delete_entry(const char *key) {
    unsigned long idx = hash_key(key) % TABLE_SIZE;
    struct Entry *cur = table[idx];
    struct Entry *prev = NULL;
    while (cur != NULL) {
        if (strcmp(cur->key, key) == 0) {
            if (prev == NULL) {
                table[idx] = cur->next;
            } else {
                prev->next = cur->next;
            }
            free(cur);
            return 1;
        }
        prev = cur;
        cur = cur->next;
    }
    return 0;
}

static void free_table(void) {
    for (int i = 0; i < TABLE_SIZE; i++) {
        struct Entry *cur = table[i];
        while (cur != NULL) {
            struct Entry *next = cur->next;
            free(cur);
            cur = next;
        }
        table[i] = NULL;
    }
}

static void trim_newline(char *s) {
    size_t len = strlen(s);
    while (len > 0 && (s[len - 1] == '\n' || s[len - 1] == '\r')) {
        s[len - 1] = '\0';
        len--;
    }
}

int main(void) {
    for (int i = 0; i < TABLE_SIZE; i++) table[i] = NULL;

    char line[MAX_LINE];
    if (fgets(line, sizeof(line), stdin) == NULL) return 0;
    int k = atoi(line);

    for (int i = 0; i < k; i++) {
        if (fgets(line, sizeof(line), stdin) == NULL) break;
        trim_newline(line);

        char cmd[16];
        char key[MAX_KEY];
        char rest[MAX_LINE];
        cmd[0] = '\0';
        key[0] = '\0';
        rest[0] = '\0';

        int matched = sscanf(line, "%15s %63s %[^\n]", cmd, key, rest);

        if (strcmp(cmd, "INSERT") == 0 && matched >= 2) {
            long value = 0;
            if (matched >= 3) value = atol(rest);
            insert_entry(key, value);
            printf("OK\n");
        } else if (strcmp(cmd, "GET") == 0 && matched >= 2) {
            struct Entry *e = find_entry(key);
            if (e != NULL) {
                printf("%ld\n", e->value);
            } else {
                printf("NOT_FOUND\n");
            }
        } else if (strcmp(cmd, "DEL") == 0 && matched >= 2) {
            if (delete_entry(key)) {
                printf("OK\n");
            } else {
                printf("NOT_FOUND\n");
            }
        }
    }

    free_table();
    return 0;
}

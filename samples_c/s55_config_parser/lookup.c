#include <string.h>
#include "config.h"

/* key'i bulursa value'ya isaretci dondurur; bulamazsa NULL. */
const char *config_lookup(const ConfigEntry entries[], int count, const char *key)
{
    for (int i = 0; i < count; i++) {
        if (strcmp(entries[i].key, key) == 0) {
            return entries[i].value;
        }
    }
    return NULL;
}

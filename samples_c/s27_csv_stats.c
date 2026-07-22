#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Girdi: ilk satir M (satir sayisi), sonraki M satirin her biri virgulle
   ayrilmis tam sayi degerler icerir (satir basina degisken sayida sutun,
   en fazla MAX_COLS). Cikti: tum degerlerin toplami, ortalamasi (%g),
   min ve maks degeri, ardindan her satirin kendi toplami. */

#define MAX_COLS 32
#define MAX_LINE 1024

static int parse_line(char *line, long values[MAX_COLS]) {
    int count = 0;
    char *tok = strtok(line, ",\n\r");
    while (tok != NULL && count < MAX_COLS) {
        values[count++] = atol(tok);
        tok = strtok(NULL, ",\n\r");
    }
    return count;
}

int main(void) {
    int m;
    char buf[MAX_LINE];

    if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
    m = atoi(buf);

    long total = 0;
    long count_all = 0;
    int have_min = 0;
    long mn = 0, mx = 0;
    long row_sums[256];
    int row_count = 0;

    for (int i = 0; i < m; i++) {
        if (fgets(buf, sizeof(buf), stdin) == NULL) break;
        long values[MAX_COLS];
        int n = parse_line(buf, values);

        long row_sum = 0;
        for (int j = 0; j < n; j++) {
            row_sum += values[j];
            total += values[j];
            count_all++;
            if (!have_min) {
                mn = values[j];
                mx = values[j];
                have_min = 1;
            } else {
                if (values[j] < mn) mn = values[j];
                if (values[j] > mx) mx = values[j];
            }
        }
        if (row_count < 256) row_sums[row_count++] = row_sum;
    }

    double avg = count_all > 0 ? (double)total / (double)count_all : 0.0;

    printf("total=%ld\n", total);
    printf("avg=%g\n", avg);
    if (have_min) {
        printf("min=%ld max=%ld\n", mn, mx);
    } else {
        printf("min=0 max=0\n");
    }
    for (int i = 0; i < row_count; i++) {
        printf("row%d=%ld\n", i, row_sums[i]);
    }

    return 0;
}

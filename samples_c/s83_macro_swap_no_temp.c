#include <stdio.h>

#define SWAP(a, b, tmp) do { tmp = a; a = b; b = tmp; } while (0)

/* Girdi: n (dizi boyutu) ve dizinin elemanlari.
   Cikti: dizinin, YAN ETKILI bir indeks ifadesiyle (i++) SWAP makrosu
   kullanilarak degistirildikten sonraki hali, ve i'nin son degeri.
   NOT: Kok Neden I (makro coklu-degerlendirme) icin 3. bagimsiz ornek
   (s56/s82'den farkli: makro PARAMETRESININ DEGERI degil ERISIM YOLU
   yan etkilidir). SWAP govdesinde 'a' iki kez gecer (tmp=a ve a=b);
   a=arr[i++] ise, i IKI KEZ artar ve ikinci erisim FARKLI bir diziye
   yazar - klasik makro tuzagi. Fonksiyona cevrilen bir Rust karsiligi
   argumani BIR KEZ degerlendirir (i BIR KEZ artar), bu yuzden sonuc
   C'den FARKLI olur. */
int main(void) {
    int n;
    if (scanf("%d", &n) != 1 || n < 2 || n > 100) return 0;
    int arr[100];
    for (int k = 0; k < n; k++) {
        if (scanf("%d", &arr[k]) != 1) return 0;
    }
    int tmp;
    int i = 0;
    SWAP(arr[i++], arr[1], tmp);
    for (int k = 0; k < n; k++) {
        printf("%d ", arr[k]);
    }
    printf("\n");
    printf("i=%d\n", i);
    return 0;
}

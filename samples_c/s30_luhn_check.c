#include <string.h>
#include <stdio.h>

/* KAYNAK: Rosetta Code, "Luhn test of credit card numbers", C cozumu.
   https://rosettacode.org/wiki/Luhn_test_of_credit_card_numbers
   (GFDL 1.2 / CC-BY-SA lisansli, atifla yeniden kullanima acik).
   Algoritma fonksiyonu (luhn) KAYNAKTAN DEGISTIRILMEDEN alinmistir;
   yalnizca main() bu deney ortaminin stdin/stdout sozlesmesine uyacak
   sekilde YENIDEN YAZILMISTIR (orijinalde sabit kodlanmis bir liste vardi). */

int luhn(const char* cc)
{
   const int m[] = {0,2,4,6,8,1,3,5,7,9}; // mapping for rule 3
   int i, odd = 1, sum = 0;

   for (i = strlen(cc); i--; odd = !odd) {
      int digit = cc[i] - '0';
      sum += odd ? digit : m[digit];
   }

   return sum % 10 == 0;
}

int main(void)
{
   char buf[64];
   if (fgets(buf, sizeof(buf), stdin) == NULL) return 0;
   size_t len = strlen(buf);
   while (len > 0 && (buf[len-1] == '\n' || buf[len-1] == '\r')) {
      buf[--len] = '\0';
   }
   printf("%s\n", luhn(buf) ? "ok" : "not ok");
   return 0;
}

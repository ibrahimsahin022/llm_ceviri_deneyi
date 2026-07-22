#include <stdio.h>

/* KAYNAK: Rosetta Code, "Towers of Hanoi", C cozumu.
   https://rosettacode.org/wiki/Towers_of_Hanoi
   (GFDL 1.2 / CC-BY-SA lisansli, atifla yeniden kullanima acik).
   move() fonksiyonu KAYNAKTAN DEGISTIRILMEDEN alinmistir; yalnizca main()
   bu deney ortaminin stdin/stdout sozlesmesine uyacak sekilde YENIDEN
   YAZILMISTIR (orijinalde sabit kodlanmis n=4 vardi). */

void move(int n, int from, int via, int to)
{
  if (n > 1) {
    move(n - 1, from, to, via);
    printf("Move disk from pole %d to pole %d\n", from, to);
    move(n - 1, via, from, to);
  } else {
    printf("Move disk from pole %d to pole %d\n", from, to);
  }
}

int main(void)
{
  int n;
  if (scanf("%d", &n) != 1) return 0;
  if (n > 0) move(n, 1, 2, 3);
  return 0;
}

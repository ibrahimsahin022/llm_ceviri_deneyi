# Istatistiksel Guc ve Guven Araligi Raporu (Faz 2)

Bootstrap/Monte Carlo yontemleri sabit seed (42) ile calisir; aynı veri uzerinde tekrar calistirildiginda ayni sayilari uretir.

## EA Bootstrap %95 Guven Araligi

| Kosul | EA (nokta) | %95 GA |
|---|---|---|
| Round 1 - dogrudan, debug | %69.81 | [%56.60, %81.13] |
| Round 1 - dogrudan, release | %73.58 | [%60.38, %84.91] |
| Round 2 - iyilestirilmis, debug | %100.00 | [%100.00, %100.00] |

## Mann-Whitney U (LoC: PASS vs FAIL) - Etki Buyuklugu ve Guc

- n(FAIL)=16, n(PASS)=37
- U=246.0, p=0.3371
- Rank-biserial korelasyon (etki buyuklugu) r=0.169 (kucuk etki)
- Bootstrap-tabanli gerceklesen guc (achieved power, alpha=0.05, 5000 tekrar): **%15.6**
  (Yorum: gozlemlenen tam bu etki buyuklugunde ve bu n ile, testin tekrar tekrar uygulansaydi ne siklikta anlamli cikacagini gosterir. Dusuk guc, 'anlamli fark yok' sonucunun bir Tip II hatasi olabilecegi anlamina gelir - kesin bir 'iliski yoktur' iddiasi degildir.)

## Fisher Kesin Testi (Isaretci Kullanimi vs PASS/FAIL) - Odds Orani GA

- Tablo (pointer/non-pointer x PASS/FAIL): [[20, 6], [17, 10]]
- Odds orani=1.96, p=0.3718
- Odds orani %95 guven araligi (log-yaklasik): [0.59, 6.52]
  (Guven araliginin 1.0'i icermesi, iliskinin istatistiksel olarak anlamli olmadigini dogrular; aralik cok genistir - kucuk orneklemin dogal bir sonucu.)

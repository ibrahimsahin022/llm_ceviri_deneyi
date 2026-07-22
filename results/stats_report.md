# Istatistiksel Guc ve Guven Araligi Raporu (Faz 2)

Bootstrap/Monte Carlo yontemleri sabit seed (42) ile calisir; aynı veri uzerinde tekrar calistirildiginda ayni sayilari uretir.

## EA Bootstrap %95 Guven Araligi

| Kosul | EA (nokta) | %95 GA |
|---|---|---|
| Round 1 - dogrudan, debug | %70.18 | [%57.89, %82.46] |
| Round 1 - dogrudan, release | %73.68 | [%61.40, %84.21] |
| Round 2 - iyilestirilmis, debug | %100.00 | [%100.00, %100.00] |

## Mann-Whitney U (LoC: PASS vs FAIL) - Etki Buyuklugu ve Guc

- n(FAIL)=17, n(PASS)=40
- U=287.0, p=0.3594
- Rank-biserial korelasyon (etki buyuklugu) r=0.156 (kucuk etki)
- Bootstrap-tabanli gerceklesen guc (achieved power, alpha=0.05, 5000 tekrar): **%15.0**
  (Yorum: gozlemlenen tam bu etki buyuklugunde ve bu n ile, testin tekrar tekrar uygulansaydi ne siklikta anlamli cikacagini gosterir. Dusuk guc, 'anlamli fark yok' sonucunun bir Tip II hatasi olabilecegi anlamina gelir - kesin bir 'iliski yoktur' iddiasi degildir.)

## Fisher Kesin Testi (Isaretci Kullanimi vs PASS/FAIL) - Odds Orani GA

- Tablo (pointer/non-pointer x PASS/FAIL): [[23, 7], [17, 10]]
- Odds orani=1.93, p=0.3851
- Odds orani %95 guven araligi (log-yaklasik): [0.61, 6.11]
  (Guven araliginin 1.0'i icermesi, iliskinin istatistiksel olarak anlamli olmadigini dogrular; aralik cok genistir - kucuk orneklemin dogal bir sonucu.)

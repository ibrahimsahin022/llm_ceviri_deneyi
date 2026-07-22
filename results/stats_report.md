# Istatistiksel Guc ve Guven Araligi Raporu (Faz 2)

Bootstrap/Monte Carlo yontemleri sabit seed (42) ile calisir; aynı veri uzerinde tekrar calistirildiginda ayni sayilari uretir.

## EA Bootstrap %95 Guven Araligi

| Kosul | EA (nokta) | %95 GA |
|---|---|---|
| Round 1 - dogrudan, debug | %70.91 | [%58.18, %81.82] |
| Round 1 - dogrudan, release | %74.55 | [%63.59, %85.45] |
| Round 2 - iyilestirilmis, debug | %100.00 | [%100.00, %100.00] |

## Mann-Whitney U (LoC: PASS vs FAIL) - Etki Buyuklugu ve Guc

- n(FAIL)=16, n(PASS)=39
- U=250.0, p=0.2541
- Rank-biserial korelasyon (etki buyuklugu) r=0.199 (kucuk etki)
- Bootstrap-tabanli gerceklesen guc (achieved power, alpha=0.05, 5000 tekrar): **%21.1**
  (Yorum: gozlemlenen tam bu etki buyuklugunde ve bu n ile, testin tekrar tekrar uygulansaydi ne siklikta anlamli cikacagini gosterir. Dusuk guc, 'anlamli fark yok' sonucunun bir Tip II hatasi olabilecegi anlamina gelir - kesin bir 'iliski yoktur' iddiasi degildir.)

## Fisher Kesin Testi (Isaretci Kullanimi vs PASS/FAIL) - Odds Orani GA

- Tablo (pointer/non-pointer x PASS/FAIL): [[22, 6], [17, 10]]
- Odds orani=2.16, p=0.2448
- Odds orani %95 guven araligi (log-yaklasik): [0.65, 7.12]
  (Guven araliginin 1.0'i icermesi, iliskinin istatistiksel olarak anlamli olmadigini dogrular; aralik cok genistir - kucuk orneklemin dogal bir sonucu.)

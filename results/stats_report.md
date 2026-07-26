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
- Bootstrap-tabanli gerceklesen guc (achieved power, alpha=0.05, 5000 tekrar): **%15.0** (yalnizca betimsel; post-hoc guc p-degerinin tekduze bir donusumudur ve p'nin otesinde bagimsiz bilgi tasimaz - Hoenig & Heisey 2001. Asagidaki duyarlilik analizine bakiniz.)
- **Duyarlilik analizi (onerilen, post-hoc guc yerine):** n(FAIL)=17, n(PASS)=40, alpha=0.05 ile %80 guçte saptanabilecek en kucuk etki buyuklugu, anlamliligi olcmek icin kullanilan AYNI Mann-Whitney U istatistiginden ampirik rank-biserial formuluyle (r=1-2U/(n1*n2), normal/AUC yaklasik donusumu degil) hesaplandiginda rank-biserial |r|≈0.46'dir (LoC olceginde ≈77 satirlik bir ortalama farka denk gelir, pooled sigma=89.5). Gozlemlenen r=0.156 bu esigin belirgin altindadir - veri seti bu buyuklukte kucuk-orta etkileri saptayacak guce sahip degildir; 'anlamli fark yok' sonucu bu nedenle kesin bir iliskisizlik kaniti degil, dusuk guçle tutarli bir gozlem olarak okunmalidir.

## Betimsel Kod Ozellikleri (PASS vs FAIL, Tablo VII)

Olcum tanimlari (tekrarlanabilirlik icin): isaretci kullanimi = kaynakta `->` VEYA `*isim` bicimli bir isaretci degisken kullanimi (regex: `r"\*\s*[a-zA-Z_]\w*\s*[,;)\[=]"`); malloc/calloc = `malloc(`/`calloc(`/`realloc(` cagrisi; string fonksiyonu = strlen, strcmp, strncmp, strcpy, strncpy, strcat, strncat, strtok, fgets, strchr, strrchr, strstr, sprintf, snprintf, strdup fonksiyonlarindan en az birinin cagrisi.

| Ozellik | PASS(n=40) | FAIL(n=17) |
|---|---|---|
| Ortalama LoC | 67.0 | 59.8 |
| Medyan LoC | 34.5 | 27.0 |
| Isaretci kullanimi | %57.5 | %41.2 |
| malloc/calloc | %22.5 | %0.0 |
| String fonksiyonu | %37.5 | %47.1 |

## Fisher Kesin Testi (Isaretci Kullanimi vs PASS/FAIL) - Odds Orani GA

- Tablo (pointer/non-pointer x PASS/FAIL): [[23, 7], [17, 10]]
- Odds orani=1.93, p=0.3851
- Odds orani %95 guven araligi (log-yaklasik): [0.61, 6.11]
  (Guven araliginin 1.0'i icermesi, iliskinin istatistiksel olarak anlamli olmadigini dogrular; aralik cok genistir - kucuk orneklemin dogal bir sonucu.)

## McNemar Testi (Claude vs Gemini, eslestirilmis karsilastirma)

- Ortak degerlendirilen ornek sayisi: 57
- Ikisi de PASS: 37 | Ikisi de FAIL: 3
- Yalnizca Claude FAIL (Gemini PASS): 14
- Yalnizca Gemini FAIL (Claude PASS): 3
- McNemar kesin (binom-tabanli) iki-yonlu p=0.0127
  (Iki modelin genel EA farkinin istatistiksel olarak anlamli olup olmadigini, eslesmis/paired tasarima uygun bicimde test eder - bagimsiz iki orneklem testi (ör. ki-kare) burada uygun degildir cunku iki model AYNI 57 program uzerinde olculmustur. Anlamli bir genel fark, model×kategori etkilesiminin var olmadigi anlamina gelmez - bkz. Tablo VI.)

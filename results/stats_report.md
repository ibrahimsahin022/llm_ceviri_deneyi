# Istatistiksel Guc ve Guven Araligi Raporu (Faz 2)

Bootstrap/Monte Carlo yontemleri sabit seed (42) ile calisir; aynı veri uzerinde tekrar calistirildiginda ayni sayilari uretir.

## EA Bootstrap %95 Guven Araligi

| Kosul | EA (nokta) | %95 GA |
|---|---|---|
| Round 1 - dogrudan, debug | %70.77 | [%63.08, %78.46] |
| Round 1 - dogrudan, release | %74.62 | [%66.92, %82.31] |
| Round 2 - iyilestirilmis, debug | %100.00 | [%100.00, %100.00] |

## Mann-Whitney U (LoC: PASS vs FAIL) - Etki Buyuklugu ve Guc

- n(FAIL)=38, n(PASS)=92
- U=924.0, p=0.0000
- Rank-biserial korelasyon (etki buyuklugu) r=0.471 (orta etki)
- Bootstrap-tabanli gerceklesen guc (achieved power, alpha=0.05, 5000 tekrar): **%98.9** (yalnizca betimsel; post-hoc guc p-degerinin tekduze bir donusumudur ve p'nin otesinde bagimsiz bilgi tasimaz - Hoenig & Heisey 2001. Asagidaki duyarlilik analizine bakiniz.)
- **Duyarlilik analizi (onerilen, post-hoc guc yerine):** n(FAIL)=38, n(PASS)=92, alpha=0.05 ile %80 guçte saptanabilecek en kucuk etki buyuklugu, anlamliligi olcmek icin kullanilan AYNI Mann-Whitney U istatistiginden ampirik rank-biserial formuluyle (r=1-2U/(n1*n2), normal/AUC yaklasik donusumu degil) hesaplandiginda rank-biserial |r|≈0.31'dir (LoC olceginde ≈36 satirlik bir ortalama farka denk gelir, pooled sigma=64.3). Gozlemlenen r=0.471 bu esigin uzerindedir - veri seti bu buyuklukteki etkiyi saptayacak guce sahiptir; anlamli fark bulgusu dusuk-guc kaynakli bir yanilgi olarak okunmamalidir.

## Betimsel Kod Ozellikleri (PASS vs FAIL, Tablo VII)

Olcum tanimlari (tekrarlanabilirlik icin): isaretci kullanimi = kaynakta `->` VEYA `*isim` bicimli bir isaretci degisken kullanimi (regex: `r"\*\s*[a-zA-Z_]\w*\s*[,;)\[=]"`); malloc/calloc = `malloc(`/`calloc(`/`realloc(` cagrisi; string fonksiyonu = strlen, strcmp, strncmp, strcpy, strncpy, strcat, strncat, strtok, fgets, strchr, strrchr, strstr, sprintf, snprintf, strdup fonksiyonlarindan en az birinin cagrisi.

| Ozellik | PASS(n=92) | FAIL(n=38) |
|---|---|---|
| Ortalama LoC | 64.9 | 43.3 |
| Medyan LoC | 53.0 | 25.0 |
| Isaretci kullanimi | %75.0 | %34.2 |
| malloc/calloc | %17.4 | %0.0 |
| String fonksiyonu | %51.1 | %42.1 |

## Fisher Kesin Testi (Isaretci Kullanimi vs PASS/FAIL) - Odds Orani GA

- Tablo (pointer/non-pointer x PASS/FAIL): [[69, 13], [23, 25]]
- Odds orani=5.77, p=0.0000
- Odds orani %95 guven araligi (log-yaklasik): [2.54, 13.09]
  (Guven araliginin 1.0'i icermesi, iliskinin istatistiksel olarak anlamli olmadigini dogrular; aralik cok genistir - kucuk orneklemin dogal bir sonucu.)

## McNemar Testi (Claude vs Gemini, eslestirilmis karsilastirma)

- Ortak degerlendirilen ornek sayisi: 130
- Ikisi de PASS: 86 | Ikisi de FAIL: 8
- Yalnizca Claude FAIL (Gemini PASS): 30
- Yalnizca Gemini FAIL (Claude PASS): 6
- McNemar kesin (binom-tabanli) iki-yonlu p=0.0001
  (Iki modelin genel EA farkinin istatistiksel olarak anlamli olup olmadigini, eslesmis/paired tasarima uygun bicimde test eder - bagimsiz iki orneklem testi (ör. ki-kare) burada uygun degildir cunku iki model AYNI 130 program uzerinde olculmustur. Anlamli bir genel fark, model×kategori etkilesiminin var olmadigi anlamina gelmez - bkz. Tablo VI.)

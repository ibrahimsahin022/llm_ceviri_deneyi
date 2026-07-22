# Hakem Zayıflıklarını Giderme — Değişiklik Günlüğü

Bu dosya, `C:\Users\ibrah\.claude\plans\fluttering-wiggling-corbato.md` planındaki
5 fazın her birinin sonunda güncellenir. Her girdi üç kısımdan oluşur:
(a) değişen/eklenen dosyalar, (b) o fazda gerçekten ölçülen sayılar, (c) makalenin
ilgili bölümüne eklenmesi önerilen taslak metin.

---

## Faz 1 — Çoklu Model Desteği

### (a) Değişen/eklenen dosyalar
- **Yeni:** `harness/translators/__init__.py` — `Translator` soyut arayüzü,
  `TranslationResult` veri sınıfı, sabit `PROMPT_TEMPLATE`, sabit sampling
  parametreleri (`temperature=0.2`, `top_p=1.0`).
- **Yeni:** `harness/translators/gemini_translator.py`,
  `harness/translators/openai_translator.py`,
  `harness/translators/deepseek_translator.py`.
- **Yeni:** `harness/generate_translations.py` (CLI, `--model`, `--dry-run`,
  `--only`, `--limit`, `--sleep`), `harness/compare_models.py`.
- **Yeni:** `.env.example`, `.gitignore`.
- **Değişti:** `harness/run_experiment.py` — yalnızca `--skip-missing` bayrağı
  eklendi (rust dosyası olmayan örnekleri CE saymak yerine tamamen atlar;
  varsayılan davranış değişmedi, geriye dönük tam uyumlu).
- **Yeni:** `results/manifest_gemini.json` (gerçek, 19 kayıt),
  `results/manifest_gpt4o.json`, `results/manifest_deepseek.json` (48'er kayıt,
  hepsi `dry_run: true` — API çağrılmadı), `results/results_gemini.json/csv`,
  `results/model_comparison.md`.

### (b) Gerçekten ölçülen sayılar
- **Gemini (`gemini-flash-latest`, gerçek API çağrısı):** Google AI Studio
  ücretsiz katmanının **günlük kota sınırı** (`GenerateRequestsPerDayPerProjectPerModel-FreeTier`,
  limit=20 istek/gün/model) nedeniyle yalnızca **19/48** örnek çevrildi;
  20. istekten itibaren tüm çağrılar `429 RESOURCE_EXHAUSTED` ile reddedildi
  (ham hata mesajları `results/manifest_gemini.json`'da saklı). Kullanıcı
  talimatıyla mevcut 19 örnek üzerinden kısmi ama gerçek bir ölçüm raporlanıyor;
  hiçbir sayı uydurulmadı.
  - **Gemini EA (n=19, kısmi kapsam): 18/19 = %94.74** (1 FE: `s15_float_avg` —
    Claude'da da aynı kök nedenden [Kategori D, %g biçimlendirme] başarısız
    olan örnekle aynı; ilginç biçimde bağımsız bir model de aynı boşluğa
    düşmüş).
  - Claude Sonnet 5 (referans, aynı 19 örnek alt kümesinde tekrar hesaplanmadı,
    tam veri seti üzerinden mevcut sonuç): **36/48 = %75.00**.
  - **Not:** Bu iki sayı doğrudan karşılaştırılabilir DEĞİLDİR (farklı örneklem
    büyüklüğü/alt kümesi); doğru karşılaştırma için Gemini'nin kalan 29
    örneğinin tamamlanması (kota sıfırlandıktan sonra, yarından itibaren günde
    ~20 istekle kademeli olarak) gerekir.
  - OpenAI (GPT-4o) ve DeepSeek: **API anahtarı yok, hiç çağrılmadı.**
    Yalnızca `--dry-run` ile 48 örneğin tamamı için istem (prompt) inşası
    doğrulandı (`results/manifest_gpt4o.json`, `results/manifest_deepseek.json`
    — `dry_run: true` olarak işaretli, gerçek çeviri veya sonuç İÇERMEZ).

### (c) Makaleye önerilen taslak metin

**§3.2 Çeviri Süreci'ne eklenecek paragraf:**
> Çalışmanın ilk sürümünde yalnızca Claude Sonnet 5 kullanılmıştı. Hakem
> geri bildirimi doğrultusunda, model-bağımsız bir çeviri altyapısı
> (`harness/translators/`) kuruldu ve Google Gemini (`gemini-flash-latest`)
> için gerçek, otomatik API çağrılarıyla kısmi bir tekrar ölçüm yapıldı
> (19/48 örnek, Google AI Studio ücretsiz katmanının günlük kota sınırı
> nedeniyle). Bu kısmi örneklemde Gemini EA = %94.74 (18/19) ölçülmüştür;
> tek başarısızlık, Claude'da da aynı kök nedenden (çıktı biçimlendirme
> semantiği, §4.4.D) kaynaklanmıştır — bu, en azından bu tek örnekte, ilgili
> semantik boşluğun modele özgü olmayabileceğine dair ön bir işarettir. OpenAI
> ve DeepSeek adaptörleri de aynı altyapıyla yazılmış ve istem inşası
> doğrulanmıştır, ancak yazarların bu modeller için API erişimi olmadığından
> gerçek sonuç ölçülememiştir.

**§6 Geçerlilik Tehditleri → "Dış Geçerlilik" alt bölümüne eklenecek not:**
> Çoklu-model karşılaştırması bu sürümde tamamlanmamıştır: Gemini için
> yalnızca 19/48 örnek (günlük API kotası nedeniyle), GPT-4o ve DeepSeek için
> hiç gerçek sonuç yoktur. Dolayısıyla "bulgular yalnızca Claude Sonnet 5'e
> özgüdür" sınırlaması büyük ölçüde geçerliliğini korumaktadır; kısmi Gemini
> verisi yalnızca bir ön işarettir, kapsamlı bir karşılaştırma değildir.

### Sonraki adım
Google AI Studio kotası sıfırlandıkça (`python harness/generate_translations.py
--model gemini --only <eksik-id'ler> --sleep 4`) kalan 29 örnek tamamlanabilir;
tamamlandığında `python harness/compare_models.py` yeniden çalıştırılıp bu
bölüm tam n=48 sayılarıyla güncellenecektir.

---

## Faz 2 — Örneklem Gücü ve İstatistik

### (a) Değişen/eklenen dosyalar
- **Yeni:** `harness/stats_report.py` — bootstrap %95 GA (EA için), Mann-Whitney
  rank-biserial etki büyüklüğü + bootstrap-tabanlı gerçekleşen güç (achieved
  power), Fisher odds oranı için log-yaklaşık %95 GA. Çıktı: `results/stats_report.md`.
  Sabit seed (42) ile tam tekrarlanabilir.
- **Değişti:** `harness/make_figures.py` — kök-neden listesi (Şekil 5) güncellendi
  (C/F/G/H kategorilerinin her biri artık 2 bağımsız örnekle temsil ediliyor);
  yeni **Şekil 4b** eklendi (`fig4b_bootstrap_ci.png` — üç koşulda EA + bootstrap
  %95 GA çubukları).
- **Yeni C örnekleri (5):** `samples_c/s49_negative_byte_count.c` (Kategori C,
  2. örnek), `s50_id_generator.c` (Kategori E, 2. örnek), `s51_long_clamp.c`
  (Kategori F, 2. örnek), `s52_window_sum.c` (Kategori G, 2. örnek),
  `s53_tax_bracket.c` (Kategori H, 2. örnek) + `tests/s49.../`..`tests/s53.../`.
- **Yeni çeviriler:** `translations_rust/s49-s53*.rs` (Round 1, zero-shot),
  `translations_rust_refined/s49-s53*.rs` (Round 2, düzeltilmiş; `s50` zaten
  Round 1'de PASS olduğu için değişmeden kopyalandı).

### (b) Gerçekten ölçülen sayılar
Veri seti **n=48 → n=53**. Gerçek harness koşumu (Round 1, debug):

| Kosul | EA (n=53) | Bootstrap %95 GA |
|---|---|---|
| Round 1 — dogrudan, debug | %69.81 (37/53) | [%56.60, %81.13] |
| Round 1 — dogrudan, release | %73.58 (39/53) | [%60.38, %84.91] |
| Round 2 — iyilestirilmis, debug | %100.00 (53/53) | [%100.00, %100.00] |

5 yeni örnekten **4'ü başarısız oldu, 1'i (s50_id_generator) ilk seferde
geçti** — bu, kategori E'nin (global mutable durum → `static mut`) her
zaman başarısız olmadığını, modelin bazı gerçekleşmelerde doğru `unsafe`
sarmalamayı yapabildiğini gösteren dürüst, tek yönlü olmayan bir bulgudur.
Başarısız 4 örnek (s49, s51, s52, s53), ilgili kategorinin (C, F, G, H)
zaten bilinen kök nedenini bağımsız bir ikinci örnekte doğruladı — hiçbiri
yeni bir kök neden ortaya çıkarmadı (beklenen: bunlar zaten bilinen
boşlukları hedefleyerek tasarlandı).

**Mann-Whitney U (LoC, PASS vs FAIL):** U=246.0, p=0.3371 (n=45 iken
p=0.169'du). Rank-biserial etki büyüklüğü r=0.169 (küçük). **Bootstrap
gerçekleşen güç: %15.6.** Bu, hakemin "örneklem küçük, güç düşük" eleştirisini
doğrudan **doğrulayan** nicel bir kanıttır — n'i 48'den 53'e büyütmek gücü
ARTIRMADI, tam tersine biraz düşürdü, çünkü yeni eklenen 4 başarısız örneğin
LoC'si (27-33 satır) PASS grubunun LoC aralığıyla büyük ölçüde örtüşüyor
(zaten zayıf olan LoC↔başarı ilişkisini daha da zayıflattı). Bu, "n'i büyütmek
otomatik olarak gücü artırır" varsayımının burada geçerli olmadığını gösteren,
literatürde de bilinen ("etki büyüklüğü sabit kalırsa, örneklem türüne bağlı
olarak güç öngörülemez şekilde değişebilir") dürüst bir bulgudur.

**Fisher (pointer kullanımı):** odds=1.96, p=0.372, %95 GA=[0.59, 6.52] (n=45
iken odds=2.50, p=0.318 idi — GA hâlâ 1.0'i genişçe kapsıyor, anlamlılık
değişmedi).

### (c) Makaleye önerilen taslak metin

**§4.3 Kod Uzunluğu ile Başarı İlişkisi'ne eklenecek paragraf:**
> Hakem geri bildirimi doğrultusunda, mevcut sekiz kök-neden kategorisinden
> dördü (C, F, G, H — önceden yalnızca birer örnekle temsil ediliyordu) için
> bağımsız birer ikinci örnek eklenmiş (n=48→53) ve istatistiksel güç
> doğrudan ölçülmüştür. Mann-Whitney U testinin bootstrap-tabanlı
> gerçekleşen gücü yalnızca %15.6'dır (n=53) — bu, "kod uzunluğu ile başarı
> arasında anlamlı ilişki yoktur" bulgusunun, ilişkinin gerçekten
> bulunmamasından mı yoksa testin bu örneklem büyüklüğünde yetersiz güçte
> olmasından mı kaynaklandığını ayırt edemediğimizi doğrudan, nicel olarak
> gösterir. Ayrıca örneklem büyüdükçe gücün arttığı değil, hafifçe azaldığı
> gözlenmiştir (%21.7→%15.6) — bu, küçük ölçekli veri seti genişletmelerinin
> istatistiksel gücü otomatik olarak iyileştirmediğini, etki büyüklüğünün
> kendisinin de örneklemle birlikte değiştiğini göstermektedir.

**§6 Geçerlilik Tehditleri → "İstatistiksel Geçerlilik" alt bölümüne eklenecek not:**
> Bootstrap tabanlı güç analizi (`harness/stats_report.py`, sabit seed=42,
> 5000 tekrar), Mann-Whitney testinin bu veri setinde yalnızca %15.6
> gerçekleşen güce sahip olduğunu göstermektedir — konvansiyonel %80 güç
> eşiğinin çok altında. Bu nedenle "anlamlı ilişki gözlenmemiştir" ifadesi,
> "ilişki yoktur" biçiminde okunmamalıdır; mevcut n ile bir Tip II hatası
> (gerçek bir etkiyi kaçırma) olasılığı yüksektir.

### Kalan (bu fazda tamamlanmayan)
Plan, istatistiksel gücü belirgin biçimde artıracak ölçüde büyük bir n
artışı hedeflemiyordu (5 hedefli örnek); gerçek anlamda yeterli güce (%80)
ulaşmak için FAIL grubunda muhtemelen onlarca ek bağımsız örnek gerekir —
bu, gelecekteki bir faz/çalışma için not edilmiştir.

---

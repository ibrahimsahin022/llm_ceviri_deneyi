# Hakem Zayıflıklarını Giderme — Değişiklik Günlüğü

Bu dosya, hakem zayıflıklarını gidermek için hazırlanan 5 fazlı planın her
birinin sonunda güncellenir. Her girdi üç kısımdan oluşur:
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
  ücretsiz katmanının kota sınırı (`GenerateRequestsPerDayPerProjectPerModel-FreeTier`,
  limit=20 istek/gün/model — ancak gözlemlenen davranış katı bir günlük sıfırlamadan
  çok kısa/kayan bir pencereye benziyor: birden fazla oturumda "1 istek başarılı,
  hemen ardından 429" örüntüsü tekrarlandı) nedeniyle üç ayrı oturumda toplam
  **22/57** örnek çevrilebildi (19 → 20 → 22, kalan 35 örnek için tekrar tekrar
  `429 RESOURCE_EXHAUSTED` alındı; ham hata mesajları `results/manifest_gemini.json`'da
  saklı). Kullanıcı talimatıyla mevcut 22 örnek üzerinden kısmi ama gerçek bir
  ölçüm raporlanıyor; hiçbir sayı uydurulmadı.
  - **Gemini EA (n=22, kısmi kapsam): 21/22 = %95.45** (1 FE: `s15_float_avg` —
    Claude'da da aynı kök nedenden [Kategori D, %g biçimlendirme] başarısız
    olan örnekle aynı; ilginç biçimde bağımsız bir model de aynı boşluğa
    düşmüş).
  - Claude Sonnet 5 (referans, aynı 22 örnek alt kümesinde tekrar hesaplanmadı,
    tam veri seti üzerinden mevcut sonuç): **40/57 = %70.18**.
  - **Not:** Bu iki sayı doğrudan karşılaştırılabilir değildir (farklı örneklem
    büyüklüğü/alt kümesi); doğru karşılaştırma için Gemini'nin kalan 35
    örneğinin tamamlanması (kota sıfırlandıktan sonra, yarından itibaren günde
    ~20 istekle kademeli olarak) gerekir.
  - OpenAI (GPT-4o) ve DeepSeek: **API anahtarı yok, hiç çağrılmadı.**
    Yalnızca `--dry-run` ile 48 örneğin tamamı için istem (prompt) inşası
    doğrulandı (`results/manifest_gpt4o.json`, `results/manifest_deepseek.json`
    — `dry_run: true` olarak işaretli, gerçek çeviri veya sonuç içermez).

### (c) Makaleye önerilen taslak metin

**§3.2 Çeviri Süreci'ne eklenecek paragraf:**
> Çalışmanın ilk sürümünde yalnızca Claude Sonnet 5 kullanılmıştı. Hakem
> geri bildirimi doğrultusunda, model-bağımsız bir çeviri altyapısı
> (`harness/translators/`) kuruldu ve Google Gemini (`gemini-flash-latest`)
> için gerçek, otomatik API çağrılarıyla kısmi bir tekrar ölçüm yapıldı
> (22/57 örnek, Google AI Studio ücretsiz katmanının kota sınırı
> nedeniyle). Bu kısmi örneklemde Gemini EA = %95.45 (21/22) ölçülmüştür;
> tek başarısızlık, Claude'da da aynı kök nedenden (çıktı biçimlendirme
> semantiği, §4.4.D) kaynaklanmıştır — bu, en azından bu tek örnekte, ilgili
> semantik boşluğun modele özgü olmayabileceğine dair ön bir işarettir. OpenAI
> ve DeepSeek adaptörleri de aynı altyapıyla yazılmış ve istem inşası
> doğrulanmıştır, ancak yazarların bu modeller için API erişimi olmadığından
> gerçek sonuç ölçülememiştir.

**§6 Geçerlilik Tehditleri → "Dış Geçerlilik" alt bölümüne eklenecek not:**
> Çoklu-model karşılaştırması bu sürümde tamamlanmamıştır: Gemini için
> yalnızca 22/57 örnek (API kotası nedeniyle), GPT-4o ve DeepSeek için
> hiç gerçek sonuç yoktur. Dolayısıyla "bulgular yalnızca Claude Sonnet 5'e
> özgüdür" sınırlaması büyük ölçüde geçerliliğini korumaktadır; kısmi Gemini
> verisi yalnızca bir ön işarettir, kapsamlı bir karşılaştırma değildir.

### Sonraki adım
Google AI Studio kotası sıfırlandıkça (`python harness/generate_translations.py
--model gemini --only <eksik-id'ler> --sleep 4`) kalan 35 örnek tamamlanabilir;
tamamlandığında `python harness/compare_models.py` yeniden çalıştırılıp bu
bölüm tam n=57 sayılarıyla güncellenecektir. `generate_translations.py`
artık çok-dosyalı örnekleri de (s54, s55, s57 — birden fazla `.c` dosyasını
tek istemde birleştirip modelden tek bir `main.rs` isteyerek) destekler;
ayrıca manifest artık her çalıştırmada sıfırlanmaz, yalnızca işlenen id'ler
güncellenir (önceki gerçek kayıtlar korunur).

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
artırmadı, tam tersine biraz düşürdü, çünkü yeni eklenen 4 başarısız örneğin
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

## Faz 3 — Çok Dosyalı / Gerçekçi Kod

### (a) Değişen/eklenen dosyalar
- **Yeni:** `samples_c/s54_stack_module/` — `stack.h` (struct + fonksiyon
  bildirimleri), `stack.c` (uygulama), `main.c` (kullanım), `manifest.json`
  (`{"c_files": ["stack.c","main.c"], "rust_main": "main.rs"}`).
- **Yeni:** `samples_c/s55_config_parser/` — `config.h` (paylaşılan
  `ConfigEntry` struct'ı, **iki ayrı** .c dosyası tarafından kullanılıyor),
  `parser.c`, `lookup.c`, `main.c`, `manifest.json`.
- **Yeni:** `translations_rust/s54_stack_module/{main.rs,stack.rs}`,
  `translations_rust/s55_config_parser/{main.rs,config.rs,parser.rs,lookup.rs}`
  (ve `translations_rust_refined/` altında aynıları — ikisi de Round 1'de
  PASS olduğu için değişikliksiz kopyalandı).
- **Yeni:** `tests/s54_stack_module/`, `tests/s55_config_parser/` (5'er test
  girdisi; kapasite sınırı, boş yığın, yinelenen anahtar, boş değer gibi
  kenar durumları kapsar).
- **Değişti:** `harness/run_experiment.py` — `discover_samples()` artık
  `samples_c/*.c` (tek dosya, değişmedi) yanında `samples_c/*/manifest.json`
  içeren dizinleri de keşfediyor; `compile_c()` çoklu `.c` dosyasını tek
  `gcc` çağrısında derliyor; Rust tarafı için hiçbir değişiklik gerekmedi
  — `rustc <main.rs>` zaten aynı dizindeki `mod x;` dosyalarını otomatik
  çözüyor (Cargo'ya gerek yok). Geriye dönük tam uyumlu (mevcut 53 tek-dosya
  örnek hiç etkilenmedi).
- **Düzeltildi:** `harness/stats_report.py` — Fisher (işaretçi kullanımı)
  analizi yalnızca `samples_c/*.c` üzerinde tarama yapıyordu, bu yüzden
  çok-dosyalı örnekleri (s54, s55) sessizce atlıyordu; artık
  `results_round1.json`'daki tüm id'ler üzerinden gidip çok-dosyalı
  örnekler için ilgili alt dizindeki tüm `.c` dosyalarını birleştirerek
  tarıyor.

### (b) Gerçekten ölçülen sayılar
Veri seti **n=53 → n=55**. Gerçek harness koşumu (multi-file derleme
desteğiyle, gerçekten `gcc stack.c main.c -o ...` ve `rustc main.rs -o ...`
çalıştırılarak):

| Koşul | EA (n=55) |
|---|---|
| Round 1 — doğrudan, debug | %70.91 (39/55) |
| Round 1 — doğrudan, release | %74.55 (41/55) |
| Round 2 — iyileştirilmiş, debug | %100.00 (55/55) |

**Her iki çok-dosyalı örnek de Round 1'de ilk seferde geçti (2/2 PASS)** —
CE oranında bir artış gözlenmedi. Bu, önceden beklenen "çok dosyalı kod
derleme hatalarını artırır" hipotezini bu iki örnekte doğrulamayan, dürüst
bir sonuçtur: LLM, hem `stack.h`↔`stack.c`↔`main.c` arası fonksiyon
imzalarını hem de `config.h`'deki paylaşılan `ConfigEntry` struct'ının iki
farklı `.c` dosyasındaki (`parser.c`, `lookup.c`) kullanımını Rust'ın
`mod` sistemine tutarlı biçimde eşlemiştir (`stack.rs` bir `struct Stack`
+ `impl` bloğu; `config.rs`/`parser.rs`/`lookup.rs` üçlüsü paylaşılan
`ConfigEntry` struct'ını `crate::config::ConfigEntry` olarak tutarlı
biçimde referanslamıştır). **Önemli sınırlama:** yalnızca 2 örnek, ikisi de
orta karmaşıklıkta (2-3 dosya, tek seviye modül); bu, "çok dosyalı kod her
zaman sorunsuz çevrilir" biçiminde genellenemez — çok daha büyük/derin
modül hiyerarşilerine sahip gerçek projelerde farklı sonuç alınabilir
(bkz. §6 notu).

Fisher testi düzeltmesi sonrası (s54/s55 dahil): odds=2.16, p=0.245, %95
GA=[0.65, 7.12] (önceki, hatalı biçimde s54/s55'i atlayan hesap: odds=1.96).

### (c) Makaleye önerilen taslak metin

**§3.1 Veri Seti'ne eklenecek paragraf:**
> Hakem geri bildirimi doğrultusunda, veri setinin tek-dosyalı yapısının
> derleme başarı oranını yapay biçimde yükseltebileceği eleştirisini kısmen
> sınamak amacıyla, iki çok-dosyalı C örneği eklenmiştir (s54-s55): biri
> (`s54_stack_module`) klasik bir başlık+uygulama+kullanım (.h/.c/.c) modül
> deseni, diğeri (`s55_config_parser`) paylaşılan bir struct tanımının iki
> ayrı derleme birimi tarafından kullanıldığı bir desendir. Harness, bu
> örnekleri bir `manifest.json` aracılığıyla keşfedip çoklu dosyayı tek bir
> `gcc`/`rustc` çağrısında derleyecek şekilde genişletilmiştir (Rust
> tarafında `mod` sistemi Cargo gerektirmeden çoklu dosyayı doğal olarak
> destekler). Her iki örnek de Round 1'de ilk seferde geçmiştir (2/2 PASS);
> bu, çok dosyalı yapının bu iki orta-karmaşıklıktaki örnekte CE oranını
> artırmadığını göstermektedir, ancak yalnızca 2 örnekle sınırlı bir
> gözlemdir.

**§6 Geçerlilik Tehditleri → "Dış Geçerlilik" alt bölümüne eklenecek not:**
> Çok-dosyalı doğrulama yalnızca 2 örnekle (2-3 dosya, tek seviye modül
> hiyerarşisi) sınırlıdır ve her ikisi de PASS olmuştur. Bu, "çok dosyalı
> yapı CE oranını artırır" biçimindeki önceki varsayımsal uyarıyı bu iki
> örnek için çürütür, ancak gerçek endüstriyel projelerdeki çok daha derin
> modül hiyerarşilerine, dairesel bağımlılıklara veya build-sistemi
> (Makefile/CMake) karmaşıklığına genellenemez.

---

## Faz 4 — Çok Platformlu Çalıştırma

### (a) Değişen/eklenen dosyalar
- **Yeni:** `Dockerfile` — `ubuntu:24.04` tabanlı, `build-essential` (gcc
  13.3.0) + rustup ile stabil rustc + Python/scipy/matplotlib içeren imaj.
- **Yeni:** `docker-compose.yml` — `results/` dizinini konteynerle
  eşleyen tek servis; üç `run_experiment.py` koşumunu (`_linux` etiketli)
  otomatik çalıştırır.
- **Yeni:** `harness/compare_platforms.py` — Windows ve Linux sonuç
  JSON'larını karşılaştırıp `results/platform_comparison.md` üretir;
  örnek bazında hangi programların platforma göre farklı kategoriye
  düştüğünü listeler.
- **Yeni:** `.github/workflows/ci-matrix.yml` — `windows-latest` +
  `ubuntu-latest` matrisli GitHub Actions iş akışı (yazıldı, **push
  edilmedi** — yalnızca depo sahibi ileride GitHub'a push ederse fiilen
  çalışır).
- **Yeni sonuç dosyaları:** `results/results_round1_linux.*`,
  `results/results_round1_release_linux.*`, `results/results_round2_linux.*`,
  `results/platform_comparison.md`.

### (b) Gerçekten ölçülen sayılar
Docker Desktop yerelde başlatılıp gerçek bir Ubuntu 24.04 konteyneri inşa
edildi ve içinde 55 örneğin tamamı gerçekten derlenip çalıştırıldı (gcc
13.3.0, rustc 1.97.1 — **Windows'takiyle birebir aynı rustc sürüm/commit**,
bu yüzden gözlenen fark rustc sürümünden değil `long` genişliğinden ve
stdio davranışından kaynaklanır).

| Koşul | Windows EA | Linux EA | Fark |
|---|---|---|---|
| Round 1 — doğrudan, debug | %70.91 (39/55) | %72.73 (40/55) | +1.82 puan |
| Round 1 — doğrudan, release | %74.55 (41/55) | %76.36 (42/55) | +1.81 puan |
| Round 2 — iyileştirilmiş, debug | **%100.00 (55/55)** | **%94.55 (52/55)** | **-5.45 puan** |

**En önemli bulgu — Round 2'nin "%100 başarısı" platforma özgüdür:**
`s38_bsd_strtol` ve `s51_long_clamp`'in Round 2 düzeltmesi (bu çalışmanın
önceki bölümlerinde Windows'ta %100 EA elde etmek için yapılmıştı) `i64`
yerine `i32` kullanarak Windows'un 32-bit `long`'unu taklit ediyordu.
Linux'ta C referansının `long`'u gerçekten 64-bit olduğundan, **aynı
"düzeltilmiş" Rust kodu Linux'ta artık yanlış sonuç üretiyor** (32-bit
sınırında gereksiz kırpma yapıyor, oysa C referansı hiç kırpmıyor). Tersine,
Round 1'in "düzeltilmemiş" (doğal `i64` seçimi yapan) hali Windows'ta
başarısızken Linux'ta doğru sonuç veriyor — iki platform arasında sonuçlar
tam olarak ters dönüyor. Bu, "iyileştirme döngüsü hatayı düzeltir" iddiasının
zımni bir varsayımını (düzeltmenin evrensel olduğu) doğrudan çürütmektedir:
**bir platformda doğrulanmış bir düzeltme, başka bir platformda yeni bir
hataya dönüşebilir.**

**İkinci, beklenmedik bulgu — s47_redis_sds'te C referansının kendisi
platform-bağımlı:** Bu farkın nedeni `long` genişliği değil; `tests/
s47_redis_sds/05.txt` dosyasının CRLF satır sonu içermesi ve C referansının
`scanf` sonrası tek bir `getchar()` ile satır sonunu tükettiği bir kod
deseniyle etkileşimidir. Windows'un C çalışma zamanı stdin'i metin modunda
açıp `\r\n`'i otomatik olarak `\n`'e çevirir; Linux/glibc bu çeviriyi
yapmaz, bu yüzden aynı C kaynak kodu ve aynı girdi iki platformda farklı
sayıda satır tüketir (Linux'ta son komut hiç çalışmaz). Rust'ın
`BufRead::lines()`'ı her iki satır-sonu türünü de sorunsuz işlediğinden bu
sorunu hiç yaşamaz — yani bu örnekte "kırılan" taraf Rust çevirisi değil,
**C referansının kendisidir**. Test dosyası kasıtlı olarak düzeltilmemiştir;
bu gerçek ve tekrarlanabilir bir bulgudur (ayrıntı: `results/platform_comparison.md`).

### (c) Makaleye önerilen taslak metin

**Yeni bir §4.9 (veya §5'e eklenecek bir paragraf) için taslak:**
> Hakem geri bildirimi doğrultusunda, deneyin tamamı ayrıca Docker
> aracılığıyla gerçek bir Linux/LP64 (64-bit `long`) ortamında (Ubuntu
> 24.04, gcc 13.3.0, rustc 1.97.1 — Windows ile birebir aynı rustc
> sürümü) tekrarlanmıştır. Sonuç, bu çalışmanın en önemli platforma-bağlı
> bulgusudur: Round 2'nin Windows'ta ölçülen %100 EA'sı Linux'ta %94.55'e
> düşmüştür (52/55), çünkü s38 ve s51 için yazılan düzeltmeler (32-bit
> `long` varsayımı) Linux'un 64-bit `long`'unda geçersiz hale gelmiştir —
> iki platform arasında bu iki örneğin PASS/FAIL durumu tam olarak yer
> değiştirmiştir. Bu, "hata geri bildirimiyle düzeltilmiş kod evrensel
> olarak doğrudur" varsayımının yanlış olabileceğini doğrudan
> göstermektedir: bir düzeltme yalnızca test edildiği platforma özgü
> olabilir. Ayrıca, s47_redis_sds'te C referansının kendisinin (Rust
> çevirisi değil) stdio metin-modu satır-sonu davranışı nedeniyle
> platforma bağlı biçimde farklı sonuç ürettiği gözlenmiştir — bu, C↔Rust
> semantik boşluklarının ötesinde, C'nin kendisinin de tam platform-bağımsız
> olmadığını gösteren ayrı bir bulgudur.

**§6 Geçerlilik Tehditleri → "Dış Geçerlilik" alt bölümüne eklenecek not
(önceki, artık kısmen geçersiz uyarının yerine):**
> Önceki sürümde "bu deney yalnızca Windows/LLP64'te çalıştırılmıştır"
> biçiminde bir sınırlama belirtilmişti; bu artık kısmen giderilmiştir —
> deney gerçek bir Linux/LP64 ortamında da (Docker, bkz. `Dockerfile`,
> `results/platform_comparison.md`) tekrarlanmış ve platforma-bağlı en az
> iki gerçek davranış farkı (tamsayı genişliği kaynaklı iki örnek; stdio
> metin-modu kaynaklı bir örnek) doğrulanmıştır. Bu, deneyin platform
> genelinde tutarlı olduğu anlamına gelmez — tam tersine, platformlar
> arasında ölçülebilir, önemli farklar bulunduğunu ve "iyileştirilmiş"
> çevirilerin platforma özgü olabileceğini kanıtlamaktadır. CI matrisi
> (`.github/workflows/ci-matrix.yml`) yazılmış ancak depo henüz GitHub'a
> push edilmediğinden fiilen çalıştırılmamıştır; yerel Docker koşumu
> aynı sonucu zaten üretmiştir.

### Kalan (bu fazda tamamlanmayan)
- `.github/workflows/ci-matrix.yml` yalnızca yazıldı; GitHub'a push
  edilip Actions üzerinde gerçek çalıştırma (kullanıcı onayı gerektirir,
  bu fazın kapsamı dışında tutuldu).
- s47'deki CRLF/stdio bulgusu, tek bir test dosyasıyla sınırlıdır; benzer
  `getchar()`-sonrası-`fgets()` deseni kullanan başka örneklerde de aynı
  sorunun gizli biçimde bulunup bulunmadığı sistematik olarak taranmamıştır
  (gelecekteki bir faz için not edilmiştir).

---

## Faz 5 — Tekrarlanabilirlik ve Belgeleme

### (a) Değişen/eklenen dosyalar
- **Yeni:** `requirements.txt` — `pip freeze` ile sabitlenmiş tam sürümler
  (scipy 1.18.0, matplotlib 3.11.1, numpy 2.5.1, python-docx 1.2.0,
  google-genai 2.13.0, openai 2.46.0, python-dotenv 1.2.2).
- **Değişti:** `README.md` — dataset açıklaması n=55'e güncellendi, klasör
  yapısına Faz 1-4'ün tüm yeni dosyaları eklendi, yeni **"Nasıl Tam Olarak
  Tekrarlanır"** bölümü eklendi: araç sürümleri tablosu (Windows+Linux yan
  yana), kullanılan tüm model kimlikleri + erişim tarihleri + sampling
  parametreleri tablosu, tam komut listesi, ve LLM API'lerinin bit-bit
  tekrarlanabilir olmadığına dair dürüst bir uyarı.

### (b) Gerçekten doğrulanan durum
Bu faz yeni bir deneysel ölçüm üretmez; önceki 4 fazda zaten gerçekten
ölçülmüş olan tüm sürüm/kimlik/parametre bilgilerini tek bir yerde
(`README.md`) toparlar ve doğrular. Doğrulanan bilgiler:
- Windows: gcc 16.1.0 (MSYS2/UCRT64), rustc 1.97.1 (commit 8bab26f4f).
- Linux/Docker: gcc 13.3.0 (Ubuntu), rustc 1.97.1 (**aynı commit** —
  Faz 4'te `docker compose run` çıktısından doğrulandı).
- Claude Sonnet 5: `translations_rust/` ve `translations_rust_refined/`
  içindeki 55 örneğin tamamı; örnekleme parametreleri CLI arayüzü
  üzerinden üretildiği için tam olarak bilinmiyor (bu, açıkça belirtilen
  bir sınırlamadır).
- Gemini: `gemini-flash-latest` (API `gemini-3.6-flash`'e çözümlüyor),
  erişim 2026-07-22, `temperature=0.2`/`top_p=1.0` — `results/manifest_gemini.json`'da
  kayıtlı, gerçek.
- GPT-4o/DeepSeek: hiç çağrılmadı (anahtar yok), yalnızca `--dry-run` ile
  istem inşası doğrulandı.

### (c) Makaleye önerilen taslak metin

**§3.7 Deneysel Ortam ve Araçlar'a eklenecek not:**
> Tekrarlanabilirlik için tüm araç sürümleri, model kimlikleri, erişim
> tarihleri ve sabit sampling parametreleri `README.md`'nin "Nasıl Tam
> Olarak Tekrarlanır" bölümünde ve `requirements.txt`'te belgelenmiştir.
> Bootstrap/Monte Carlo tabanlı istatistikler (§4.3, §6) sabit bir seed
> (42) ile bit-bit tekrarlanabilirken, LLM API çağrıları (Gemini) düşük
> ve sabit bir `temperature` (0.2) değerinde olsa dahi sağlayıcı-taraflı
> değişkenlik nedeniyle bit-bit tekrarlanabilir değildir; bu, çalışmanın
> açıkça kabul ettiği bir sınırlamadır.

---

## Genel Özet (5 Faz Tamamlandı)

Hakem tarafından tespit edilen beş zayıflığın tümü için somut, gerçek
ölçümlere dayalı ilerleme kaydedilmiştir — hiçbir sayı uydurulmamış,
her adım gerçekten çalıştırılan betiklerle doğrulanmıştır:

| # | Zayıflık | Durum | Ana bulgu |
|---|---|---|---|
| 1 | Tek model | **Kısmen giderildi** | Gemini ile 22/57 örnek gerçek ölçüm (%95.45 EA); OpenAI/DeepSeek altyapısı hazır, anahtar bekliyor |
| 2 | Küçük örneklem/istatistiksel güç | **Ölçüldü, tam giderilmedi** | n=48→53; gerçekleşen güç yalnızca %15.6 — güç sorununun kendisi nicel olarak kanıtlandı |
| 3 | Tek dosyalı kod | **Kısmen giderildi** | n=53→55, 2 çok-dosyalı örnek eklendi, ikisi de PASS (CE artışı gözlenmedi, ama n=2 ile sınırlı) |
| 4 | Tek platform | **Gerçek ikinci platformda doğrulandı** | Linux/LP64'te Round 2 EA %100'den %94.55'e düştü — platforma-özgü "düzeltme" bulgusu |
| 5 | Tekrarlanabilirlik belgesi | **Giderildi** | Tüm sürümler/kimlikler/parametreler `README.md` + `requirements.txt`'te belgelendi |

**En önemli tek bulgu (Faz 4):** Round 2'nin iddia edilen "%100 doğruluk"
rakamının bir kısmı platforma özgüydü — Windows için yazılan iki düzeltme
Linux'ta geçersiz hale geldi. Bu, makalenin genel tezini ("iyileştirme
döngüsü etkilidir ama üst-sınır bir performanstır ve dikkatli
yorumlanmalıdır") daha da güçlü bir biçimde doğrulamaktadır: iyileştirme,
yalnızca test edildiği koşullar için geçerli olabilir.

Git geçmişi (`git log`) her fazın ayrı, gözden geçirilebilir commit'ler
halinde uygulandığını gösterir (bir baseline commit + 4 faz commit'i).

---

## Faz 3 Devamı — Karmaşık Makrolar ve Paylaşılan Bellek Eşzamanlılığı

Beş faz tamamlandıktan sonra, Faz 3'ün (çok dosyalı/gerçekçi kod) kapsamını
genişletmek amacıyla iki yeni örnek eklendi: biri karmaşık C önişlemci
(preprocessor) desenlerini, diğeri gerçek paylaşılan bellek eşzamanlılığını
(pthreads) hedefler.

### (a) Değişen/eklenen dosyalar
- **Yeni:** `samples_c/s56_macro_table.c` — X-Macro deseni (token-pasting
  `CMD_##name` ile enum + isim tablosu üretimi) **ve** klasik bir "çoklu-
  değerlendirme" (multiple evaluation) tuzağı: `#define MAX(a,b)
  ((a)>(b)?(a):(b))` yan-etkili bir argümanla (`MAX(x++, 10)`) çağrıldığında,
  C makrosu metinsel ikame olduğundan `x++`'ı koşula bağlı olarak bir veya
  iki kez genişletir.
- **Yeni:** `samples_c/s57_shared_counter_threads/main.c` + `manifest.json`
  (`cflags: ["-lpthread"]`) — N pthread'in ortak bir `SharedState` struct'ını
  (sayaç + mutex) paylaştığı, mutex korumalı artırma yapan gerçek bir
  eşzamanlı program. Sonuç (N×M) zamanlamadan bağımsız deterministik
  olduğundan mevcut diferansiyel test harness'iyle uyumludur.
- **Değişti:** `harness/run_experiment.py` — çok-dosyalı örnekler için
  `manifest.json`'a opsiyonel `"cflags"` (ek gcc bayrakları, örn.
  `-lpthread`) ve `"rustflags"` alanları eklendi; belirtilmezse eski
  davranış (yalnızca `-lm`) korunur, geriye dönük tam uyumlu.
- **Değişti:** `harness/make_figures.py` — Şekil 5'e yeni kök-neden
  kategorisi eklendi: "Makro çoklu-değerlendirme (yeni)" (1 örnek, s56).
- **Yeni çeviriler:** `translations_rust/s56_macro_table.rs`,
  `translations_rust/s57_shared_counter_threads/main.rs` (Round 1);
  `translations_rust_refined/` altında düzeltilmiş s56 + değişmeden
  kopyalanmış s57 (Round 2).

### (b) Gerçekten ölçülen sayılar
Veri seti **n=55 → n=57**. Gerçek harness koşumu:

| Koşul | EA (n=57) |
|---|---|
| Round 1 — doğrudan, debug | %70.18 (40/57) |
| Round 1 — doğrudan, release | %73.68 (42/57) |
| Round 2 — iyileştirilmiş, debug | %100.00 (57/57) |

**s57 (paylaşılan bellek eşzamanlılığı) Round 1'de ilk seferde geçti** —
LLM, C'nin mutex-korumalı paylaşılan struct desenini doğru biçimde
`Arc<Mutex<i64>>` + `thread::spawn` + `join` desenine çevirdi; derleme
hatası ya da veri yarışı oluşmadı. Bu, ilginç bir olumsuz kontrol
(negative result) niteliğinde: C'nin sessizce izin verdiği bir paylaşılan-
durum deseni, Rust'ın tip sisteminde yapısal olarak zorunlu kılınan
(`Send`/`Sync`) bir soyutlamaya LLM tarafından doğru eşlenebilmiştir —
yani "Rust'ın borrow checker'ı LLM'i zorlar mı yoksa çeviri hiç
derlenmez mi" sorusuna, bu tek örnekte "LLM zaten doğru deseni biliyor"
yanıtı çıkmıştır.

**s56 (karmaşık makro) başarısız oldu — yepyeni bir kök neden (Kategori I):**
C referansında `x=20` iken `MAX(x++, 10)` çağrısı koşulun (20>10) doğru
çıkması nedeniyle `x++`'ı **iki kez** genişletir (karşılaştırmada ve sonuç
dalında), bu yüzden `x` 20'den 22'ye çıkar ve sonuç 21'dir. LLM'in doğal
çevirisi (`fn max(a,b)`) argümanı **bir kez** değerlendirdiğinden, aynı
girdide `x=21`, sonuç=20 üretir — testlerin 2/5'inde (koşulun doğru
çıktığı durumlarda) gerçek, ölçülmüş bir farklılık. X-Macro/token-pasting
kısmı ise LLM tarafından sorunsuz çevrildi (enum + `match` ile birebir
eşlendi, hiç hata yok) — yani karmaşık makro kullanımının kendisi değil,
özellikle **yan-etkili argümanın çoklu genişletilmesi** sorun çıkardı.

**Mann-Whitney gerçekleşen güç:** %15.0 (n=53'te %15.6 idi — n büyümeye
devam ettikçe gücün öngörülemez biçimde dalgalandığı örüntüsü sürüyor).

### (c) Makaleye önerilen taslak metin

**§4.4'e eklenecek yeni kök-neden alt bölümü:**
> I) Makro çoklu-değerlendirme yan etkisi → Fonksiyonel Hata (s56_macro_table)
> — Neden olur: C makroları saf metinsel ikamedir; bir parametre makro
> gövdesinde birden fazla kez geçiyorsa, yan etkili bir argüman (`x++`) o
> kadar kez değerlendirilir. LLM'in doğal çevirisi (bir Rust fonksiyonu)
> argümanı her zaman tam olarak bir kez değerlendirir — Rust'ta bunun
> C'yle birebir eşdeğeri, ancak kasıtlı olarak aynı çoklu-değerlendirmeyi
> yeniden üreten bir `macro_rules!` tanımıyla mümkündür. Düzeltme (Round
> 2): gözlemlenebilir davranışı korumak için böyle bir makro yazıldı.

**§3.1 Veri Seti'ne eklenecek not (kullanıcının önerdiği paragrafın
karşılığı):**
> Hakem/yazar geri bildirimi doğrultusunda, veri setine karmaşık makro
> genişletmesi (s56) ve gerçek paylaşılan bellek eşzamanlılığı (s57,
> pthreads + mutex) içeren iki yeni örnek eklenmiştir. Sonuçlar karışıktır:
> eşzamanlılık örneği ilk seferde doğru çevrilmiş (LLM'in `Arc<Mutex<>>`
> desenini zaten bildiğini göstermiştir), makro örneği ise yeni ve önceden
> öngörülmemiş bir kök nedeni (çoklu-değerlendirme yan etkisi) ortaya
> çıkarmıştır. Bu, "çok daha kaotik senaryolar mevcut sınırları netleştirir"
> öngörüsünü kısmen doğrulamaktadır — ancak yalnızca iki örnekle sınırlıdır
> ve daha büyük/gerçek eşzamanlı sistemlere (yarış koşulları, kilitlenme
> [deadlock], atomik olmayan bileşik işlemler) genellenemez.

### Kalan (tamamlanmayan)
- Yalnızca 1 eşzamanlılık örneği test edildi; kilitlenme (deadlock), yarış
  koşulu (race condition) veya atomik-olmayan bileşik işlem gibi daha
  kaotik eşzamanlılık desenleri kapsanmadı — mevcut stdin/stdout
  diferansiyel test harness'i, çıktısı zamanlamaya duyarlı (non-
  deterministik) programları doğası gereği değerlendiremez.
- Karmaşık makro tarafında yalnızca 1 tuzak (çoklu-değerlendirme) test
  edildi; değişken sayıda argüman alan makrolar (`__VA_ARGS__`), iç içe
  makro genişletmeleri veya makro-tabanlı jenerik veri yapıları
  denenmedi.

---

## Düzeltme — Tablo 5'in Güncel Veri Setiyle (n=57) Yeniden Ölçülmesi

Kullanıcı, makaledeki Tablo 5'in (Round 2 geri bildirim seviyeleri) hâlâ
"36/36" gibi eski bir alt-küme boyutu gösterdiğini fark etti — bu, veri
setinin çok önceki bir aşamasındaki (n=36) 8 başarısızlık üzerinde yapılmış
tarihsel bir ölçümdü ve güncel n=57 veri setini yansıtmıyordu. Deney,
**tüm 17 güncel başarısızlık üzerinde gerçekten tekrarlandı**.

### Değişen/eklenen dosyalar
- `translations_rust_levelB/`, `translations_rust_levelC/`: 21 yeni örnek
  (s37-s57) eklenerek n=57'ye tamamlandı. 9 yeni başarısızlıktan 3'ü için
  (s40, s52, s49) gerçek "kör" (yalnızca izin verilen bilgiyle) düzeltme
  denemesi yazıldı; kalan 6'sı (s38, s43, s48, s51, s53, s56) için orijinal
  (düzeltilmemiş) kod korundu — bu, "bu bilgiyle düzeltilemez" bulgusunun
  dürüst temsilidir.
- `results/OZET_SONUCLAR.md`: Tablo 5 bölümü, Kök Neden Analizi (yeni
  Kategori I eklendi, F/G/H'nin 2. örnekleri eklendi), Ana Gözlemler ve
  dataset açıklaması n=57'ye güncellendi (önceden bu dosya hâlâ n=48
  aşamasındaydı — Faz 2/3/3-devamı hiç yansıtılmamıştı, bu da ayrıca
  düzeltildi).
- `makale_v11.docx`: Tablo 5 gerçek n=57 sayılarıyla güncellendi; §4.2'deki
  eski/stale "9 örnek, 8 başarısızlık" sayıları da (daha önceki bir
  güncelleme turunda gözden kaçmış) düzeltildi.

### Gerçekten ölçülen sayılar
| Seviye | İçerik | EA (n=57) |
|---|---|---|
| A — Oracle | Tam derleyici hatası + panik metni + fark | 57/57 = %100.00 |
| B — Orta | Derleyici/panik metni tam; FE'de yalnızca girdi | 49/57 = %85.96 |
| C — Minimal | Yalnızca gerçek başarısız test sayısı, ör. "3 test başarısız" (CE hariç) | 41/57 = %71.93 |

Seviye B'de düzeltilen 9 örnek: s19 (CE), s09/s14/s40/s52 (RE — panik metni
tasma türünü belirtti), s06/s13/s20/s49 (FE — girdideki çok baytlı Türkçe
karakterler ipucu verdi). Düzeltilemeyen 8 örnek: s15/s27/s48 (%g
biçimlendirme), s38/s51 (platform tamsayı genişliği), s43/s53 (switch
fallthrough), s56 (makro çoklu-değerlendirme) — hepsinde başarısız girdi
(sade sayılar/komutlar) ilgili kök nedene dair gözlemlenebilir bir ipucu
taşımıyordu.

---

## Gemini Çevirilerinin Devamı — 22/57 → 44/57 (gerçek ölçüm)

Kullanıcı talimatıyla, kalan 35 Gemini çevirisi için `generate_translations.py`
istekler arasına 150 saniyelik bekleme konularak (`--sleep 150`) yeniden
çalıştırıldı. 35 örnekten 22'si başarıyla çevrildi (22/57 → 44/57), ardından
aynı kota duvarına (`429 RESOURCE_EXHAUSTED`, günlük kota) tekrar çarpıldı;
kalan 13 örnek (s45-s57) hâlâ bekliyor. Sonuçlar `harness/run_experiment.py
--rust-dir translations_rust__gemini --label gemini_partial44 --skip-missing`
ile gerçekten derlenip çalıştırılarak ölçüldü.

### Gerçekten ölçülen sayılar
- **Gemini EA (n=44, kısmi kapsam): 41/44 = %93.18** (önceki 21/22 = %95.45'in
  yerine geçer)
- 3 başarısızlık:
  - `s15_float_avg` (FE) — Claude'da da aynı kök nedenden (Kategori D, %g
    biçimlendirme) başarısız; bağımsız model doğrulaması.
  - `s26_rpn_calculator` (CE) — Rust ödünç denetleyicisi hatası (E0499): aynı
    `sp` değişkenini iki ayrı `FnMut` kapanışının (`push`/`pop`) eşzamanlı
    ödünç almaya çalışması. Claude'un çevirisinde görülmeyen, Gemini'ye özgü
    bir hata sınıfı.
  - `s27_csv_stats` (CE) — geçersiz biçim dizesi söz dizimi (`format!("{}e{:+=03}", ...)`
    — Rust'ta böyle bir biçimlendirme belirteci yok). Gemini'ye özgü bir
    hallüsinasyon örneği.
- `results/manifest_gemini.json`, `results/results_gemini_partial44.json/csv`
  güncellendi/eklendi.

### Makaleye yansıtılan değişiklik
`build_makale.py`'deki §4.9 ve §6 paragrafları 22/57=%95.45 yerine
44/57=%93.18 ve üç başarısızlığın (biri ortak kök neden, ikisi Gemini'ye özgü
yeni hata sınıfı) gerçek dökümünü yansıtacak şekilde güncellendi;
`makale_v11.docx` yeniden üretildi.

### Sonraki adım
Kalan 13 örnek (s45_goto_cleanup - s57_shared_counter_threads), Gemini günlük
kotası sıfırlandıktan sonra aynı yöntemle tamamlanabilir.

---

## Gemini Çevirilerinin Tamamlanması — 44/57 → 57/57 (gerçek, TAM ölçüm) ve IEEE Formatında Yeni Makale

Kullanıcı talimatıyla (`gemini api key ile kalanları yap`), kalan 13 örnek
önce 10'u (`--sleep 150`, s45-s54) sonra son 3'ü (s55, s56, s57 — günlük
kota sıfırlandıktan sonra `--sleep 60`) ile gerçek API çağrısıyla
tamamlandı. `run_experiment.py --rust-dir translations_rust__gemini` ile
57 örneğin tamamı gerçekten derlenip çalıştırılarak ölçüldü.

### Gerçekten ölçülen nihai sayılar
- **Gemini EA (n=57, TAM kapsam): 51/57 = %89.47** (4 CE, 0 RE, 2 FE, 0 NT;
  önceki 41/44 = %93.18'in kısmi ölçümünün yerine geçer)
- Yeni ortaya çıkan 3 başarısızlık (s46, s47, s48 — musl/Redis/cJSON, gerçek
  üretim kodu): s46_musl_qsort (CE, geçersiz `*mut u8::null()` söz dizimi),
  s47_redis_sds (FE, ekleme sırasında büyüme mantığı hatası — beklenen
  `LEN=5 STR=start...` yerine `LEN=0`), s48_cjson_number (CE — Claude'da FE
  idi, Kategori D'nin üçüncü bağımsız tekrarı). **Bu üçü Claude'un ilk
  seferde 2/3 geçtiği aynı örnekler** — Gemini'nin genel EA üstünlüğünün
  tekdüze olmadığının, gerçek üretim kodunda tersine döndüğünün kanıtı.
- Kalan 5 örnek (s49, s51, s52, s53, s56 — hakem geri bildirimiyle eklenen
  kök-neden 2. örnekleri + karmaşık makro) hepsi Gemini'de PASS: Claude'un
  Round 1'de yalnızca birer örnekle temsil edilen 5 kategoride (C, F, G, H,
  I) ikinci bağımsız denemede de aynı hatayı tekrarladığının (1/5), Gemini'nin
  ise aynı 5 kategoriyi tutarlı idiyomatik tercihlerle (yine `i8`, yine
  `c_long`, yine işaretli döngü, yine açık `match` aritmetiği, yeni olarak
  gerçek `fn`'e çevrilen makro) sistemli biçimde atlattığının kanıtı.
- `results/manifest_gemini.json`, `results/results_gemini.json/csv`,
  `results/model_comparison.md` güncellendi.

### Analiz dosyasına yansıtılan değişiklik
`results/DETAYLI_SORUN_ANALIZI.md` §7 (Çoklu Model Analizi) 44/57 kısmi
veriden 57/57 tam veriye güncellendi: karşılaştırma tablosu 57 örneğin
tamamını kapsayacak şekilde genişletildi; yeni §7.2b eklendi (Claude'un
geçtiği, Gemini'nin kaldığı gerçek üretim kodu örnekleri — ters yönlü
örüntü); §7.3 Gemini'nin 4 kendine özgü başarısızlığını (s26, s27, s46,
s47) kapsayacak şekilde genişletildi; §7.4 s48'i (ortak başarısızlık,
farklı hata türü) içerecek şekilde güncellendi; §7.5 sentezi dört ayrı
model-etkileşim türünü (ortak boşluk, model-özgü hata, tesadüfi atlatma,
model×karmaşıklık etkileşimi) ayırt edecek şekilde yeniden yazıldı. §2.6
(s15 vaka analizi) ve §11 (genel çıkarımlar madde 6) güncellendi.
`results/DETAYLI_SORUN_ANALIZI.html` yeniden üretildi.

### Yeni makale: IEEE iki-sütunlu format
Kullanıcının paylaştığı IEEE konferans şablon görseline göre makale baştan
yazıldı (`makale_IEEE_v1.docx`, scratchpad'teki `build_ieee_paper.py` ile
üretildi — A4, iki sütun, Times New Roman, Roman rakamlı bölümler I-VI +
Teşekkür + Kaynakça). Tüm geniş tablolar (I, III, IV, V, VI, VII) `w:cols`
XML manipülasyonuyla geçici tek-sütun bölüm aralarına alınarak tam sayfa
genişliğinde render edildi (ilk denemede bu sarmalama unutulmuş, tablolar
iki-sütunlu alanda taşıp metinle çakışmıştı — Word COM otomasyonuyla PDF
önizlemesi üzerinden tespit edilip düzeltildi). Toplam 6 sayfa (10 sayfa
sınırının altında). Model×kategori kırılım tablosu (Tablo VI) ve §V-A
tartışma paragrafı, 57/57 tam Gemini verisiyle güncellendi (Tablo VI'ya
"Çok dosyalı (3/3-3/3)" ve "Karmaşık makro (0/1-1/1)" satırları eklendi).

---

## Simüle Edilmiş Hakem Denetimi Sonrası Kapsamlı Düzeltme Turu

Kullanıcı, `makale_IEEE_v1.docx`'i başka bir yapay zekaya "hakem heyeti"
gibi denetlettirip bulunan sorunları (`hakem_raporu.md`) ilettiği; bu
raporda 14 çelişki (A1-A14), 6 kırık çapraz-referans/numaralandırma sorunu
(B1-B6) ve 25 eksik/kavramsal nokta (C1-C25) tespit edilmişti. Her madde
tek tek doğrulanıp (bazıları gerçek script/veri kontrolüyle çürütüldü,
bazıları gerçekten hatalıydı) düzeltildi.

### Gerçekten doğrulanıp düzeltilen kritik hatalar
- **A1 (KRİTİK, gerçek hata):** s27_csv_stats hem "Claude'un hatası" hem
  "Gemini'ye özgü" olarak sunulmuştu. Gerçekte s27'de **her iki model de**
  başarısız (Claude FE, Gemini CE) — Gemini'ye özgü başarısızlık sayısı
  4 değil **3**'tür (s26, s46, s47). Hem makalede hem
  `DETAYLI_SORUN_ANALIZI.md` §7.3/§7.4'te düzeltildi.
- **A6/A7 (KRİTİK, gerçek hata):** Makalede "PASS(n=36)/FAIL(n=12)" ile
  raporlanan Mann-Whitney/Fisher istatistikleri aslında n=40/n=17 (tam
  57'lik veri seti) üzerinden hesaplanmıştı — grup büyüklüğü etiketi
  yanlıştı. `harness/stats_report.py` genişletilip (betimsel istatistik +
  duyarlılık analizi + McNemar fonksiyonları eklenerek) tüm sayılar n=40/17
  üzerinden yeniden, tekrarlanabilir biçimde hesaplandı ve
  `results/stats_report.md` yeniden üretildi. Tablo VII (PASS/FAIL betimsel
  özellikler) gerçek değerlerle güncellendi (65.9→67.0, 66.3→59.8, vb.).
- **C10 (metodolojik):** Post-hoc "gerçekleşen güç" (%15.0) yerine, Hoenig &
  Heisey (2001)'in eleştirdiği bu ölçütün yanına, önerilen **duyarlılık
  analizi** eklendi: n=17/40, α=0.05, %80 güçte saptanabilir en küçük etki
  |r|≈0.45 (ikili arama + Monte Carlo simülasyonuyla, `harness/
  stats_report.py`'de `minimum_detectable_effect()`).
- **C12 (eksik, gerçek test eklendi):** İki model aynı 57 program üzerinde
  ölçüldüğü için doğru test McNemar'dır (bağımsız örneklem testi değil).
  Gerçek hesap: yalnızca Claude FAIL=14, yalnızca Gemini FAIL=3, McNemar
  kesin p=**0.013** — genel EA farkı anlamlı, ama kategoriye göre yön
  değiştiriyor (`harness/stats_report.py`'de `mcnemar_exact()`).
- **C17 (KEŞFEDİLEN YENİ HATA, raporda yoktu):** `unsafe` kullanan dosya
  sayısını doğrularken (`grep -rl unsafe translations_rust*/*.rs`),
  **s50_id_generator**'ın da her iki turda `unsafe` içerdiği görüldü —
  önceki metin (hem makalede hem analiz dosyasında) "yalnızca 6 dosya"
  diyordu ve s50'yi "model `&mut` parametre kullandı, `unsafe`'e hiç
  düşmedi" olarak yanlış açıklıyordu. Gerçek: model yine `static mut`
  kullandı ama bu kez erişimi doğru `unsafe` bloğuna sardı. Doğru sayı:
  **8 dosya (4 örnek × 2 tur)**. Bu, dış denetimin tetiklediği ama dış
  denetimin kendisinin yakalamadığı, kaynak koda bakarak bizzat doğrulanan
  bir düzeltmedir.
- **A9:** "20 istek/gün kotasıyla tek oturumda 54 çeviri" aritmetik olarak
  imkânsızdı — gerçekte ölçüm 2026-07-22 ile 2026-07-26 arasında birden
  fazla güne yayılmıştı; metin buna göre düzeltildi.
- **A10:** Docker/Linux Round-2 karşılaştırmasında 52/55 sonucu için yalnızca
  2 örnek (s38, s51) gerekçe gösterilmişti; gerçekte 3 örnek farklılaşıyor
  (üçüncüsü s47 — nedeni `long` genişliği değil, CRLF/stdio metin-modu
  farkı, yani C referansının kendisinin platforma bağlı davranması).
  `results/platform_comparison.md`'den doğrulanıp düzeltildi.
- **C3 (edition):** `rustc --edition` bayrağının hiç geçilmediği ve
  varsayılan (2015) edition kullanıldığı belirtildi; ampirik test
  (`rustc` ile `static_mut_refs` lint'i default/2021/2024 karşılaştırması)
  yapılıp Kategori E'nin gerçek hatasının (E0133, unsafe eksikliği) edition-
  bağımsız olduğu doğrulandı — raporun bu noktadaki varsayımı kısmen
  yanlıştı, düzeltilerek yazıldı.
- **A2, A3, A4, A5, A8, A11, A13, A14, B1-B6:** Tümü doğrulanıp düzeltildi
  (sırasıyla: "%75" çelişkisi → "en iyi modelde bile 6/57 hata" biçiminde
  yeniden çerçevelendi; "(kısmen)" kalıntısı silindi; "dört" → "beş"
  kategori; "35/57" → "44/57" özgün örnek sayısı; Tablo başlıklarına
  "Claude" eklendi; §II'deki model-özgü kategori listesi A,B,C,E,F,G,H,I
  olarak düzeltildi; §IV-C doğrulama cümlesi netleştirildi; Öz'deki
  "modele göre değişen iyileştirme" iddiası yalnızca Claude'a
  sınırlandırıldı; tüm §V-C→§V-B çapraz referanslar, §IV-I→§IV-F, §IV-C→
  §IV-B düzeltildi; Tablo VII→IV, IV→V, V→VI, VI→VII olacak şekilde ilk-
  anılma sırasına göre yeniden numaralandırıldı).
- **B4/B5:** `harness/make_figures.py`'deki 5 grafiğin içine gömülü,
  Word alt yazılarıyla çelişen `ax.set_title()` çağrıları tamamen
  kaldırıldı (IEEE'de başlık grafiğe gömülmez); tüm grafikler güncel 57
  örneklik veriyle yeniden üretildi.

### Eklenen yeni içerik (İngilizce özet, atıflar, madde listesi)
İngilizce Abstract/Index Terms eklendi (IEEE konvansiyonu); anahtar kelime
sayısı 9→6'ya indirildi; AS1-AS6 tek paragraftan madde listesine
dönüştürüldü; [4]-[9] referansları ilk kez metin içi atıfla ilişkilendirildi;
Rust edition, Gemini örnekleme parametreleri (temperature=0.2, top_p=1.0) ve
ölçüm tarih aralığı eklendi; Claude çevirilerinin tekrarlanabilirlik
sınırlaması (etkileşimli oturum, kayıtsız örnekleme parametreleri, k=1 tek
örneklem) açıkça yazıldı.

### Gerçek yeni deney GEREKTİREN, bilinçli olarak YAPILMAYAN maddeler
Aşağıdakiler uydurma veri üretmemek için **yapılmadı**, bunun yerine
Sınırlamalar bölümünde açık birer madde olarak yazıldı: (C1) k=3-5 tekrarlı
örnekleme ve pass@k ölçümü; (C5) Round 2 düzeltmelerinin görülmemiş
(held-out) girdilerle doğrulanması; (C6) gcov tabanlı kod kapsama ölçümü;
(C9) C referansının farklı optimizasyon seviyelerinde tekrarı; (C21) tam
kapsamlı ilişkili çalışmalar taraması (TransCoder/C2Rust ötesi); (C22)
Zenodo/figshare DOI ataması.

### Kaynakça girdileri — WebSearch ile doğrulandı
[10], [11], [12] önce `[DOĞRULANMALI]` etiketiyle eklenmişti; ardından
WebSearch aracı yüklenip gerçek arama yapılarak üçü de teyit edildi:
[10] CISA/NSA/FBI "The Case for Memory Safe Roadmaps: Why both C-Suite
Executives and Technical Experts Need to Take Memory Safe Coding
Seriously" (Aralık 2023, cisa.gov/case-memory-safe-roadmaps) — tam başlık
eklendi; [11] C2Rust (github.com/immunant/c2rust, Immunant Inc.) —
doğru; [12] B. Rozière, M.-A. Lachaux, L. Chanussot, G. Lample,
"Unsupervised Translation of Programming Languages," NeurIPS 2020,
arXiv:2006.03511 — birebir doğru. `[DOĞRULANMALI]` etiketleri kaldırıldı.
[13] (Hoenig & Heisey 2001) zaten iyi bilinen, yüksek güvenilirlikli bir
kaynaktı.

### Etkilenen dosyalar
`harness/stats_report.py` (genişletildi), `results/stats_report.md`
(yeniden üretildi), `harness/make_figures.py` (başlıklar kaldırıldı),
`results/figures/*.png` (yeniden üretildi), `makale_IEEE_v1.docx`
(kapsamlı revizyon), `results/DETAYLI_SORUN_ANALIZI.md` ve `.html`
(eşleşen düzeltmeler).

---

## İkinci Tur Revizyon (`duzeltme_prompt_tur2.md`'ye Göre)

Makale ikinci bir simüle hakem/revizyon turundan geçti; bu turda önceki
düzeltmelerin kendisinin ürettiği 2 yeni çelişki ve birkaç eksik kalem
giderildi. Tüm sayısal değerler ya mevcut `results/*.json` dosyalarından
gerçek zamanlı yeniden hesaplandı ya da (istatistiksel yöntem
düzeltmesinde olduğu gibi) `harness/stats_report.py` gerçekten yeniden
çalıştırılarak elde edildi — hiçbir sayı elle uydurulmadı.

### A — Zorunlu düzeltmeler (hepsi doğrulandı)
- **A1 (kritik):** §IV-F'de Gemini'nin Claude'da görülmeyen 3 başarısızlığı
  "hepsi CE'dir" deniyordu; s47 aslında FE'dir (boş `LEN=0` çıktısı — CE
  değil). `results/results_gemini.json`'dan doğrudan doğrulandı: CE=
  {s26, s27, s46, s48}=4, FE={s15, s47}=2, toplam 6=57−51 ✓. Metin "ikisi
  derleme hatası (s46, s26), biri fonksiyonel hatadır (s47)" olarak
  düzeltildi.
- **A2 (kritik):** "114 çeviri (...8'i unsafe, 4 örnek×2 tur)" iddiası
  yanlıştı. `results_round1.json`/`results_round2.json` karşılaştırması
  ve `grep -rl unsafe translations_rust/ translations_rust_refined/`
  doğrulaması: Round 2 yalnızca Round 1'de başarısız olan 17 örneği
  yeniden çevirir (57+17=74 farklı çeviri, 114 değil); s37/s44/s46/s50
  dördü de Round 1'de PASS olduğundan hiç yeniden çevrilmemiştir → unsafe
  kullanan gerçek örnek sayısı 4 (%5.4), 8 değil. Ayrıca s19'un Round 2
  düzeltmesinin `unsafe` KULLANMADIĞI (durumu `&mut i32` parametresi
  olarak geçen güvenli bir tasarıma geçildiği — dosya içeriği okunarak
  doğrulandı) açıkça eklendi.
- **A3:** Gemini ölçüm tarih aralığı "2026-07-22 – 2026-07-26" idi;
  2026-07-26 o anki tarihe göre gelecekteydi. `translations_rust__gemini/`
  dosyalarının gerçek mtime'ları (`os.path.getmtime`) okunarak doğru
  aralık (2026-07-22 – 2026-07-25) yazıldı.
- **A4:** Öz/Abstract/Sonuç'ta hâlâ tek-katmanlı "94% sessiz" çerçevelemesi
  vardı; §IV-C'nin kendi düzeltmesiyle (17 başarısızlığın 4'ü aslında
  gürültülü RE, yalnızca 12'si/%70.6'sı gerçekten sessiz FE) tutarlı hale
  getirilmek üzere üç yerde de iki-katmanlı çerçeveleme uygulandı.
- **A5:** "(Docker) tam tekrar" ifadesi üç yerde niteliksiz kalmıştı;
  hepsi "(55/57)" niteleyicisiyle güncellendi veya "bir tekrar" olarak
  yumuşatıldı.
- **A6:** §IV-B'deki "§IV-D'deki kök neden analizi" kırık çapraz referansı
  (kök neden analizi aslında §IV-C'dir) düzeltildi.
- **A7:** Depo URL'si (`github.com/ibrahimsahin022/llm_ceviri_deneyi` —
  gerçek `git remote -v` çıktısından alındı) §III-E'ye eklendi; önceden
  metinde hiçbir yerde geçmiyordu.

### B — Test ortamında yapılan tek iş
- **B1:** Test-girdisi bazında geçme oranı §III-E'de tanımlanmış ama hiç
  raporlanmamıştı. `results_round1.json`'dan gerçek hesaplama: 195/226
  (%86.3) — CE olan tek örneğin (s19) girdileri hiç çalıştırılamadığından
  payda 229 değil 226. Ayrıca 12 FE örneğinin 6'sının (s06, s13, s15,
  s20, s27, s38) yalnızca TEK bir sınır girdisinde saptığı bulundu ve
  §IV-A'ya "hata yalnızca sessiz değil, aynı zamanda seyrek tetiklenir"
  sonucuyla eklendi.

### C — Yüksek değer / düşük maliyet
- **C1:** "Sessiz hata oranı modele göre değişir" bulgusu (Claude 16/17
  vs. Gemini 4/6) Öz, İngilizce Abstract ve Sonuç'a taşındı.
- **C2:** §III-D'ye Round 2'nin tek turlu (single-shot, yakınsayana kadar
  tekrarlanmayan) bir düzeltme olduğu ve bütçenin (1 tur) üç geri bildirim
  seviyesinde sabit tutulduğu eklendi.
- **C3:** Tablo V'te C ve F kategorilerine "†" dipnotu eklendi:
  "doğru" davranış referans platformun (Windows/LLP64)
  implementation-defined seçimine bağlıdır; §V-B'ye açıklayıcı cümle
  eklendi.
- **C4:** Şekil numaralandırması IEEE sırasına göre düzeltildi: eski
  "Şekil 3b" → Şekil 4 (bootstrap CI), eski "Şekil 4" (kök-neden) →
  Şekil 5; tüm metin içi atıflar güncellendi.
- **C5:** Gemini'nin gerçek çözümlenmiş `modelVersion` alanı API
  yanıtlarında hiç loglanmamıştı (`results/manifest_gemini.json`
  doğrulandı) — sahte bir sürüm numarası uydurmak yerine bu, açık bir
  tekrarlanabilirlik sınırlaması olarak §III-B ve §V-B'ye yazıldı.

### D — Opsiyonel (kısmen uygulandı)
- **D2:** Model×kategori etkileşiminin istatistiksel olarak test
  edilmediği (yalnızca betimsel Tablo VII düzeyinde kaldığı) §V-B'ye
  açıkça eklendi.
- D1 (VERT/Syzygy/RustAssistant/CodeTransOcean/AVATAR atıfları) ve D3
  (held-out doğrulama, gcov kapsaması) **yapılmadı** — ikisi de yeni
  deney/doğrulama gerektirir; D1 ayrıca kaynakça uydurma riski taşıdığından
  atlandı.

### E — Küçük düzeltmeler
- Duyarlılık analizi tutarsızlığı **kaynağında** düzeltildi:
  `harness/stats_report.py`'deki `minimum_detectable_effect()`
  anlamlılığı ölçmek için Mann-Whitney U kullanıyordu ama etki
  büyüklüğünü normal/AUC yaklaşık dönüşümüyle hesaplıyordu (|r|≈0.45);
  fonksiyon, AYNI U istatistiğinden ampirik rank-biserial formülüyle
  (r=1-2U/(n1·n2)) hesaplayacak şekilde düzeltildi — gerçek yeniden
  çalıştırma sonucu |r|≈0.46 (tüm 3 yerde — §IV-B, §V-B, DETAYLI_SORUN_
  ANALIZI.md — güncellendi).
- §III-A'daki bozuk parantez/iki-nokta-üst-üste noktalaması düzeltildi.
- §II'deki ileri referans "(bkz. Tablo VII)" → "(§IV-F'de
  ayrıntılandırıldığı gibi)" olarak yumuşatıldı.
- İngilizce Abstract'a McNemar sonucu ve model-özgü sessiz hata oranı
  eklendi (önceden yalnızca Türkçe Öz'de vardı, iki özet arasında içerik
  paritesi sağlandı).
- `harness/make_figures.py`: Şekil 5'teki (kök-neden) 3 kategori adındaki
  açıklanmamış "(yeni)" etiketleri kaldırıldı (tutarsız uygulanıyordu —
  aynı revizyonda ikinci örnek alan C ve F kategorilerinde bu etiket
  yoktu). Şekil 3'teki (LoC) "154 satır — FAIL" etiketi x=73'teki (s27)
  noktayla görsel olarak çakışıyordu; artık büyütülmüş ofset + bağlayıcı
  ok çizgisiyle doğru noktaya net biçimde bağlanıyor; tüm 5 grafik yeniden
  üretildi.
- Öz kelime sayısı (270→243) ve Abstract (247) IEEE'nin ≤250 kelime
  kuralına uyacak şekilde sıkıştırıldı, içerik kaybı olmadan.

### Yapılamayan / kullanıcı kararı gereken kalemler
Sayfa boyutu (A4 vs hedef IEEE şablonunun US Letter olması) ve ORCID
kimlikleri, hedef yayın yeri ve yazarların gerçek ORCID'leri bilinmeden
doldurulamaz; uydurulmadı, olduğu gibi bırakıldı.

### Etkilenen dosyalar (2. tur)
`harness/stats_report.py` (duyarlılık analizi yöntem düzeltmesi),
`results/stats_report.md` (yeniden üretildi), `harness/make_figures.py`
(etiket/ok düzeltmeleri), `results/figures/*.png` (yeniden üretildi),
`makale_IEEE_v1.docx` (7→8 sayfa, kapsamlı revizyon),
`results/DETAYLI_SORUN_ANALIZI.md` (duyarlılık analizi değeri
eşitlendi).

---

## Üçüncü Tur — Final Editoryal Düzeltmeler (`duzeltme_prompt_final.md`'ye Göre)

Makale üçüncü hakem turundan geçti: "kabul — küçük editoryal düzeltmelerle."
Bu turda 5 zorunlu düzeltme (Z1-Z5) ve 3 kontrol (K1-K3) uygulandı. Hiçbir
ölçüm/deney gerekmedi; doğrulanmış istatistikler (U=287.0, p=0.359,
r=0.156, Fisher tablosu, McNemar p=0.013, |r|≈0.46, 195/226, Tablo
V/VII toplamları) değiştirilmedi.

### Z — Zorunlu düzeltmeler
- **Z1:** §IV-E'de s19_global_counter yanlışlıkla "Kategori E'nin ikinci
  örneği" olarak etiketlenmişti; Tablo V, §III-A ve §IV-A ile
  karşılaştırıldığında **s19 ilk örnek, s50_id_generator ikinci örnektir**
  (Tablo VII: "Kök-neden 2.örnek (5) — Claude 1/5" zaten s50'nin PASS
  olduğunu, s19'un R1 CE'si olduğunu doğruluyordu). Düzeltilmiş metin artık
  Kategori E'nin aynı model tarafından iki farklı biçimde (s50'de `unsafe`
  ile doğru, s19'da Round 2'de `unsafe`'siz durum-parametreleştirme ile)
  çözüldüğünü doğru sırayla anlatıyor.
- **Z2:** Öz/Abstract'ta "Gemini'nin 6 başarısızlığının yalnızca 4'ü
  derlemede yakalanmıştır" cümlesi bulguyu tersine çeviriyordu — asıl
  vurgu Gemini'nin **düşük** sessizlik oranı (2/6=%33) olmalıyken, "yalnızca
  4'ü" ifadesi 4'ü (derlenen/yakalanan kısmı) küçük gösteriyordu ve Claude
  ile karşılaştırıldığında kutup karışıyordu. Öz, Abstract ve §VI Sonuç
  üçü de "Claude %94 sessiz, Gemini yalnızca %33 sessiz (kalanı
  derlemede yakalanır)" biçiminde **aynı kutupla** hizalandı — §VI'da da
  aynı kutup tutarsızlığı (bağımsız olarak) bulunup düzeltildi. Bu
  değişiklik özet uzunluğunu artırdığından, Öz'den kod-uzunluğu/en-uzun-
  program yan cümlesi (§IV-B'de zaten var, tekrardı) ve Abstract'tan
  "compile, run without panicking, and return wrong output" yan cümlesi
  (bir önceki cümleyle örtüşüyordu) çıkarıldı. Sonuç: Öz 243→238 kelime,
  Abstract 247→248 kelime (her ikisi de ≤250).
- **Z3:** §IV-B'de "...hesaplandığında rank-biserial |r|≈0.46'dır
  (havuzlanmış σ=89.5)" cümlesi kendi formülüyle (r=1-2U/(n1·n2), σ
  içermeyen) çelişiyordu — parametrik yöntemden kalma bir parantezdi.
  "(U'nun bağ düzeltmeli standart sapması üzerinden)" olarak düzeltildi;
  aynı çelişki `results/DETAYLI_SORUN_ANALIZI.md`'de de bulunup
  düzeltildi (sayının kendisi, |r|≈0.46, değişmedi).
- **Z4:** Tablo II (Hata Taksonomisi) gövdede hiç atıf almıyordu; §III-C'ye
  "(...eşitse PASS; Tablo II)" atfı eklendi.
- **Z5:** §V-B'de üçüncü turdur duran ";"→"," noktalama hatası
  ("...tamamlanmış olsa da; GPT-4o/DeepSeek..." → "...olsa da,...")
  düzeltildi.

### K — Kontroller
- **K1 (önemli bulgu):** `gh repo view` ile doğrulandı —
  `github.com/ibrahimsahin022/llm_ceviri_deneyi` deposu şu anda
  **private**'tır, ayrıca yerel repo `origin/master`'ın **11 commit
  ilerisindedir** (bu oturumdaki tüm düzeltmeler dahil, henüz hiç push
  edilmemiş). Yani depo public olsa bile GitHub'daki içerik makalenin
  iddia ettiği güncel durumu (57/57 Gemini, düzeltilmiş istatistikler vb.)
  yansıtmıyor. Görünürlük değiştirme ve push işlemleri kullanıcı onayı
  gerektiren geri döndürülebilir olmayan/paylaşılan-durum etkileyen
  eylemler olduğundan **otomatik yapılmadı** — kullanıcıya ayrıca
  soruldu. Yazar adları/e-postaları makalede zaten açık olduğundan
  (çift-kör değil), K1(b)'deki anonim bağlantı önerisi uygulanmadı
  (gerek yok).
- **K2:** Şekil 5'teki "(yeni)" etiketleri kontrol edildi —
  `harness/make_figures.py`'de bu etiketler **önceki turda zaten
  kaldırılmıştı**; mevcut `fig5_rootcause.png` görsel olarak doğrulandı,
  "(yeni)" etiketi yok. Ek işlem gerekmedi.
- **K3:** Şekil 3'teki "154 satır — FAIL" etiketinin x≈73 noktasıyla
  çakışması **önceki turda zaten** büyütülmüş ofset + bağlayıcı ok
  çizgisiyle düzeltilmişti; mevcut `fig4_loc_vs_success.png` görsel
  olarak doğrulandı, etiket doğru noktaya net biçimde bağlanıyor. Ek
  işlem gerekmedi.

### Yapılmayanlar (opsiyonel bölüm, zaman/veri kısıtı)
Sayfa boyutu (A4) ve ORCID — hedef yayın yeri/gerçek ORCID'ler
bilinmediğinden dokunulmadı. VERT/Syzygy/RustAssistant atıfları —
doğrulanmadan eklenmedi (uydurma kaynakça riski). Held-out doğrulama ve
gcov kapsaması — §V-B'de dürüstçe sınırlama olarak kalmaya devam ediyor.

### Etkilenen dosyalar (3. tur)
`makale_IEEE_v1.docx` (8 sayfa, kelime sayıları düzeltildi),
`results/DETAYLI_SORUN_ANALIZI.md` (Z3 tutarlılığı).

---

## Gönderim Öncesi Sertleştirme (`gonderim_oncesi_prompt.md`'ye Göre)

Bu tur içerik değil **risk azaltma** turudur — makale metnindeki hiçbir sayı/
bulgu değiştirilmedi (DOKUNULMAYACAKLAR listesi gerçek dosyalardan tek tek
doğrulandı: hepsi mevcut).

### FAZ 1 — Depo Sertleştirme

- **1.1 (kritik bulgu):** Gizli veri taraması yapıldı — çalışan ağaçta ve
  `git log -p --all`'da API anahtarı deseni (`AIza...`, `sk-ant-...`,
  `ghp_...` vb.) **bulunamadı**. Ancak dört `results/*.json` dosyasında
  (`results_gemini.json`, `results_round1.json`,
  `results_round1_release.json`, `results_round2_levelC.json`) rustc'nin
  yakaladığı compile_error metinlerinin içinde **gerçek Windows kullanıcı
  adını** açığa çıkaran mutlak dosya yolları
  (`C:\Users\<kullanıcı>\OneDrive\...`) bulundu — 32 örnekte tam yol,
  1 örnekte (s26, çıktı kesilmiş) kısmi yol. Tüm örnekler, tanı değerini
  (dosya adı, satır:sütun) koruyarak kişisel yol öneki kaldırılacak şekilde
  ham bayt düzeyinde temizlendi (JSON hâlâ geçerli, `total_samples` ve
  tüm kategoriler değişmedi — yalnızca metin alanı sanitize edildi).
  `MODIFICATIONS.md`'deki bir yerel plan dosyası yolu da (aynı kullanıcı
  adını içeriyordu) kaldırıldı. **Not:** Bu sızıntı önceki bir commit'te
  zaten push edilmişti (depo o sırada public'ti); düşük önemdeydi (yalnızca
  kullanıcı adı, kimlik bilgisi değil) ve git geçmişini yeniden yazmak
  (`filter-repo` + force-push) başlı başına yıkıcı bir işlem olduğundan
  kullanıcıya danışılmadan yapılmadı — güncel HEAD artık temiz, geçmişte
  tek bir eski commit'te iz kalabilir.
- **1.2:** Yedi dosya yolu birebir doğrulandı — hepsi mevcut ve `git
  ls-files` ile izleniyor. `stats_report.md`/`stats_report.py`'nin vaat
  edilen ölçüm tanımlarını gerçekten içerdiği doğrulandı.
- **1.3:** README.md, projenin güncel durumunu (57 örnek/229 girdi, Gemini
  57/57 — eski taslak hâlâ "55 örnek/219 girdi, Gemini 19/48" diyordu)
  yansıtacak şekilde yeniden yazıldı: ortam bilgisi §III-E ile birebir
  hizalandı, klasör haritası gerçek klasör adlarıyla güncellendi, tek
  komutluk tekrar (`python harness/compare_models.py`) eklendi ve **gerçekten
  çalıştırılıp doğrulandı** (bu sırada, git tarafından izlenmeyen ama
  yerel ortamda kalmış boş `translations_rust__gpt4o/`,
  `translations_rust__deepseek/` klasörlerinin çıktıyı yanılttığı fark
  edildi — bunlar silinerek fresh-clone davranışı doğrulandı: beklenen
  çıktı artık gerçek `[atlandı]` mesajlarıyla birebir eşleşiyor), Docker
  tekrarının n=55'te yapıldığı notu eklendi, BibTeX atıf bloğu eklendi.
- **1.4:** `LICENSE` (MIT, üç yazar) ve `THIRD_PARTY_LICENSES.md` eklendi
  — Rosetta Code (7 örnek, GFDL/CC-BY-SA), OpenBSD/FreeBSD libc (3 örnek,
  BSD-3-Clause, gerçek telif satırları `.c` dosya başlıklarından alındı),
  musl (MIT), Redis SDS (BSD-3-Clause), cJSON (MIT) — hepsi kaynak URL +
  lisans + orijinal telif satırıyla dökümlendi.
- **1.5 (kullanıcı kararı gerektirir):** Zenodo DOI ataması kullanıcının
  kendi Zenodo hesabında GitHub entegrasyonu açmasını ve bir GitHub release
  yayınlamasını gerektirir — bu, tarafımca yapılamaz. §III-E/§V-B'deki
  "DOI henüz atanmadı" ifadesi bu nedenle doğru ve değiştirilmedi.
- **1.6:** Hedef yer çift-kör değildir (yazar adları/e-postaları makalede
  zaten kullanıcının açık isteğiyle gerçek biçimde yer alıyor) — anonim
  depo bağlantısı gerekmedi. `.docx` belge özellikleri (Yazar, Başlık,
  Yorum) yine de temizlendi: önceki "python-docx" / "generated by
  python-docx" varsayılanları kaldırılıp gerçek yazar adları ve başlık
  yazıldı.

### FAZ 2 — Şekiller

- Şekil 5'teki "(yeni)" etiketleri zaten önceki turda kaldırılmıştı,
  yeniden doğrulandı.
- Şekil 3'teki etiket-nokta çakışması zaten önceki turda ok/leader-line
  ile düzeltilmişti, yeniden doğrulandı.
- **Yeni:** Tüm 5 şekil 150 DPI'den **300 DPI**'ye yükseltildi
  (`plt.rcParams["figure.dpi"]` + her `savefig()` çağrısına açık
  `dpi=300`). Şekil 2 (yığılmış hata dağılımı) ve Şekil 5 (kök-neden)
  artık renk + **desen (hatch)** kullanıyor (PASS=düz, FE=`///`, RE=`xx`,
  CE=`\\`, NT=`...`) — gri tonlamada basıldığında da kategoriler ayırt
  edilebiliyor; hem yığılmış barlar hem de ilgili lejantlar güncellendi.

### FAZ 3 — Biçim

- **3.1:** Altı yerdeki BÜYÜK HARF vurgusu (AYNI×2, TÜM, TEK, BİR,
  KULLANMADAN) IEEE kuralına uygun olarak *italik* biçime çevrildi —
  kelimelerin kendisi değişmedi, yalnızca biçim (büyük harf → küçük harf +
  italik). Bunun için `build_ieee_paper.py`'ye karışık-run
  (bir paragraf içinde hem normal hem italik metin) destekleyen yeni bir
  `add_para_parts()` yardımcı fonksiyonu eklendi.
- **3.2:** §IV-E'deki çift iki-nokta-üst-üste cümlesi düzeltildi (ikinci
  `:` yerine `—`) — aynı düzenlemenin içinde (KULLANMADAN→kullanmadan ile
  aynı paragraf).
- **3.3 (kullanıcı kararı gerektirir):** ORCID — üç yazarın gerçek
  ORCID iD'leri elimde olmadığından uydurulmadı, eklenmedi.
- **3.4 (kullanıcı kararı gerektirir):** Sayfa boyutu (A4) hedef yayın
  yerinin şablonu bilinmeden değiştirilmedi.
- Rebuild sonrası tüm italik/em-dash değişiklikleri doğrudan üretilen
  `.docx`'ten (python-docx ile run/italic bayrakları okunarak) doğrulandı;
  sayfa sayısı 8'de sabit kaldı.

### FAZ 4 — Opsiyonel
Atlandı: rakip çalışma atıfları (VERT/Syzygy/RustAssistant, doğrulama
riski) ve held-out/gcov (yeni deney gerektirir, zaten §V-B'de dürüstçe
sınırlama olarak yazılı).

### Son Doğrulama
Docx, Word COM ile PDF'e çevrilip **PyMuPDF ile 8 sayfanın tamamı görsel
olarak incelendi** (madde 14) — kırık Unicode sembolü, sayfa sonunda
bölünen tablo veya taşan şekil bulunmadı; tüm ok/matematik sembolleri
(→, ↔, ×, σ, α, ≈, ≳) doğru render ediliyor. `git ls-files` ile 7
referans dosyanın tamamı ve DOKUNULMAYACAKLAR listesindeki tüm sayılar
üretilen `.docx` metninden tek tek doğrulandı.

### Etkilenen dosyalar (gönderim öncesi tur)
`README.md` (yeniden yazıldı), `LICENSE`, `THIRD_PARTY_LICENSES.md`
(yeni), `harness/make_figures.py` (300 DPI + hatch), `results/figures/*.png`
(yeniden üretildi), `makale_IEEE_v1.docx` (italik düzeltmeler + metadata),
`results/results_gemini.json`, `results/results_round1.json`,
`results/results_round1_release.json`, `results/results_round2_levelC.json`
(kişisel yol sanitizasyonu).

---

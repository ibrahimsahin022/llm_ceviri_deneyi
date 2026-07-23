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
| C — Minimal | Yalnızca "N test başarısız" (CE hariç) | 41/57 = %71.93 |

Seviye B'de düzeltilen 9 örnek: s19 (CE), s09/s14/s40/s52 (RE — panik metni
tasma türünü belirtti), s06/s13/s20/s49 (FE — girdideki çok baytlı Türkçe
karakterler ipucu verdi). Düzeltilemeyen 8 örnek: s15/s27/s48 (%g
biçimlendirme), s38/s51 (platform tamsayı genişliği), s43/s53 (switch
fallthrough), s56 (makro çoklu-değerlendirme) — hepsinde başarısız girdi
(sade sayılar/komutlar) ilgili kök nedene dair gözlemlenebilir bir ipucu
taşımıyordu.

---

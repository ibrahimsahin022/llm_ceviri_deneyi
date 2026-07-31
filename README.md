# LLM Kod Çevirisi Deney Ortamı (C → Rust)

Bu proje, **eski kodların büyük dil modelleriyle (LLM) yeni bir dile çevrilirken
oluşan sessiz semantik hataları** deneysel olarak ölçmek için hazırlanmış,
çalıştırılabilir bir test ortamıdır. Makalenin "Yöntem" ve "Bulgular" bölümlerinin
verisi bu ortamdan üretilir. Proje **57 C programı** ve **233 test girdisi**
içerir (24 temel algoritma; 5 daha uzun özgün program; Rosetta Code'dan [4] 7
algoritma; OpenBSD/FreeBSD libc'sinden [5] 3 üretim fonksiyonu; musl/Redis/cJSON'dan
[6,7,8] 3 uzun üretim kodu; hedeflenmemiş boşlukları kapsayan 6 özgün program;
beş kök-neden kategorisinin bağımsız 2. örnekleri (5 program); çok dosyalı/
eşzamanlı yapı test eden 3 program; karmaşık makro kullanımını test eden 1
program — tam katalog `results/VERISETI_VE_ALGORITMALAR.md`'dedir).

Deney **iki bağımsız model** üzerinde gerçek, otomatik ölçüm içerir: Claude
Sonnet 5 (57/57, EA=%70.18) ve Google Gemini (57/57, EA=%89.47, gerçek API
çağrısıyla — bkz. `harness/translators/`). Ayrıca **çok dosyalı kod** (bkz.
`samples_c/s54_stack_module/`, `s55_config_parser/`, `s57_shared_counter_threads/`)
ve **çoklu platform** (Windows/LLP64 + Docker/Linux/LP64, bkz. `Dockerfile`,
`harness/compare_platforms.py`) boyutlarını da kapsar — ayrıntı ve gerçek
ölçülmüş sayılar için `MODIFICATIONS.md`'e bakın.

Sonuç dosyaları (`results/` altında):
- `OZET_SONUCLAR.md` — makaleye hazır bulgular + hata analizi.
- `VERISETI_VE_ALGORITMALAR.md` — kullanılan model/araçlar, veri seti kartı ve her programın algoritması.
- `DETAYLI_SORUN_ANALIZI.md` / `.html` — 57 örneğin tamamının vaka analizi.
- `stats_report.md` — bootstrap GA, Mann-Whitney duyarlılık analizi, Fisher GA, McNemar testi (üreten betik: `harness/stats_report.py`).
- `platform_comparison.md` — Windows/LLP64 vs Linux/LP64 karşılaştırması (üreten betik: `harness/compare_platforms.py`).
- `model_comparison.md` — Claude vs Gemini karşılaştırma tablosu (üreten betik: `harness/compare_models.py`).

## Ne yapıyor?
1. `samples_c/` içindeki C programlarını **referans (doğru)** kabul eder.
2. `translations_rust/` (Claude) ve `translations_rust__gemini/` (Gemini)
   içindeki, aynı programların **LLM ile Rust'a çevrilmiş** hallerini alır.
3. Her ikisini de derleyip `tests/` içindeki girdilerle çalıştırır ve çıktıları
   **karşılaştırır** (diferansiyel test).
4. Farkları 4 hata türünde sınıflandırır: Derleme (CE), Çalışma Zamanı (RE),
   Sonlanmama (NT), Fonksiyonel (FE).
5. **Yürütme Doğruluğu (EA)** ve hata dağılımını hesaplayıp `results/` altına yazar.

## Klasör yapısı
```
llm_ceviri_deneyi/
├── samples_c/                  # Kaynak (referans) C programları — tekil .c dosyaları
│                                #   veya çok dosyalı örnekler icin alt klasor (manifest.json ile)
├── tests/<ornek>/*.txt          # Her örnek için test girdileri (233 dosya, 57 örnek)
├── translations_rust/           # Claude Sonnet 5 cevirileri (Round 1 — dogrudan)
├── translations_rust_refined/   # Claude, Round 2 — Seviye A/oracle geri bildirim
├── translations_rust_levelB/    # Claude, Round 2 — Seviye B (orta ayrintili geri bildirim)
├── translations_rust_levelC/    # Claude, Round 2 — Seviye C (minimal geri bildirim)
├── translations_rust__gemini/   # Google Gemini cevirileri (57/57, gercek API cagrisi)
├── translations_rust__gpt4o/    # (bos - API anahtari yok, bkz. asagida)
├── translations_rust__deepseek/ # (bos - API anahtari yok, bkz. asagida)
├── harness/
│   ├── run_experiment.py    # Değerlendirme motoru (tek + çok dosyalı örnekleri destekler)
│   ├── make_figures.py      # Sonuçlardan figür üretir (results/figures/*.png)
│   ├── stats_report.py      # Bootstrap GA + Mann-Whitney duyarlilik analizi + Fisher GA + McNemar
│   ├── compare_platforms.py # Windows vs Linux/Docker sonuç karşılaştırması
│   ├── generate_translations.py  # Coklu-model ceviri ureticisi (Gemini/GPT-4o/DeepSeek)
│   ├── compare_models.py    # Modeller arası EA karşılaştırma tablosu (Faz 1.3'teki tek komut)
│   └── translators/         # Model-bağımsız Translator arayüzü + adaptörler
├── results/                 # Sonuç dosyaları (JSON/CSV) + figures/ (bkz. yukarida)
├── Dockerfile, docker-compose.yml  # Linux/LP64 ortamında tekrarlama icin
├── requirements.txt         # Sabitlenmiş Python bağımlılıkları
├── .env.example             # API anahtarı şablonu (gercek anahtar ASLA commit edilmez)
├── LICENSE                  # Bu projenin kendi kodu icin (MIT)
├── THIRD_PARTY_LICENSES.md  # Rosetta Code / BSD libc / musl / Redis / cJSON kaynak+lisans dokumu
├── MODIFICATIONS.md         # Hakem geri bildirimine yanıt günlüğü (tüm revizyon turları)
└── build/                   # Derlenen ikili dosyalar (otomatik, .gitignore'da)
```

## Kurulum

Üç şey gerekli: **Python 3**, **gcc** (C derleyici), **rustc** (Rust derleyici).

### Windows (makaledeki ana ölçüm ortamı — §III-E)
1. **Python 3:** https://www.python.org/downloads/ (kurulumda "Add to PATH" işaretle).
2. **gcc:** MSYS2 (https://www.msys2.org/) kur, ardından MSYS2 terminalinde:
   `pacman -S mingw-w64-ucrt-x86_64-gcc` — sonra `...\ucrt64\bin` klasörünü PATH'e ekle.
   (Alternatif: [w64devkit](https://github.com/skeeto/w64devkit).)
3. **Rust:** https://rustup.rs adresinden `rustup-init.exe` çalıştır.
4. Yeni bir terminal aç ve doğrula: `python --version`, `gcc --version`, `rustc --version`.

Makalede raporlanan gerçek sürümler: **gcc 16.1.0** (MSYS2/UCRT64, `-O2`),
**rustc/cargo 1.97.1**. Derleme her örnek için `rustc <dosya>.rs -o <çıktı>`
ile yapılır — **Cargo projesi değildir**, açık bir `--edition` bayrağı da
**kullanılmaz** (rustc'nin varsayılan edition'ı: 2015). Bu, `static_mut_refs`
gibi bazı lint'lerin şiddetini etkiler (bkz. §III-E) — tekrarlarken bu bayrağı
eklemeyin, aksi halde bazı örneklerin derleme sonucu değişebilir.

### macOS
```bash
xcode-select --install                 # gcc/clang sağlar
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # Rust
# Python zaten var; yoksa: brew install python
```

### Linux (Debian/Ubuntu)
```bash
sudo apt update && sudo apt install -y build-essential python3
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # Rust
```

Doğrulama (her platform):
```bash
python3 --version   # veya: python --version
gcc --version
rustc --version
pip install -r requirements.txt
```

## Tek komutla tekrar (5 dakika)

Temiz bir klon üzerinde, kurulumdan sonra:

```bash
python harness/compare_models.py
```

Bu komut Claude'un mevcut Round 1 sonucunu (`results/results_round1.json`)
okur, `translations_rust__gemini/` üzerinde `run_experiment.py`'yi çalıştırır
ve ikisini karşılaştıran bir tablo üretir. `translations_rust__gpt4o/` ve
`translations_rust__deepseek/` boş olduğundan (API anahtarı yok — bkz.
aşağıdaki "Çoklu-model" bölümü) bu iki model için `[atlandi] ... bulunamadi`
mesajı görürsünüz — bu **beklenen bir durumdur, hata değildir.**

**Beklenen çıktı** (`results/model_comparison.md`, gerçek ölçüm):

| Model | Kapsam | EA (örnek) | EA % | CE | RE | FE | NT |
|---|---|---|---|---|---|---|---|
| claude-sonnet-5 (referans, round1) | 57/57 | 40/57 | %70.18 | 1 | 4 | 12 | 0 |
| gemini | 57/57 | 51/57 | %89.47 | 4 | 0 | 2 | 0 |

## Makaledeki ana sayılar nerede üretiliyor?

| Makaledeki değer | Üreten dosya |
|---|---|
| Tablo III (40/57, 42/57, 57/57 — üç koşul) | `harness/run_experiment.py` (3 ayrı `--label` koşusu) |
| Tablo IV (betimsel özellikler) + Mann-Whitney/Fisher/bootstrap/McNemar | `harness/stats_report.py` → `results/stats_report.md` |
| Tablo VII (model × kategori kırılımı, 40/51) | `harness/compare_models.py` → `results/model_comparison.md` |
| Şekil 1-5 | `harness/make_figures.py` → `results/figures/*.png` |
| Platform farkı (Round 2: %100→%94.74, 54/57) | `harness/compare_platforms.py` → `results/platform_comparison.md` |
| Gemini çağrı kaydı (istem, zaman damgası, parametreler) | `harness/generate_translations.py` → `results/manifest_gemini.json` |

## Tam koşu (üç koşul + figürler + istatistikler)

```bash
python harness/run_experiment.py --label round1
python harness/run_experiment.py --release --label round1_release
python harness/run_experiment.py --rust-dir translations_rust_refined --label round2
python harness/make_figures.py
python harness/stats_report.py
```

Çıktılar:
- Terminalde özet tablo + EA + hata dağılımı.
- `results/results_<label>.json` ve `.csv`
- `results/figures/*.png` (Şekil 1-5, IEEE makalede kullanılanlarla birebir aynı)
- `results/stats_report.md` (bootstrap GA, duyarlılık analizi, Fisher, McNemar)

## Linux/LP64 ortamında tekrarlama (Docker)

```bash
docker compose build
docker compose run --rm experiment-linux   # 3 round'u konteyner icinde calistirir
python harness/compare_platforms.py        # Windows vs Linux karsilastirmasi
```

**Not:** Bu Docker tekrarı ilk olarak veri seti 55 örnekken (s56 ve s57
eklenmeden önce) yapılmıştı; 2026-07-30'da s56/s57 dahil **57 örneğin
tamamı** üzerinde yeniden çalıştırıldı ve `results/platform_comparison.md`
artık `n=57` üzerindendir (Round 2 Linux: %94.74, 54/57). Yeni eklenen
s56/s57 iki platformda da aynı sonucu verdi; platforma duyarlı üç örnek
(s38, s51, s47) değişmedi.

## Çoklu-model çeviri üretme (Gemini/GPT-4o/DeepSeek)

```bash
cp .env.example .env         # .env'e gerçek API anahtarınızı ekleyin (ASLA commit etmeyin)
python harness/generate_translations.py --model gemini
python harness/compare_models.py
```

Her çağrının tam istemi (prompt), model kimliği, zaman damgası ve sampling
parametreleri `results/manifest_<model>.json` içine kaydedilir (Gemini için:
`temperature=0.2`, `top_p=1.0`, model kimliği `gemini-flash-latest`, gerçek
erişim tarihleri 2026-07-22 – 2026-07-25). GPT-4o ve DeepSeek adaptörleri
yazılmıştır (`harness/translators/`) ama API anahtarı olmadığından hiç
çalıştırılmamıştır — bu, makalenin §V-B (Sınırlamalar) bölümünde açıkça
belirtilmiştir.

## Yeni örnek eklemek
1. `samples_c/sNN_ad.c` ekle (stdin'den oku, stdout'a yaz) — veya çok dosyalı
   bir örnek için `samples_c/sNN_ad/` altına `manifest.json` + kaynak dosyaları.
2. `tests/sNN_ad/01.txt, 02.txt ...` girdilerini ekle (referans çıktıyı harness,
   C'yi çalıştırarak kendisi üretir — beklenen çıktıyı elle yazmana gerek yok).
3. `translations_rust/sNN_ad.rs` çevirisini ekle.
4. Harness'i tekrar çalıştır.

## Metodolojik notlar

- **Diferansiyel test:** Hedef dilde önceden yazılmış birim testi olmadığından,
  eşdeğerlik kaynak programın çıktısıyla karşılaştırılarak otomatik doğrulanır.
- **Derleme modu önemlidir:** Rust'ta tamsayı taşması debug'da panic verir,
  release'de sarar. Deneyde her iki mod da raporlanmıştır.
- **Çeviri modelleri:** Claude Sonnet 5 (etkileşimli CLI oturumu, örnekleme
  parametreleri kayıt altına alınmamıştır) ve Google Gemini (gerçek API,
  parametreler `manifest_gemini.json`'da tam kayıtlıdır). Betik model-bağımsız
  tasarlanmıştır — `translations_rust__<model>/` klasörüne başka bir modelin
  çevirilerini koyup aynı ölçümü tekrarlayabilirsiniz.

## Lisans

Bu projenin kendi kodu (harness, özgün örnekler) `LICENSE` dosyasındaki MIT
lisansı altındadır. Veri setinin bir kısmı üçüncü taraf açık kaynak kod
içerir (Rosetta Code, OpenBSD/FreeBSD libc, musl, Redis, cJSON) — kaynak,
lisans ve orijinal telif bilgileri için `THIRD_PARTY_LICENSES.md`'e bakın.

## Atıf

Bu çalışmayı kullanırsanız lütfen atıfta bulunun:

```bibtex
@misc{baykara2026sessizhatalar,
  author       = {Baykara, Muhammet and {\c{S}}ahin, {\.I}brahim Halil and A{\c{s}}k{\i}n, Emre},
  title        = {{LLM Destekli Kod \c{C}evirisinde Sessiz Semantik Hatalar: C'den Rust'a \c{C}oklu-Model Deneysel Bir De{\u{g}}erlendirme}},
  year         = {2026},
  howpublished = {\url{https://github.com/ibrahimsahin022/llm_ceviri_deneyi}},
  note         = {F{\i}rat {\"U}niversitesi, Yaz{\i}l{\i}m M{\"u}hendisli{\u{g}}i}
}
```

## Tekrarlanabilirlik notu

Bootstrap/Monte Carlo istatistikleri (`stats_report.py`, `make_figures.py`
Şekil 4) sabit bir seed (42) kullanır ve **bit-bit aynı sayıları üretir**.
Ancak LLM API çağrıları (Gemini/GPT-4o/DeepSeek) — `temperature=0.2` gibi
düşük ve sabit bir değerde olsa dahi — sağlayıcı tarafında zamanla değişen
model sürümleri (`gemini-flash-latest` tarihli bir sürüm değil rotasyonlu bir
takma addır), sunucu-taraflı örnekleme gürültüsü veya API güncellemeleri
nedeniyle **bit-bit tekrarlanabilir değildir**; yalnızca *yaklaşık olarak*
tekrarlanabilir olduğu dürüstçe belirtilir. Claude Sonnet 5 çevirileri ise
bir CLI arayüzü üzerinden üretildiğinden (API değil), örnekleme parametreleri
tam olarak bilinmemektedir — bu, çalışmanın açıkça belirtilen bir
sınırlamasıdır (bkz. §III-B, §V-B).

# LLM Kod Çevirisi Deney Ortamı (C → Rust)

Bu proje, **eski kodların büyük dil modelleriyle (LLM) yeni bir dile çevrilirken
oluşan halüsinasyon ve mantık hatalarını** deneysel olarak ölçmek için hazırlanmış,
çalıştırılabilir bir test ortamıdır. Makalenin "Yöntem" ve "Bulgular" bölümlerinin
verisi bu ortamdan üretilir. Şu an **55 C programı** ve **219 test girdisi** içerir
(bunlardan 7'si — s30-s36 — Rosetta Code'dan alınmış eğitim amaçlı klasik
algoritmalar, 3'ü — s37-s39 — OpenBSD/FreeBSD libc'sinden alınmış gerçek üretim
(production) kodu, 11'i — s40-s45 ve s49-s53 — daha önce hedeflenmemiş veya
yalnızca birer örnekle temsil edilen C↔Rust boşluklarını (usize taşması, union,
bit-alanı, switch fallthrough, fonksiyon-lokal static, goto, char işaretliliği,
global durum, platform tamsayı genişliği) test eden özgün programlardır, 3'ü —
s46-s48 — musl libc, Redis ve cJSON gibi yaygın kullanılan gerçek açık kaynak
projelerinden alınmış, önceki örneklerden belirgin biçimde daha uzun/karmaşık
kod parçalarıdır (262-522 satır), 2'si — s54-s55 — çok dosyalı/gerçekçi C
proje yapısını (paylaşılan başlık dosyası, birden fazla derleme birimi) test
eder; ayrıntı için `results/VERISETI_VE_ALGORITMALAR.md`'e bakın).

Deney ayrıca **çoklu model** (Claude Sonnet 5 referans + Google Gemini ile
gerçek ölçüm, bkz. `harness/translators/`), **çok dosyalı kod** (bkz.
`samples_c/s54_stack_module/`, `s55_config_parser/`) ve **çoklu platform**
(Windows/LLP64 + Docker/Linux/LP64, bkz. `Dockerfile`,
`harness/compare_platforms.py`) boyutlarını da kapsayacak şekilde
genişletilmiştir — ayrıntı ve gerçek ölçülmüş sayılar için `MODIFICATIONS.md`'e
bakın.

Sonuç dosyaları (`results/` altında):
- `OZET_SONUCLAR.md` — makaleye hazır bulgular + hata analizi + "%100 nasıl çıkıyor" açıklaması.
- `VERISETI_VE_ALGORITMALAR.md` — kullanılan model/araçlar, veri seti kartı ve her programın algoritması.

## Ne yapıyor?
1. `samples_c/` içindeki C programlarını **referans (doğru)** kabul eder.
2. `translations_rust/` içindeki, aynı programların **LLM ile Rust'a çevrilmiş**
   hallerini alır.
3. Her ikisini de derleyip `tests/` içindeki girdilerle çalıştırır ve çıktıları
   **karşılaştırır** (diferansiyel test).
4. Farkları 4 hata türünde sınıflandırır: Derleme (CE), Çalışma Zamanı (RE),
   Sonlanmama (NT), Fonksiyonel (FE).
5. **Yürütme Doğruluğu (EA)** ve hata dağılımını hesaplayıp `results/` altına yazar.

## Klasör yapısı
```
llm_ceviri_deneyi/
├── samples_c/               # Kaynak (eski) programlar — C  [REFERANS]
├── translations_rust/       # LLM'in Rust çevirileri (Round 1 — doğrudan)
├── translations_rust_refined/  # İyileştirilmiş çeviriler (Round 2 — Seviye A/oracle geri bildirim)
├── translations_rust_levelB/   # Round 2, Seviye B (orta ayrıntılı, CI-benzeri geri bildirim)
├── translations_rust_levelC/   # Round 2, Seviye C (minimal geri bildirim)
├── tests/<ornek>/*.txt      # Her örnek için test girdileri
├── harness/
│   ├── run_experiment.py    # Değerlendirme motoru (tek + çok dosyalı örnekleri destekler)
│   ├── make_figures.py      # Sonuçlardan figür üretir
│   ├── stats_report.py      # Bootstrap GA + Mann-Whitney gücü + Fisher GA
│   ├── compare_platforms.py # Windows vs Linux/Docker sonuç karşılaştırması
│   ├── generate_translations.py  # Coklu-model ceviri ureticisi (Gemini/GPT-4o/DeepSeek)
│   ├── compare_models.py    # Modeller arası EA karşılaştırma tablosu
│   └── translators/         # Model-bağımsız Translator arayüzü + adaptörler
├── results/                 # Sonuç dosyaları (JSON/CSV) + figures/
│   └── OZET_SONUCLAR.md     # Makaleye hazır sonuç özeti
├── Dockerfile, docker-compose.yml  # Linux/LP64 ortamında tekrarlama icin
├── .github/workflows/ci-matrix.yml  # Windows+Linux CI matrisi (yazildi, push edilmedi)
├── requirements.txt         # Sabitlenmiş Python bağımlılıkları
├── .env.example             # API anahtarı şablonu (Faz 1, çoklu model)
├── MODIFICATIONS.md          # Hakem geri bildirimine yanıt günlüğü (5 faz)
└── build/                   # Derlenen ikili dosyalar (otomatik)
```

## Kurulum

Üç şey gerekli: **Python 3**, **gcc** (C derleyici), **rustc** (Rust derleyici).

### Windows
1. **Python 3:** https://www.python.org/downloads/ (kurulumda "Add to PATH" işaretle).
2. **gcc:** MSYS2 (https://www.msys2.org/) kur, ardından MSYS2 terminalinde:
   `pacman -S mingw-w64-ucrt-x86_64-gcc` — sonra `...\ucrt64\bin` klasörünü PATH'e ekle.
   (Alternatif: [w64devkit](https://github.com/skeeto/w64devkit).)
3. **Rust:** https://rustup.rs adresinden `rustup-init.exe` çalıştır.
4. Yeni bir terminal aç ve doğrula: `python --version`, `gcc --version`, `rustc --version`.

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
```

## Çalıştırma

Proje klasörünün içindeyken:

```bash
# 1) Round 1 — doğrudan (zero-shot) çeviriler, debug modu (varsayılan)
python3 harness/run_experiment.py --label round1

# 2) Round 2 — iyileştirilmiş çeviriler
python3 harness/run_experiment.py --rust-dir translations_rust_refined --label round2

# 3) Ek koşu — Round 1 çeviriler, RELEASE modu (taşma kontrolü kapalı)
python3 harness/run_experiment.py --release --label round1_release

# 4) Figürleri üret (yukarıdaki 3 koşu çalıştırıldıktan sonra)
python3 harness/make_figures.py
```
> Windows'ta komut `python` olabilir (`python3` yerine).

Çıktılar:
- Terminalde özet tablo + EA + hata dağılımı.
- `results/results_<label>.json` ve `.csv`
- `results/figures/*.png`

## Sonuçları nasıl yorumlamalıyım?
`results/OZET_SONUCLAR.md` dosyasına bak — orada üç koşulun karşılaştırması,
örnek bazında sonuçlar ve iki başarısızlığın kök neden analizi hazır olarak var.
Bunu doğrudan makalenin Bulgular/Tartışma bölümüne uyarlayabilirsin.

## İyileştirme (refinement) döngüsünü kendin nasıl işletirsin?
Bir örnek Round 1'de başarısız olduğunda:
1. `results/results_round1.json` içinde o örneğin `category` ve `stderr`/`got` alanına bak.
2. İlgili `.rs` dosyasını, hata mesajıyla birlikte Claude'a ver ve "bu hatayı düzelt" de.
3. Düzeltilmiş dosyayı `translations_rust_refined/` içine koy.
4. Round 2'yi tekrar çalıştır ve EA'nın nasıl değiştiğini kaydet.

## Yeni örnek eklemek
1. `samples_c/sNN_ad.c` ekle (stdin'den oku, stdout'a yaz).
2. `tests/sNN_ad/01.txt, 02.txt ...` girdilerini ekle (referans çıktıyı harness,
   C'yi çalıştırarak kendisi üretir — beklenen çıktıyı elle yazmana gerek yok).
3. `translations_rust/sNN_ad.rs` çevirisini ekle.
4. Harness'i tekrar çalıştır.

## Metodolojik notlar (makalede belirt)
- **Diferansiyel test:** Hedef dilde önceden yazılmış birim testi olmadığından,
  eşdeğerlik kaynak programın çıktısıyla karşılaştırılarak otomatik doğrulanır
  (FLUORINE çalışmasındaki yaklaşımla aynı mantık).
- **Derleme modu önemlidir:** Rust'ta tamsayı taşması debug'da panic verir,
  release'de sarar. Deneyde her iki mod da raporlanmıştır.
- **Çeviri modeli:** Bu çalışmada çeviriyi yapan LLM olarak Claude kullanılmıştır;
  betik model-bağımsızdır — istersen `translations_rust/` içine başka bir modelin
  çevirilerini koyup aynı ölçümü tekrarlayabilirsin.

## Nasıl Tam Olarak Tekrarlanır

Bu bölüm, hakem geri bildirimindeki "tekrarlanabilirlik" zayıflığına yanıt
olarak eklenmiştir (Faz 5). Tekrarlamak için gereken her şey:

### Araç sürümleri (gerçekten kullanılan, doğrulanmış)

| Bileşen | Windows (ana ortam) | Linux/Docker (Faz 4 doğrulaması) |
|---|---|---|
| İşletim sistemi | Windows 11 Pro | Ubuntu 24.04 (`ubuntu:24.04` imajı) |
| C derleyici | gcc 16.1.0 (MSYS2/UCRT64) | gcc 13.3.0 (Ubuntu 13.3.0-6ubuntu2~24.04.1) |
| Rust derleyici | rustc 1.97.1 (stable, commit 8bab26f4f) | rustc 1.97.1 (stable, **aynı commit**) |
| `long` genişliği (ABI) | 32-bit (LLP64) | 64-bit (LP64) |
| Python | 3.14.6 | 3.12.3 (container apt) |

Python bağımlılıkları `requirements.txt` içinde sabitlenmiştir:
```bash
pip install -r requirements.txt
```

### Kullanılan model kimlikleri ve erişim tarihleri

| Model | Tam kimlik | Erişim | Sıcaklık/top_p | Not |
|---|---|---|---|---|
| Claude Sonnet 5 | `claude-sonnet-5` | Bu projenin ana çalışması boyunca (2026), Claude Code CLI arayüzü | Arayüz varsayılanları (bkz. §3.2, tam olarak bilinmiyor) | `translations_rust/`, `translations_rust_refined/` — 55 örneğin tamamı |
| Google Gemini | `gemini-flash-latest` (API yanıtına göre `gemini-3.6-flash`'e çözümleniyor) | 2026-07-22, Google AI Studio ücretsiz katmanı, `harness/translators/gemini_translator.py` üzerinden gerçek API çağrısı | `temperature=0.2`, `top_p=1.0` (sabit, kod içinde) | Yalnızca 19/48 örnek — günlük kota sınırı (20 istek/gün/model) |
| OpenAI GPT-4o | `gpt-4o` | **Hiç çağrılmadı** (API anahtarı yok) | — | Yalnızca `--dry-run` ile istem inşası doğrulandı |
| DeepSeek | `deepseek-chat` | **Hiç çağrılmadı** (API anahtarı yok) | — | Yalnızca `--dry-run` ile istem inşası doğrulandı |

Çoklu-model çeviri üretmek isterseniz:
```bash
cp .env.example .env         # .env'e gerçek API anahtarınızı ekleyin
pip install -r requirements.txt
python harness/generate_translations.py --model gemini --sleep 4
python harness/compare_models.py
```
Her çağrının tam istemi (prompt), model kimliği, zaman damgası ve sampling
parametreleri `results/manifest_<model>.json` içine kaydedilir.

### Linux/LP64 ortamında tekrarlama (Faz 4)
```bash
docker compose build
docker compose run --rm experiment-linux   # 3 round'u konteyner icinde calistirir
python harness/compare_platforms.py        # Windows vs Linux karsilastirmasi
```

### Tam komut listesi (baştan sona)
```bash
pip install -r requirements.txt
python harness/run_experiment.py --label round1
python harness/run_experiment.py --release --label round1_release
python harness/run_experiment.py --rust-dir translations_rust_refined --label round2
python harness/make_figures.py
python harness/stats_report.py
docker compose build && docker compose run --rm experiment-linux
python harness/compare_platforms.py
```

### Önemli tekrarlanabilirlik notu
Bootstrap/Monte Carlo istatistikleri (`stats_report.py`, `make_figures.py`
Şekil 4b) sabit bir seed (42) kullanır ve **bit-bit aynı sayıları üretir**.
Ancak LLM API çağrıları (Gemini/GPT-4o/DeepSeek) — `temperature=0.2` gibi
düşük ve sabit bir değerde olsa dahi — sağlayıcı tarafında zamanla değişen
model sürümleri, sunucu-taraflı örnekleme gürültüsü veya API güncellemeleri
nedeniyle **bit-bit tekrarlanabilir değildir**; yalnızca *yaklaşık olarak*
tekrarlanabilir olduğu dürüstçe belirtilir. Claude Sonnet 5 çevirileri ise
bir CLI arayüzü üzerinden üretildiğinden (API değil), örnekleme parametreleri
tam olarak bilinmemektedir — bu, çalışmanın açıkça belirtilen bir sınırlamasıdır
(bkz. §3.2, §6).

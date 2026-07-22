# LLM Kod Çevirisi Deney Ortamı (C → Rust)

Bu proje, **eski kodların büyük dil modelleriyle (LLM) yeni bir dile çevrilirken
oluşan halüsinasyon ve mantık hatalarını** deneysel olarak ölçmek için hazırlanmış,
çalıştırılabilir bir test ortamıdır. Makalenin "Yöntem" ve "Bulgular" bölümlerinin
verisi bu ortamdan üretilir. Şu an **48 C programı** ve **185 test girdisi** içerir
(bunlardan 7'si — s30-s36 — Rosetta Code'dan alınmış eğitim amaçlı klasik
algoritmalar, 3'ü — s37-s39 — OpenBSD/FreeBSD libc'sinden alınmış gerçek üretim
(production) kodu, 6'sı — s40-s45 — daha önce hedeflenmemiş C↔Rust boşluklarını
(usize taşması, union, bit-alanı, switch fallthrough, fonksiyon-lokal static,
goto) test eden yeni özgün programlardır, 3'ü — s46-s48 — musl libc, Redis ve
cJSON gibi yaygın kullanılan gerçek açık kaynak projelerinden alınmış, önceki
örneklerden belirgin biçimde daha uzun/karmaşık kod parçalarıdır (262-522
satır); ayrıntı için `results/VERISETI_VE_ALGORITMALAR.md`'e bakın).

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
│   ├── run_experiment.py    # Değerlendirme motoru
│   └── make_figures.py      # Sonuçlardan figür üretir
├── results/                 # Sonuç dosyaları (JSON/CSV) + figures/
│   └── OZET_SONUCLAR.md     # Makaleye hazır sonuç özeti
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

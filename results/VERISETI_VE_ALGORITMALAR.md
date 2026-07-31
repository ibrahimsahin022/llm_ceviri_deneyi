# Veri Seti, Modeller ve Algoritmalar (Yöntem Bölümü İçin)

Bu belge, deneyde **neyin kullanıldığını** açıkça listeler: veri seti, çeviriyi
yapan model, derleyiciler, değerlendirme yöntemi ve her örnek programın algoritması.

---

## 1. Kullanılan Modeller ve Araçlar

| Bileşen | Ne kullanıldı | Rol |
|---|---|---|
| **Çeviri modeli (LLM)** | **Claude Sonnet 5** (model kimliği: `claude-sonnet-5`) | C kodunu tek geçişte (zero-shot) Rust'a çevirir |
| Kaynak derleyici | `gcc` (C, `-O2`) | Referans (ground-truth) programı üretir |
| Hedef derleyici | `rustc` (debug: `overflow-checks=on`; release: `-O`) | Çeviriyi derler |
| Değerlendirme | Python 3 harness (`run_experiment.py`) | Diferansiyel test + hata sınıflandırma |
| İstatistik | Python + scipy (Mann-Whitney U, Fisher'in kesin testi) | Kod uzunluğu/özellik-başarı ilişkisi anlamlılık testi |
| Görselleştirme | Python + matplotlib | Figürler |

> Not 1: Model sürümü kasıtlı olarak tam belirtilmiştir (tekrarlanabilirlik için).
> Bulgular yalnızca Claude Sonnet 5'e özgüdür; "LLM'ler genel olarak" biçiminde
> genellenmemelidir.
>
> Not 2: İstem (prompt) mühendisliği en yalın haliyle uygulanmıştır: modele her
> C dosyası için yalnızca "bu C programını Rust'a çevir" doğrultusunda doğrudan
> bir istek verilmiş; few-shot örnek, ek kısıtlama veya belirli bir Rust
> deyimselliği talebi verilmemiştir. Model, çeviriyi üretmeden önce kodu derleme
> veya test etme imkanına sahip değildi (gerçek zero-shot koşulu).
>
> Not 3: Betik **model-bağımsızdır**. `translations_rust/` klasörüne başka bir
> modelin (GPT, Gemini, DeepSeek, Qwen vb.) çevirilerini koyup aynı ölçümü
> tekrarlayabilirsin — ancak bu, yöntemin uygulanabilirliğini gösterir, farklı
> modellerin AYNI sonuçları vereceği anlamına gelmez.

## 2. Veri Seti: "C→Rust Legacy Çeviri Benchmark'ı" (özgün)

- **Boyut:** 57 program, 233 test girdisi, kaynak kod uzunluğu 10–522 satır (C).
  (İlk sürüm 24 program / 10–88 satırdı; kod uzunluğu ile başarı ilişkisini daha
  güçlü test etmek için 69–141 satır arasında 5 yeni program eklendi: s25–s29;
  ardından veri setinin tamamen kendi yazdığımız kodlardan oluşmadığını göstermek
  için gerçek dünyadan (eğitim amaçlı) 7 program eklendi: s30–s36; ardından
  "Rosetta Code çok temiz/eğitici" eleştirisini gidermek için OpenBSD/FreeBSD
  libc'sinden gerçek ÜRETİM (production) kodu 3 program eklendi: s37–s39; sonra
  hedeflenmemiş C↔Rust boşluklarını sınamak için 6 özgün program eklendi:
  s40–s45; ardından "gerçek kod ama hâlâ kısa" eleştirisini gidermek için
  musl libc, Redis ve cJSON gibi çok yaygın kullanılan gerçek açık kaynak
  projelerinden, önceki tüm örneklerden belirgin biçimde daha uzun/karmaşık
  3 program eklendi: s46–s48, 262–522 satır; hakem geri bildirimi doğrultusunda
  istatistiksel gücü artırmak için tek örnekle temsil edilen dört kök-neden
  kategorisini ikinci birer örnekle güçlendiren 5 program eklendi: s49–s53;
  son olarak çok dosyalı kod (s54, s55), karmaşık makro (s56) ve gerçek
  paylaşılan bellek eşzamanlılığı (s57) için 4 program daha eklendi.)
- **Kaynak (s01-s29, 29 program):** Tarafımızca yazıldı; klasik algoritma ve
  eski-kod (legacy) desenlerinden (hash fonksiyonları, matris işlemleri, string
  işleme, ayrıştırıcı, sayısal algoritmalar, dinamik veri yapıları) türetildi.
- **Kaynak (s30-s36, 7 program):** **Tarafımızca yazılmadı** — [Rosetta
  Code](https://rosettacode.org)'dan alınmış eğitim amaçlı gerçek dünya
  algoritmalarıdır (GFDL 1.2 / CC-BY-SA lisanslı, atıfla yeniden kullanıma
  açık), `acmeism/RosettaCodeData` GitHub aynası üzerinden
  (`raw.githubusercontent.com`) erişilmiştir. Her programın **çekirdek algoritma
  fonksiyonu kaynaktan değiştirilmeden alınmıştır**; yalnızca `main()` bu ortamın
  stdin/stdout sözleşmesine uyacak şekilde yeniden yazılmıştır (orijinal
  kaynaklarda çoğunlukla sabit kodlanmış örnek girdiler vardı). Kaynak URL'si ve
  nelerin değiştirildiği her `.c` dosyasının başında yorum olarak belirtilmiştir:

  | Örnek | Algoritma | Kaynak URL |
  |---|---|---|
  | s30_luhn_check | Luhn sağlama toplamı | rosettacode.org/wiki/Luhn_test_of_credit_card_numbers |
  | s31_soundex | Soundex fonetik kodlama | rosettacode.org/wiki/Soundex |
  | s32_levenshtein | Levenshtein düzenleme mesafesi | rosettacode.org/wiki/Levenshtein_distance |
  | s33_knapsack | 0/1 sırt çantası (DP) | rosettacode.org/wiki/Knapsack_problem/0-1 |
  | s34_hanoi | Hanoi kuleleri | rosettacode.org/wiki/Towers_of_Hanoi |
  | s35_lcs | En uzun ortak alt dizi (DP) | rosettacode.org/wiki/Longest_common_subsequence |
  | s36_crc32 | CRC-32 (tablo tabanlı) | rosettacode.org/wiki/CRC-32 |

- **Kaynak (s37-s39, 3 program) — YENİ:** **Tarafımızca yazılmadı**, ve Rosetta
  Code'un aksine eğitim amaçlı değil **gerçek üretim (production) işletim
  sistemi kütüphane (libc) kodu**dur — OpenBSD/FreeBSD projelerinden, BSD-3-Clause
  lisanslı, 32-39 yıllık ve hâlâ fiilen kullanılan fonksiyonlar:

  | Örnek | Fonksiyon | Kaynak | Yıl |
  |---|---|---|---|
  | s37_bsd_getopt | `getopt()` — komut satırı seçenek ayrıştırıcı | github.com/freebsd/freebsd-src (lib/libc/stdlib/getopt.c) | 1987 |
  | s38_bsd_strtol | `strtol()` — dize→tamsayı dönüştürücü | github.com/openbsd/src (lib/libc/stdlib/strtol.c) | 1990 |
  | s39_bsd_heapsort | `heapsort()` — generic void* yığın sıralaması | github.com/openbsd/src (lib/libc/stdlib/heapsort.c) | 1991 |

  Çekirdek fonksiyon mantığı (ve heapsort'un SWAP/COPY/CREATE/SELECT makroları)
  kaynaktan hiçbir değişiklik yapılmadan alınmıştır. Sonuç: 2/3 (getopt,
  heapsort) Round 1'de ilk seferde geçti; 1/3 (strtol) yeni, önceden
  öngörülmemiş bir kök nedenden (platforma bağlı tamsayı genişliği, bkz. §3
  Kök Neden F) başarısız oldu.

- **Kaynak (s46-s48, 3 program) — YENİ:** **Tarafımızca yazılmadı**; s37-s39'daki
  "gerçek ama kısa" sınırını aşmak için, GitHub'da çok yaygın kullanılan (her
  biri binlerce bağımlı projeye sahip), önceki tüm örneklerden belirgin biçimde
  daha uzun/karmaşık gerçek kod tabanlarından alınmıştır. Her dosyanın başında
  kaynak URL'si, lisans ve tam olarak neyin değiştirildiği (yalnızca dış
  bağımlılık/altyapı katmanları — algoritmanın kendisi değil) belgelenmiştir:

  | Örnek | Kaynak proje | Fonksiyon/modül | Lisans | LoC |
  |---|---|---|---|---|
  | s46_musl_qsort | musl libc | Smoothsort (`__qsort_r`, Leonardo sayılarıyla adaptif heapsort) | MIT | 262 |
  | s47_redis_sds | Redis 7.2.4 | SDS — değişken genişlikte başlıklı dinamik string kütüphanesi | BSD-3-Clause | 522 |
  | s48_cjson_number | cJSON | `parse_number`/`print_number` — round-trip garantili `%g` sayı yazdırma | MIT | 389 |

  s46, musl'un gerçek üretim ortamlarında (ör. Alpine Linux) kullanılan
  qsort() uygulamasıdır — bit-düzeyinde Leonardo-sayı kodlamasıyla çalışan,
  veri setindeki en karmaşık tek algoritmadır. s47, Redis'in dahili string
  temsilidir; başlık bilgisini (len/alloc/flags) pointer'ın hemen öncesinde
  gizli tutan, C'ye özgü bir bellek düzeni kullanır. s48, mevcut "Çıktı
  biçimlendirme (%g)" kök nedenini (bkz. Kök Neden D) tamamen bağımsız,
  yaygın kullanılan başka bir kod tabanında yeniden sınar. Sonuç: 2/3 (s46,
  s47) Round 1'de ilk seferde geçti — s47 özellikle dikkat çekicidir çünkü
  veri setindeki EN UZUN program olmasına rağmen (522 satır) ilk seferde
  sorunsuz geçmiştir; 1/3 (s48) mevcut %g kök nedeninden başarısız oldu, bu
  da Kök Neden D'nin tek bir kod tabanına özgü bir tuhaflık olmadığını,
  sistematik bir C↔Rust boşluğu olduğunu güçlendirir.

- Test girdileri elle seçildi; **referans çıktılar, C programı çalıştırılarak
  otomatik üretilir** (elle beklenen çıktı yazılmaz).
- **Tasarım ilkesi:** Kolaydan zora, ASCII'den çok baytlı (UTF-8/Türkçe) girdiye kadar
  çeşitlilik; ayrıca C ile Rust arasında bilinen semantik boşlukları (tamsayı taşması,
  string modeli, işaretlilik, biçimlendirme, global durum) hedefleyen örnekler.
- **Literatürle ilişki:** Yaklaşım, CodeNet tabanlı bir veri setinde execution-guided
  refinement kullanan Gandhi vd. (2024, LLM4Code'24, doi:10.1145/3643795.3648388) ve
  GitHub projelerinden örnek çıkaran FLUORINE/Eniser vd. (2024, arXiv:2405.11514) ile
  aynı ailedendir; ancak bu benchmark **özgündür ve tekrarlanabilir** (tüm kaynaklar
  pakette). Tam bibliyografik bilgi için makalenin Kaynakça bölümüne bakınız.

### Örnek Programların Kataloğu (algoritma + boyut + rol)

| # | Örnek | Uygulanan algoritma / desen | LoC | Test | Rol (hedeflenen olgu) |
|---|---|---|---|---|---|
| 1 | s01_sum | Dizi toplama | 16 | 4 | Temel doğrulama |
| 2 | s02_gcd | Öklid EBOB | 19 | 4 | Negatif/modulo davranışı |
| 3 | s03_factorial | Faktöriyel (u64) | 14 | 4 | Tamsayı sınırı |
| 4 | s04_fibonacci | İteratif Fibonacci (u64) | 16 | 4 | Büyük değer |
| 5 | s05_count_primes | Deneme bölmesiyle asallık | 22 | 4 | Döngü/koşul |
| 6 | s06_reverse_string | Dize ters çevirme | 19 | 4 | **String modeli (bayt/karakter)** |
| 7 | s07_bubble_sort | Kabarcık sıralama | 26 | 4 | Dizi/takas |
| 8 | s08_binary_search | İkili arama | 22 | 3 | İndeks aritmetiği |
| 9 | s09_djb2_hash | djb2 karma fonksiyonu | 17 | 3 | **Unsigned taşma** |
| 10 | s10_caesar_cipher | Caesar şifreleme (modüler) | 23 | 3 | Karakter aritmetiği |
| 11 | s11_collatz | Collatz adım sayımı | 17 | 4 | Döngü, büyük ara değer |
| 12 | s12_matrix_mult | Matris çarpımı | 36 | 2 | İç içe döngü |
| 13 | s13_word_count | Kelime/karakter sayımı (wc) | 19 | 4 | **Karakter/bayt sayımı** |
| 14 | s14_fnv_hash | FNV-1a karma fonksiyonu | 17 | 3 | **Unsigned taşma (2. örnek)** |
| 15 | s15_float_avg | Ortalama + `%g` biçim | 17 | 4 | **Float biçimlendirme** |
| 16 | s16_rle_encode | Çalışma-uzunluğu kodlama | 22 | 3 | String tarama |
| 17 | s17_determinant | Laplace açılımı (özyineleme) | 35 | 4 | Orta boy, özyineleme |
| 18 | s18_expr_eval | Özyinelemeli inişli ifade ayrıştırıcı | 88 | 6 | Orta-uzun, özyineleme |
| 19 | s19_global_counter | Global sayaç (durumlu fonksiyon) | 21 | 3 | **Global durum → static mut (CE)** |
| 20 | s20_char_sum | Bayt değerleri toplamı | 17 | 3 | **char işaretliliği** |
| 21 | s21_matrix_transpose | Matris transpozu | 20 | 3 | Orta boy, indeksleme |
| 22 | s22_gray_code | Gray kodu (n ^ n>>1) | 10 | 4 | Bit işlemleri |
| 23 | s23_histogram | Harf frekansı histogramı | 18 | 3 | Sayaç dizisi |
| 24 | s24_roman | Tamsayı → Roma rakamı (açgözlü) | 19 | 4 | String kurma |
| 25 | s25_linked_list_ops | Bağlı liste kurma/ters çevirme | 87 | 4 | Uzun program, dinamik bellek/pointer |
| 26 | s26_rpn_calculator | Ters-Polonya (RPN) hesap makinesi | 69 | 5 | String tokenizasyonu + yığın (stack) |
| 27 | s27_csv_stats | CSV ayrıştırma + istatistik | 73 | 3 | **Çıktı biçimlendirme (%g), 2. örnek** |
| 28 | s28_bst_traversal | İkili arama ağacı (BST) kurma/dolaşma | 80 | 4 | Özyineleme + dinamik bellek |
| 29 | s29_hashtable_cmds | Zincirlemeli hash tablosu + komutlar | 141 | 3 | Uzun program, string hashleme |
| 30 | s30_luhn_check | Luhn sağlama toplamı (gerçek dünya, Rosetta Code) | 34 | 4 | Gerçek dünya kodu |
| 31 | s31_soundex | Soundex fonetik kodlama (gerçek dünya, Rosetta Code) | 71 | 4 | Gerçek dünya kodu, karakter dizisi |
| 32 | s32_levenshtein | Levenshtein mesafesi, özyinelemeli (gerçek dünya, Rosetta Code) | 50 | 4 | Gerçek dünya kodu, üstel özyineleme |
| 33 | s33_knapsack | 0/1 sırt çantası, DP (gerçek dünya, Rosetta Code) | 71 | 3 | Gerçek dünya kodu, dinamik programlama |
| 34 | s34_hanoi | Hanoi kuleleri (gerçek dünya, Rosetta Code) | 27 | 4 | Gerçek dünya kodu, özyineleme |
| 35 | s35_lcs | En uzun ortak alt dizi, DP (gerçek dünya, Rosetta Code) | 67 | 4 | Gerçek dünya kodu, dinamik programlama |
| 36 | s36_crc32 | CRC-32, tablo tabanlı (gerçek dünya, Rosetta Code) | 61 | 3 | Gerçek dünya kodu, bit/unsigned işlemler |
| 37 | s37_bsd_getopt | getopt(), seçenek ayrıştırıcı (gerçek üretim kodu, OpenBSD/FreeBSD) | 148 | 5 | Gerçek üretim kodu, global durum |
| 38 | s38_bsd_strtol | strtol(), dize→tamsayı (gerçek üretim kodu, OpenBSD) | 154 | 4 | **Tamsayı genişliği (platform, FE)** |
| 39 | s39_bsd_heapsort | heapsort(), generic void* sıralama (gerçek üretim kodu, OpenBSD) | 143 | 4 | Gerçek üretim kodu, void* işaretçi aritmetiği |
| 40 | s40_diff_sum | Ardışık fark toplamı (özgün) | 27 | 4 | **usize altında taşma (yeni, RE)** |
| 41 | s41_float_bits | Float→bit örüntüsü, union (özgün) | 21 | 4 | Union/type punning (to_bits) |
| 42 | s42_bitfields | Bit-alanı kırpma (özgün) | 25 | 4 | C bit-alanları (bit-fields) |
| 43 | s43_switch_fallthrough | Switch/case fallthrough (özgün) | 25 | 5 | **Kontrol akışı düşmesi (yeni, FE)** |
| 44 | s44_fib_memo_static | Fonksiyon-lokal static memoizasyon (özgün) | 29 | 4 | Fonksiyon-kapsamlı kalıcı durum (unsafe) |
| 45 | s45_goto_cleanup | goto ile kaynak temizleme (özgün) | 43 | 4 | Kontrol akışı yeniden yapılandırma (RAII) |
| 46 | s46_musl_qsort | Smoothsort, adaptif heapsort (gerçek üretim kodu, musl libc) | 262 | 5 | Gerçek üretim kodu, bit-düzeyi işaretçi aritmetiği (unsafe) |
| 47 | s47_redis_sds | SDS dinamik string kütüphanesi (gerçek üretim kodu, Redis) | 522 | 5 | **En uzun program (veri setinde)**, gizli başlık/pointer düzeni |
| 48 | s48_cjson_number | Sayı ayrıştırma/yazdırma, round-trip garantili `%g` (gerçek üretim kodu, cJSON) | 389 | 5 | **Çıktı biçimlendirme (%g), 3. örnek (FE)** |
| 49 | s49_negative_byte_count | Bayt değerleri toplamı, 2. desen (özgün) | 27 | 5 | **char işaretliliği, 2. örnek (FE)** |
| 50 | s50_id_generator | Ardışık kimlik üretici, global durum (özgün) | 30 | 4 | Güvensiz global durum, 2. örnek (PASS) |
| 51 | s51_long_clamp | `long` aralık sınırlama (özgün) | 33 | 5 | **Tamsayı genişliği, 2. örnek (FE)** |
| 52 | s52_window_sum | Kayan pencere toplamı (özgün) | 33 | 5 | **usize altında taşma, 2. örnek (RE)** |
| 53 | s53_tax_bracket | Kademeli vergi dilimi hesabı (özgün) | 31 | 5 | **Switch fallthrough, 2. örnek (FE)** |
| 54 | s54_stack_module | Yığın (stack) modülü — başlık+uygulama+kullanım (özgün, çok dosyalı) | 125 | 5 | Çok dosyalı derleme, paylaşılan başlık (PASS) |
| 55 | s55_config_parser | Paylaşılan struct + 2 derleme birimi (özgün, çok dosyalı) | 139 | 5 | Çok dosyalı derleme, paylaşılan veri yapısı (PASS) |
| 56 | s56_macro_table | X-Macro token-pasting + makro çoklu-değerlendirme (özgün) | 97 | 5 | **Makro çoklu-değerlendirme yan etkisi (FE)** |
| 57 | s57_shared_counter_threads | N pthread + mutex korumalı paylaşılan sayaç (özgün, çok dosyalı) | 71 | 5 | Gerçek paylaşılan bellek eşzamanlılığı (PASS) |

Not: s40-s45, madde 1 (küçük örneklem) zayıflığını gidermek için eklenen 6 yeni
özgün programdır; ikisi (s40, s43) daha önce hiç öngörülmemiş, gerçekten yeni
kök nedenler (G: usize taşması, H: switch fallthrough) ortaya çıkarmıştır.
s46-s48 ise "gerçek kod ama hâlâ kısa" eleştirisini gidermek için eklenen,
GitHub'da yaygın kullanılan gerçek açık kaynak projelerden (musl libc, Redis,
cJSON) alınmış, önceki tüm örneklerden belirgin biçimde daha uzun/karmaşık 3
programdır (262-522 satır); ayrıntı için yukarıdaki "Kaynak (s46-s48)" alt
bölümüne bakınız. s49-s53, o ana kadar yalnızca birer örnekle temsil edilen
dört kök-neden kategorisini (C, F, G, H) ikinci, bağımsız birer örnekle
güçlendirir (istatistiksel güç için). s54/s55/s57 çok dosyalı derlemeyi
(`manifest.json` ile birden fazla `.c` kaynağı) test eder; s56 karmaşık
önişlemci (preprocessor) kullanımını sınar.

## 3. Deneyin Kendi Algoritması (Yöntem)

### 3.1 Diferansiyel test (eşdeğerlik doğrulama)
Hedef dilde (Rust) hazır birim test olmadığından, çevirinin doğruluğu **kaynak
programla aynı girdide çalıştırılıp çıktı karşılaştırılarak** ölçülür:

```
her örnek için:
    C_ikili   = derle(C_kaynak)                 # referans
    Rust_ikili = derle(Rust_çeviri)             # derlenmezse -> CE
    her test girdisi için:
        beklenen = çalıştır(C_ikili, girdi)
        alınan   = çalıştır(Rust_ikili, girdi)  # zaman aşımı -> NT ; çökme -> RE
        eğer alınan != beklenen -> FE
        değilse -> PASS
    örnek doğrulugu = tüm girdiler PASS ise 1, değilse 0
```

### 3.2 Hata taksonomisi (literatürdeki 4 tür)
- **CE** Derleme Hatası — Rust kodu derlenmez.
- **RE** Çalışma Zamanı Hatası — panik/çökme, sıfır olmayan çıkış.
- **NT** Sonlanmama — zaman aşımı (varsayılan 5 sn).
- **FE** Fonksiyonel Hata — derlenir ve çalışır ama çıktı referanstan farklıdır.

### 3.3 İyileştirme (refinement) döngüsü
Round 1'de başarısız olan her örnek için hata geri bildirimi (derleyici mesajı,
panik metni veya beklenen-alınan farkı) modele verilir; model çeviriyi düzeltir ve
düzeltme `translations_rust_refined/` altına konur (Round 2). Bu, literatürdeki
"execution-guided / feedback-based" iyileştirme yaklaşımının (ACM'nin LRM'i,
FLUORINE'in fuzzer geri bildirimi) küçük ölçekli bir uygulamasıdır.

### 3.4 Metrik
**Yürütme Doğruluğu (Execution Accuracy, EA)** = (tüm test girdilerini geçen örnek
sayısı) / (toplam örnek sayısı). Ayrıca test-girdisi bazında ve hata-türü bazında
dağılımlar raporlanır. Derleme modu (debug/release) her zaman belirtilir.

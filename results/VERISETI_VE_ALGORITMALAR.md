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

- **Boyut:** 130 program, 521 test girdisi, kaynak kod uzunluğu 10–522 satır (C).
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
  paylaşılan bellek eşzamanlılığı (s57) için 4 program daha eklendi; en son
  olarak veri setini istatistiksel olarak daha güçlü ve çeşitli hale getirmek
  için 73 yeni örnek (s58–s130) üç grup halinde eklendi: mevcut A–I kök-neden
  kategorilerinin her birini üçer yeni, çoğunlukla özgün örnekle derinleştiren
  27 program (s58–s84); SQLite, zlib, curl, Redis, OpenSSL, libsodium,
  OpenBSD/FreeBSD libc, nginx, musl libc, cJSON ve Apache HTTP Server gibi
  izin verici lisanslı (GPL içermeyen) gerçek açık kaynak ÜRETİM kodundan
  alınmış 25 program (s85–s109); ve genel çeşitlilik, çok dosyalı derleme ile
  gerçek pthread eşzamanlılığını genişletmek için 21 program (s110–s130).)
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

- **Kaynak (s85-s109, 25 program) — YENİ:** **Tarafımızca yazılmadı**; s37-s39
  ve s46-s48'in "gerçek üretim kodu" ilkesini, GitHub'da milyonlarca kurulumda
  fiilen kullanılan 11 farklı açık kaynak projeye (SQLite, zlib, curl, Redis,
  OpenSSL, libsodium, OpenBSD/FreeBSD libc, nginx, musl libc, cJSON, Apache
  HTTP Server) yayarak genişletir. Tümü **izin verici lisanslı**dır (Public
  Domain, MIT, BSD-2/3-Clause, Apache-2.0, ISC-benzeri, zlib License) —
  `THIRD_PARTY_LICENSES.md` ile tutarlılık gereği **GPL kaynak
  kullanılmamıştır**. Her dosyanın başında kaynak URL'si, lisans ve (varsa)
  değiştirilen kısım (yalnızca dış bağımlılık/altyapı katmanı — algoritmanın
  kendisi değil) belgelenmiştir:

  | Örnek | Kaynak proje | Fonksiyon/modül | Lisans | LoC |
  |---|---|---|---|---|
  | s85_sqlite_stricmp | SQLite | `sqlite3_stricmp`/`sqlite3StrICmp` — ASCII büyük/küçük harf duyarsız karşılaştırma | Public Domain | 65 |
  | s86_sqlite_strglob | SQLite | `patternCompare` — GLOB desen eşleştirme motoru | Public Domain | 185 |
  | s87_sqlite_utf8_read | SQLite | `sqlite3Utf8Read` + `sqlite3Utf8Trans1[]` — UTF-8 kod noktası çözümleme | Public Domain | 54 |
  | s88_zlib_adler32_real | zlib | `adler32_z`/`adler32` — NMAX bloklamalı Adler-32 | zlib License | 86 |
  | s89_zlib_crc32_table | zlib | `crc32` — tablo üretimi + bayt-bazlı güncelleme | zlib License | 61 |
  | s90_curl_urldecode | curl | `Curl_urldecode` — yüzde-kod (percent-encoding) çözme | curl/MIT-benzeri | 82 |
  | s91_redis_ll2string | Redis | `ll2string` — `long long` → metin dönüşümü | BSD-3-Clause | 71 |
  | s92_openssl_base64_encode | OpenSSL | `evp_encodeblock_int`/`EVP_EncodeBlock` — Base64 kodlama | Apache-2.0 | 60 |
  | s93_libsodium_bin2hex | libsodium | `sodium_bin2hex` — sabit-zamanlı (dallanmasız) hex kodlama | ISC | 58 |
  | s94_freebsd_reallocarray | OpenBSD | `reallocarray` — çarpımsal taşma korumalı yeniden ayırma | ISC-benzeri | 40 |
  | s95_redis_stringmatchlen | Redis | `stringmatchlen_impl`/`stringmatchlen` — `KEYS` komutunun glob eşleyicisi | BSD-3-Clause | 152 |
  | s96_cjson_print_string | cJSON | `print_string_ptr` — JSON string kaçışlama | MIT | 104 |
  | s97_cjson_hex4_unicode | cJSON | `parse_hex4` — `\uXXXX` unicode kaçış çözümü | MIT | 42 |
  | s98_musl_memmem | musl libc | `memmem` — twobyte/threebyte/fourbyte hızlı yollar + Two-Way dize eşleştirme | MIT | 158 |
  | s99_musl_strsep | musl libc | `strsep` | MIT | 35 |
  | s100_musl_strverscmp | musl libc | `strverscmp` — sürüm dizesi karşılaştırma | MIT | 44 |
  | s101_freebsd_strlcpy | FreeBSD/OpenBSD | `strlcpy` — güvenli NUL-sonlandırmalı kopyalama | BSD-3-Clause | 49 |
  | s102_freebsd_strnstr | FreeBSD | `strnstr` — sınırlı alt-dize arama | BSD-3-Clause | 47 |
  | s103_nginx_hextoi | nginx | `ngx_hextoi` — onaltılık ayrıştırma, cutoff taşma kontrolü | BSD-2-Clause | 63 |
  | s104_musl_strcasestr | musl libc | `strcasestr` | MIT | 32 |
  | s105_musl_memrchr | musl libc | `memrchr` | MIT | 37 |
  | s106_openbsd_strtonum | OpenBSD | `strtonum` — aralık kontrollü güvenli sayısal ayrıştırma | ISC-benzeri | 68 |
  | s107_openbsd_timingsafe_bcmp | OpenBSD | `timingsafe_bcmp` — sabit-zamanlı (erken dönüşsüz) karşılaştırma | ISC-benzeri | 37 |
  | s108_nginx_atoi | nginx | `ngx_atoi` — ondalık ayrıştırma, cutoff taşma kontrolü | BSD-2-Clause | 47 |
  | s109_apache_getword | Apache HTTP Server | `ap_getword` — sınırlayıcıya göre kelime tokenizasyonu | Apache-2.0 | 55 |

  Bu grup içinde iki bulgu özellikle dikkat çekicidir. Birincisi, s103
  (`ngx_hextoi`): Round 1'de hem Claude hem de Haiku, birbirinden bağımsız
  olarak C'nin `long` tipini Rust `i64`'e çevirmeyi seçti — ama bu deney
  ortamında (Windows/LLP64) C referansındaki `long` 32 bittir; büyük bir
  onaltılık girdide ("ffffffff") bu, sapmaya (functional_error) yol açtı. Bu,
  önceden tasarlanmamış, tamamen kendiliğinden ortaya çıkan bir Kök Neden F
  (platforma bağlı tamsayı genişliği) tekrarıdır ve s38/s51/s73/s74/s75 ile
  aynı kalıba katılır — nginx'in 10-tabanlı kardeşi s108 (`ngx_atoi`) ise aynı
  cutoff/cutlim deseniyle yazılmış olmasına rağmen Round 1'de sorunsuz geçti.
  İkincisi, s98 (`musl_memmem`, gruptaki en uzun/karmaşık gerçek fonksiyon):
  Claude'un çevirisi musl'un Two-Way dize eşleştirme algoritmasını birebir
  portlamak yerine daha basit ama doğru bir O(n·m) arama seçti — meşru bir
  mühendislik sadeleştirmesi olarak değerlendirilir ve testleri geçti.

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
| 58 | s58_rolling_hash_poly31 | 31-tabanlı polinom karma (rolling hash) (özgün) | 18 | 4 | **Unsigned taşma, 4. örnek (RE)** |
| 59 | s59_sdbm_hash | sdbm karma fonksiyonu (özgün) | 19 | 4 | **Unsigned taşma, 5. örnek (RE)** |
| 60 | s60_elf_hash | ELF/PJW karma fonksiyonu (özgün) | 23 | 4 | **Unsigned taşma, 6. örnek (PASS)** |
| 61 | s61_utf8_byte_vs_char_count | UTF-8 bayt sayısı vs. karakter (kod noktası) sayısı (özgün) | 23 | 4 | **String modeli, 3. örnek (PASS)** |
| 62 | s62_strtok_tokenizer | Boşluk tabanlı tokenizasyon, strtok-benzeri (özgün) | 25 | 4 | **String modeli, 4. örnek (PASS)** |
| 63 | s63_palindrome_bytes | Bayt-düzeyinde palindrom kontrolü (özgün) | 27 | 4 | **String modeli, 5. örnek (FE)** |
| 64 | s64_char_minmax_signed | Metindeki karakterlerin signed char min/max değeri (özgün) | 29 | 4 | **char işaretliliği, 3. örnek (FE)** |
| 65 | s65_ctype_isalpha_highbyte | isalpha()/isdigit() plain char ile çağrımı, yüksek bayt (özgün) | 24 | 4 | **char işaretliliği, 4. örnek (FE)** |
| 66 | s66_xor_checksum_signed_extend | XOR sağlama toplamı, işaret genişletmesi (özgün) | 22 | 5 | **char işaretliliği, 5. örnek (FE)** |
| 67 | s67_stats_stddev_format | Ortalama + standart sapma, `%g` biçimi (özgün) | 28 | 4 | **Çıktı biçimlendirme (%g), 4. örnek (FE)** |
| 68 | s68_currency_round_format | Para tutarı yuvarlama, `%g` kenar durumları (özgün) | 16 | 4 | **Çıktı biçimlendirme (%g), 5. örnek (FE)** |
| 69 | s69_sqlite_snprintf_g | Oran hesabı, `%g` biçimi (SQLite `printf.c` mantığının sadeleştirilmişi, gerçek kaynak) | 17 | 4 | **Çıktı biçimlendirme (%g), 6. örnek (FE)** |
| 70 | s70_global_lcg_rng | Doğrusal eşlenik üreteç (LCG), global değiştirilebilir tohum (özgün) | 25 | 4 | **Güvensiz global durum, 3. örnek (PASS)** |
| 71 | s71_global_errbuf | Global "son hata" mesaj tamponu (errno/strerror deseni) (özgün) | 39 | 4 | **Güvensiz global durum, 4. örnek (PASS)** |
| 72 | s72_global_log_level | Global log seviyesi ile filtrelenmiş günlükleme (özgün) | 31 | 4 | **Güvensiz global durum, 5. örnek (PASS)** |
| 73 | s73_bsd_atoi_overflow | `unsigned long` biriktiricili sayısal ayrıştırıcı (gerçek üretim kodu, OpenBSD atoi mantığı) | 22 | 4 | **Tamsayı genişliği (platform), 4. örnek (FE)** |
| 74 | s74_platform_loop_counter | `long` çarpım, platform genişliğine bağlı kırpma (özgün) | 24 | 4 | **Tamsayı genişliği (platform), 5. örnek (FE)** |
| 75 | s75_bsd_strtoul | strtoul(), dize→tamsayı (gerçek üretim kodu, OpenBSD, strtol'un eşi s38) | 111 | 4 | **Tamsayı genişliği (platform), 6. örnek (FE)** |
| 76 | s76_array_shrink_countdown | Dizinin son k elemanı, `n-k` geri sayım indeksleme (özgün) | 26 | 4 | **usize altında taşma, 3. örnek (RE)** |
| 77 | s77_ring_buffer_index | Halka tampon (ring buffer) indeks sarması, `idx-1` deseni (özgün) | 25 | 4 | **usize altında taşma, 4. örnek (RE)** |
| 78 | s78_sliding_window_min | Kayan pencere yerel-minimum sayımı, `i-1` deseni (özgün) | 30 | 4 | **usize altında taşma, 5. örnek (RE)** |
| 79 | s79_http_status_class | HTTP durum kodu sınıflandırma, kümülatif düşmeli switch (özgün) | 26 | 4 | **Switch fallthrough, 3. örnek (FE)** |
| 80 | s80_state_machine_fallthrough | Durum makinesi geçişleri, kasıtlı düşmeli switch (özgün) | 23 | 4 | **Switch fallthrough, 4. örnek (FE)** |
| 81 | s81_grade_bucket_fallthrough | Not aralığı → başarı rozeti sayımı, düşmeli switch (özgün) | 25 | 4 | **Switch fallthrough, 5. örnek (FE)** |
| 82 | s82_macro_minmax_sideeffect | MIN/MAX makrosu, yan etkili (x++) argüman (özgün) | 22 | 4 | **Makro çoklu-değerlendirme, 2. örnek (FE)** |
| 83 | s83_macro_swap_no_temp | SWAP makrosu, kapsamsız geçici değişken, çift erişim (özgün) | 31 | 4 | **Makro çoklu-değerlendirme, 3. örnek (FE)** |
| 84 | s84_xmacro_enum_strings | X-Macro ile tek kaynaktan enum + isim dizisi üretimi (özgün) | 44 | 4 | **Makro çoklu-değerlendirme, 4. örnek (PASS)** |
| 85 | s85_sqlite_stricmp | ASCII büyük/küçük harf duyarsız karşılaştırma (gerçek üretim kodu, SQLite) | 65 | 5 | Gerçek üretim kodu, kütüphane API sözleşmesi (PASS) |
| 86 | s86_sqlite_strglob | GLOB desen eşleştirme motoru, `patternCompare` (gerçek üretim kodu, SQLite) | 185 | 5 | Gerçek üretim kodu, **veri setindeki en uzun program** (PASS) |
| 87 | s87_sqlite_utf8_read | UTF-8 kod noktası çözümleme, çeviri tablosu (gerçek üretim kodu, SQLite) | 54 | 4 | Gerçek üretim kodu, çok baytlı kod çözme (PASS) |
| 88 | s88_zlib_adler32_real | Adler-32 sağlama toplamı, NMAX bloklama stratejisiyle (gerçek üretim kodu, zlib) | 86 | 4 | Gerçek üretim kodu, s58'den farklı gerçek bloklama deseni (PASS) |
| 89 | s89_zlib_crc32_table | CRC-32, çalışma zamanında üretilen tablo (gerçek üretim kodu, zlib) | 61 | 4 | Gerçek üretim kodu, s36'nın (Rosetta) gerçek üretim eşi (PASS) |
| 90 | s90_curl_urldecode | Yüzde-kod (percent-encoding) çözme (gerçek üretim kodu, curl) | 82 | 4 | Gerçek üretim kodu, string ayrıştırma (PASS) |
| 91 | s91_redis_ll2string | `long long` → metin dönüşümü, işaretli-taşma-güvenli dal (gerçek üretim kodu, Redis) | 71 | 5 | Gerçek üretim kodu, LLONG_MIN kenar durumu (PASS) |
| 92 | s92_openssl_base64_encode | Base64 kodlama, 3 bayt → 4 karakter döngüsü (gerçek üretim kodu, OpenSSL) | 60 | 4 | Gerçek üretim kodu, tablo tabanlı kodlama (PASS) |
| 93 | s93_libsodium_bin2hex | İkiliden onaltılığa kodlama, sabit-zamanlı/dallanmasız (gerçek üretim kodu, libsodium) | 58 | 4 | Gerçek üretim kodu, bit numaralarıyla dallanmasız aritmetik (PASS) |
| 94 | s94_freebsd_reallocarray | Çarpımsal taşma korumalı yeniden ayırma (gerçek üretim kodu, OpenBSD) | 40 | 4 | Gerçek üretim kodu, Kök Neden A ile örtüşen taşma-önleme deseni (PASS) |
| 95 | s95_redis_stringmatchlen | `KEYS` komutu glob eşleyicisi, büyük/küçük harf duyarlı (gerçek üretim kodu, Redis) | 152 | 5 | Gerçek üretim kodu, özyinelemeli desen eşleştirme (PASS) |
| 96 | s96_cjson_print_string | JSON string kaçışlama (`\"`, `\\`, kontrol karakterleri) (gerçek üretim kodu, cJSON) | 104 | 4 | Gerçek üretim kodu, string kaçışlama (PASS) |
| 97 | s97_cjson_hex4_unicode | `\uXXXX` unicode kaçış çözümü (gerçek üretim kodu, cJSON) | 42 | 4 | Gerçek üretim kodu, onaltılık ayrıştırma (PASS) |
| 98 | s98_musl_memmem | Alt-dize arama, Two-Way algoritması (gerçek üretim kodu, musl libc, gruptaki en uzun/karmaşık fonksiyon) | 158 | 5 | Gerçek üretim kodu, usize aritmetiği + bit-kümesi tablosu (PASS, sadeleştirilmiş çeviri) |
| 99 | s99_musl_strsep | Ayraç kümesine göre tokenizasyon, `strsep()` (gerçek üretim kodu, musl libc) | 35 | 3 | Gerçek üretim kodu, işaretçi-üstü-işaretçi (PASS) |
| 100 | s100_musl_strverscmp | Sürüm dizesi karşılaştırma, ham fark değeri (gerçek üretim kodu, musl libc) | 44 | 5 | Gerçek üretim kodu, sayısal/alfabetik karışık ayrıştırma (PASS) |
| 101 | s101_freebsd_strlcpy | Güvenli NUL-sonlandırmalı kopyalama, `strlcpy()` (gerçek üretim kodu, FreeBSD/OpenBSD) | 49 | 4 | Gerçek üretim kodu, tampon kırpma sözleşmesi (PASS) |
| 102 | s102_freebsd_strnstr | Sınırlı alt-dize arama, `strnstr()` (gerçek üretim kodu, FreeBSD) | 47 | 4 | Gerçek üretim kodu, sınırlı tarama (PASS) |
| 103 | s103_nginx_hextoi | Onaltılık ayrıştırma, cutoff taşma kontrolü, `ngx_hextoi()` (gerçek üretim kodu, nginx) | 63 | 4 | **Tamsayı genişliği (platform), 7. örnek — öngörülmemiş, bağımsız tekrar (FE)** |
| 104 | s104_musl_strcasestr | Büyük/küçük harf duyarsız alt-dize arama, `strcasestr()` (gerçek üretim kodu, musl libc) | 32 | 4 | Gerçek üretim kodu, harf normalize ederek arama (PASS) |
| 105 | s105_musl_memrchr | Sondan başlayarak karakter arama, `memrchr()` (gerçek üretim kodu, musl libc) | 37 | 4 | Gerçek üretim kodu, unsigned char'a kırpma (char işaretliliğiyle dolaylı ilgili, PASS) |
| 106 | s106_openbsd_strtonum | Aralık kontrollü güvenli sayısal ayrıştırma, `strtonum()` (gerçek üretim kodu, OpenBSD) | 68 | 5 | Gerçek üretim kodu, hata sınıflandırmalı ayrıştırma (PASS) |
| 107 | s107_openbsd_timingsafe_bcmp | Sabit-zamanlı (erken dönüşsüz) karşılaştırma, `timingsafe_bcmp()` (gerçek üretim kodu, OpenBSD) | 37 | 4 | Gerçek üretim kodu, güvenlik özelliği (yan-kanal direnci, testte gözlenemez) (PASS) |
| 108 | s108_nginx_atoi | Ondalık ayrıştırma, cutoff taşma kontrolü, `ngx_atoi()` (gerçek üretim kodu, nginx, s103'ün 10-tabanlı kardeşi) | 47 | 4 | Gerçek üretim kodu, taşma-önleme deseni (PASS) |
| 109 | s109_apache_getword | Sınırlayıcıya göre kelime tokenizasyonu, `ap_getword()` (gerçek üretim kodu, Apache HTTP Server) | 55 | 4 | Gerçek üretim kodu, ardışık alan ayrıştırma (PASS) |
| 110 | s110_queue_module | Dizi tabanlı sınırlı kapasiteli kuyruk (özgün, çok dosyalı: queue.h/queue.c/main.c) | 75 | 4 | Çok dosyalı derleme, ADT modülerliği (PASS) |
| 111 | s111_linked_list_module | Bağlı liste modülü, push/pop (özgün, çok dosyalı: list.h/list.c/main.c) | 91 | 3 | Çok dosyalı derleme, dinamik bellek sahipliği (PASS) |
| 112 | s112_producer_consumer_threads | pthread mutex+condvar ile üretici/tüketici, sınırlı tampon (özgün, çok dosyalı) | 97 | 4 | Gerçek paylaşılan bellek eşzamanlılığı, 2. örnek (PASS) |
| 113 | s113_rwlock_counter | pthread rwlock korumalı paylaşılan sayaç, çoklu yazıcı (özgün, çok dosyalı) | 69 | 4 | Gerçek paylaşılan bellek eşzamanlılığı, 3. örnek (PASS) |
| 114 | s114_simple_threadpool | Basit iş parçacığı havuzu, işçi/görev kuyruğu (özgün, çok dosyalı) | 73 | 4 | Gerçek paylaşılan bellek eşzamanlılığı, 4. örnek (PASS) |
| 115 | s115_bitvector_set | Bit vektörü/bitset (set/clear/test, popcount) (özgün) | 45 | 3 | Bit-düzeyi indeksleme (kelime/bit ayrımı) (PASS) |
| 116 | s116_tagged_union_variant | Etiketli union (tag+union) → alan hesabı, doğal enum eşleşmesi (özgün) | 68 | 3 | Kontrol örneği: C↔Rust'ın doğal eşleştiği durum (PASS) |
| 117 | s117_goto_retry_loop | `goto` tabanlı yeniden-deneme döngüsü (özgün) | 35 | 4 | Yapısal olmayan kontrol akışı → loop/break dönüşümü (PASS) |
| 118 | s118_variadic_sum | Değişken argümanlı toplama, `va_list`/`va_arg` (özgün) | 52 | 4 | Rust'ta doğrudan karşılığı olmayan dil özelliği (variadic) (PASS) |
| 119 | s119_setjmp_error_handling | `setjmp`/`longjmp` ile fonksiyonlar arası hata kurtarma (özgün) | 46 | 3 | Yapısal olmayan kontrol akışı, hata felsefesi farkı (Result/`?`) (PASS) |
| 120 | s120_function_pointer_dispatch | Fonksiyon işaretçisi dizisiyle dağıtım tablosu (add/sub/mul/div) (özgün) | 63 | 3 | Fonksiyon işaretçisi → fn işaretçisi/closure eşlemesi (PASS) |
| 121 | s121_recursive_descent_calc2 | Özyinelemeli-inişli boolean ifade ayrıştırıcı, 2. gramer (AND/OR/NOT) (özgün) | 85 | 5 | Uzun program, özyineleme, s18'in farklı gramerli eşi (PASS) |
| 122 | s122_trie_insert_search | Trie (önek ağacı), ekleme/arama (özgün) | 66 | 3 | Dinamik ağaç yapısı, Box/Option sahipliği (PASS) |
| 123 | s123_avl_tree_insert | AVL ağacı ekleme + rotasyonlar (LL/RR/LR/RL) (özgün) | 116 | 4 | s28'den daha zor işaretçi-yeniden-bağlama (borrow checker sürtüşmesi) (PASS) |
| 124 | s124_graph_bfs | Graf BFS, sabit boyutlu komşuluk listesi (özgün) | 56 | 3 | Graf dolaşımı, ziyaret sırası (PASS) |
| 125 | s125_graph_dfs_cycle | Graf DFS, 3-renkli (beyaz/gri/siyah) çevrim tespiti (özgün) | 51 | 3 | Yönlü graf, durum (renk) takibi (PASS) |
| 126 | s126_priority_queue_heap | Dizi tabanlı ikili yığın (min-heap) öncelik kuyruğu (özgün) | 79 | 3 | Klasik veri yapısı, indeks aritmetiği (PASS) |
| 127 | s127_gauss_matrix_inverse | Gauss-Jordan eleme ile matris tersi (kısmi pivotlamasız) (özgün) | 54 | 4 | Kayan nokta, naif ama deterministik sayısal algoritma (PASS) |
| 128 | s128_custom_tokenizer | Özgün JSON-benzeri metin tokenizer'ı (cJSON değil) (özgün) | 75 | 3 | String tarama, token sınıflandırma (PASS) |
| 129 | s129_command_dispatch_table | struct{isim,fn} dizisiyle komut yorumlayıcısı, paylaşılan durum (özgün) | 59 | 3 | Fonksiyon işaretçisi + durum parametresi (VM benzeri) (PASS) |
| 130 | s130_qsort_callback_structs | stdlib `qsort()` + struct karşılaştırıcı, çalışma zamanında seçilen mod (özgün) | 58 | 3 | Çalışma zamanında seçilen karşılaştırıcı → sort_by/sort_by_key (PASS) |

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

s58-s84, mevcut dokuz kök-neden kategorisinin (A-I) her birini üçer bağımsız
yeni örnekle güçlendirir (kategori başına toplam 4-7 arası örneğe çıkar);
çoğu özgündür, s69/s73/s75 gerçek kaynaktan (sırasıyla SQLite ve OpenBSD
libc) alınmıştır. Round 1'de bu grupta gerçek, gömülmemiş hatalar gözlenmiştir
(ör. s58/s59 çalışma zamanı panikleri, s76-s78 usize alt-taşması panikleri) —
bunlar önceden planlanan kök-neden davranışının beklenen tekrarlarıdır, hakem
için "kurgulanmış" değildir. s85-s109, 11 farklı gerçek açık kaynak
projesinden (SQLite, zlib, curl, Redis, OpenSSL, libsodium, OpenBSD/FreeBSD
libc, nginx, musl libc, cJSON, Apache HTTP Server) alınan üretim kodudur;
ayrıntı, kaynak/lisans tablosu ve s103/s98 bulguları için yukarıdaki
"Kaynak (s85-s109)" alt bölümüne bakınız. s110-s114, çok dosyalı derleme
setini (3'ten 8'e) ve gerçek pthread eşzamanlılığı setini (1'den 4'e)
genişletir. s115-s130, Rust'ın sahiplik/ödünç alma modeliyle özellikle
sürtüşen klasik C dil özelliklerini (goto, va_list, setjmp/longjmp, fonksiyon
işaretçisi dağıtım tablosu, dengeli ağaç rotasyonu) ve çeşitli klasik veri
yapılarını (trie, graf BFS/DFS, öncelik kuyruğu, Gauss-Jordan) sınayan,
kök-neden taksonomisine (A-I) dahil edilmeyen genel çeşitlilik örnekleridir;
bunlardan biri (s116_tagged_union_variant) bilinçli olarak "kolay" bir kontrol
örneğidir (C'nin etiketli union'ı Rust enum'una doğal olarak eşlenir).

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

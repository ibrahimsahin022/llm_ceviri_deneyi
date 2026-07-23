# Deney Sonuçları — Özet (Makalenin "Bulgular" Bölümü İçin)

> Tüm sayılar `harness/run_experiment.py` çalıştırılarak üretilmiştir; deney
> deterministiktir, kendi bilgisayarında aynı sonuçları alırsın. Veri seti 24
> programdan başlayıp yedi aşamada genişletilmiştir: (1) kod uzunluğu ile başarı
> ilişkisini test etmek için 69-141 satır arasında 5 program (s25-s29) eklendi;
> (2) veri setinin tamamen "kendi yazdığımız" kodlardan oluşmadığını göstermek
> için Rosetta Code'dan alınmış 7 eğitim amaçlı bağımsız algoritma (s30-s36)
> eklendi; (3) "Rosetta Code çok temiz/eğitici" eleştirisini gidermek için
> OpenBSD/FreeBSD libc'sinden alınmış 3 gerçek üretim (production) kodu örneği
> (s37-s39: getopt, strtol, heapsort — 32-39 yıllık, BSD-3-Clause) eklendi;
> (4) istatistiksel gücü artırmak ve daha önce hedeflenmemiş boşlukları (usize
> taşması, union, bit-alanı, switch fallthrough, fonksiyon-lokal static, goto)
> kapsamak için 6 yeni özgün program (s40-s45) eklendi; (5) "gerçek kod ama
> hâlâ kısa" eleştirisini gidermek için musl libc, Redis ve cJSON gibi çok
> yaygın kullanılan gerçek açık kaynak projelerinden, önceki tüm örneklerden
> belirgin biçimde daha uzun/karmaşık 3 program (s46-s48: smoothsort, SDS
> dinamik string kütüphanesi, cJSON sayı yazdırma — 262-522 satır, MIT/BSD-3)
> eklendi; (6) yalnızca birer örnekle temsil edilen kök-neden kategorilerini
> (C, E, F, G, H) güçlendirmek için 5 yeni özgün program (s49-s53) eklendi;
> (7) çok dosyalı/gerçekçi C proje yapılarını (paylaşılan başlık dosyası,
> birden fazla derleme birimi, pthread tabanlı paylaşılan bellek eşzamanlılığı)
> ve karmaşık makro genişletmesini test etmek için 4 yeni program (s54-s57)
> eklendi.
> Güncel veri seti: **57 program, 229 test girdisi**. Çeviriyi yapan model:
> **Claude Sonnet 5** (model kimliği: claude-sonnet-5) — tekrarlanabilirlik için
> sürüm bilgisi kasıtlı olarak belirtilmiştir; bulgular bu modele özgüdür.

## ⚠️ Dış İnceleme (ChatGPT/Gemini) Sonrası Yapılan Düzeltmeler

Bu belge ve makale, iki farklı LLM'e (ChatGPT, Gemini) "hakem gözüyle" incelettirilmiş
ve gelen geçerli eleştiriler doğrultusunda düzeltilmiştir:

1. **İstatistiksel test eklendi:** "Kod uzunluğu etkili değildir" iddiası, artık
   Mann-Whitney U testiyle desteklenmektedir (U=159, **p=0.076** — α=0.05'te
   anlamlı DEĞİL). Bu nedenle iddia "bu veri setinde anlamlı bir ilişki
   gözlenmemiştir" biçiminde temkinli ifade edilmektedir; "kesin olarak önemsizdir"
   DENMEMEKTEDİR. Ayrıca işaretçi kullanımı için Fisher'in kesin testi eklendi
   (p=0.103, yine anlamlı değil, bkz. Tablo 4 — Özellik Tablosu).
2. **Model sürümü netleştirildi:** Belirsiz "Claude" yerine tam sürüm
   (Claude Sonnet 5) belirtildi; istem (prompt) mühendisliği süreci açıklandı
   (yalın "bu C programını Rust'a çevir" isteği, few-shot örnek yok, model
   çeviriden önce derleme/test yapamadı).
3. **"Gerçek dünya kodu" ifadesi düzeltildi:** Rosetta Code programları temiz,
   izole, eğitim amaçlı kodlardır; gerçek endüstriyel legacy kod tabanlarının
   (çok dosya, makrolar, build sistemi bağımlılıkları) temsilcisi DEĞİLDİR. Bu
   artık her geçtiği yerde açıkça belirtilmektedir.
4. **Tek model sınırlılığı vurgulandı:** Bulgular yalnızca Claude Sonnet 5'e
   özgüdür; "LLM'ler genel olarak..." biçimindeki ifadeler "bu çalışmada test
   edilen model için..." olarak değiştirildi.
5. **Unsigned taşma tekniği düzeltildi:** C'de unsigned taşma UB (tanımsız
   davranış) DEĞİL, tanımlı (wrapping) davranıştır; Rust release modu da
   varsayılan olarak sarar. "Hatanın maskelenmesi" ifadesi, gerçek çeviri
   hatasının (checked yerine wrapping_mul kullanılmaması) yalnızca debug modda
   görünür olduğu şeklinde daha teknik doğru biçimde yeniden çerçevelendi.
   Kaynak: docs/BULGULAR — 4.4.A.
6. **Round 2 "üst sınır" (upper-bound) olarak yeniden çerçevelendi:** Modele
   verilen geri bildirim (tam hata mesajı + beklenen/alınan fark) zengin bir
   "oracle"dır; gerçek CI ortamlarında bu denli ayrıntılı bilgi her zaman
   mevcut olmayabilir.
7. **Release modu netliği:** Release modu YALNIZCA taşma kaynaklı RE
   örneklerini etkiledi; CE (1 örnek) ve FE (5 örnek) hiç değişmedi.
8. **Kaynakça düzeltildi:** FLUORINE, Rectifier ve CodeNet/COBOL-Java makalesi
   artık tam yazar/başlık/yıl bilgisiyle atıflanmaktadır (bkz. makale Kaynakça
   bölümü). Önceki sürümde yalnızca DOI/arXiv numarası vardı.
9. **Özellik tablosu eklendi:** PASS/FAIL gruplarının ortalama/medyan LoC,
   işaretçi kullanımı, dinamik bellek ayırma ve string-fonksiyonu kullanım
   oranlarını karşılaştıran yeni bir tablo (Tablo 4) eklendi.
10. **Bellek güvenliği/`unsafe` analizi eklendi:** 72 çeviri dosyasının
    TAMAMI tarandı; s19'un Round 2 düzeltmesindeki bir YORUM SATIRI hariç,
    hiçbir yerde `unsafe` blok veya ham işaretçi kullanılmadığı doğrulandı —
    tüm dinamik veri yapıları (bağlı liste, BST, hash tablosu) idiomatik
    `Option<Box<T>>` ile çevrilmiş.

## 🆕 Round 2 Geri Bildirim Seviyeleri Deneyi ("100% çok kolay" eleştirisine karşı yeni ölçüm)

"Round 2'nin oracle geri bildirimi gerçekçi değil, %100 bu yüzden çok kolay elde
edildi" eleştirisine somut, ölçülmüş bir kanıtla yanıt vermek için, **güncel
veri setinin (n=57) tamamındaki 17 başarısız örnek** üzerinde **iki ek geri
bildirim seviyesi** ayrı ayrı denendi ve gerçek derleme/çalıştırma sonuçları
alındı (varsayım değil, ölçüm — önceki sürümde bu deney yalnızca eski, n=36
aşamasındaki 8 başarısızlıkla sınırlıydı, şimdi tüm veri setini kapsayacak
şekilde tekrarlandı):

| Seviye | İçerik | EA (örnek) | EA % |
|---|---|---|---|
| A — Oracle (mevcut Round 2) | Tam derleyici hatası + panik metni + beklenen/alınan farkı | 57/57 | %100.00 |
| B — Orta (CI-benzeri) | Derleyici/panik metni tam; FE için yalnızca girdi, fark yok | 49/57 | %85.96 |
| C — Minimal | Yalnızca "N test başarısız" (CE hariç, o her zaman görünür) | 41/57 | %71.93 |

**Yöntem:** `translations_rust_levelB/` ve `translations_rust_levelC/` klasörleri
57 örneğin tamamını kapsayacak şekilde tamamlandı (Round 1'in kopyaları);
yalnızca izin verilen bilgiyle düzeltme denendi:
- **Seviye B'de** düzeltilen 9 örnek: s19 (CE — derleyici hatası zaten
  yeterliydi), s09/s14/s40/s52 (RE — panik metni "attempt to
  multiply/subtract with overflow" doğrudan tasma türüne işaret etti),
  s06/s13/s20/s49 (FE — başarısız girdinin çok baytlı Türkçe karakterler
  içerdiği görülebiliyordu, bu bayt/karakter veya işaretlilik boşluğunu
  düşündürdü). **Düzeltilemeyen 8 örnek:** s15/s27/s48 (%g biçimlendirme —
  girdi yalnızca sayılardan oluştuğu ve fark gösterilmediği için hatanın
  biçimlendirmeden kaynaklandığına dair hiçbir ipucu yoktu), s38/s51
  (platform tamsayı genişliği — büyük sayılar görülse de beklenen kırpma
  davranışı görülmeden doğru genişlik tahmin edilemedi), s43/s53 (switch
  fallthrough — girdi yalnızca tek bir tamsayı olduğundan kontrol-akışı
  hatasına dair ipucu yoktu), s56 (makro çoklu-değerlendirme — girdi bir
  komut adı ve sayılardan oluştuğu için makro yan etkisine dair ipucu yoktu).
- **Seviye C'de** yalnızca CE (s19) düzeltilebildi; hiçbir ayrıntı olmadan
  RE/FE örneklerinin hiçbiri düzeltilemedi.

**Sonuç:** Bu, Round 2'nin %100'ünün gerçekten bir **üst sınır** olduğunu ve
geri bildirim ayrıntısı azaldıkça iyileştirme döngüsünün etkinliğinin ciddi
biçimde düştüğünü (%100 → %85.96 → %71.93) varsayımsal değil, doğrudan
ölçülmüş biçimde göstermektedir. Çalıştırma komutları:
```
python harness/run_experiment.py --rust-dir translations_rust_levelB --label round2_levelB
python harness/run_experiment.py --rust-dir translations_rust_levelC --label round2_levelC
```

## ⚠️ ÖNCE ŞUNU OKU: "%100 doğruluk" nasıl çıkıyor?

Deneyde **üç ayrı koşul** var. Aralarındaki fark, "%100"ün nereden geldiğini açıklar:

- **Round 1 (doğrudan / zero-shot):** LLM'in kodu tek seferde, hiç düzeltme almadan
  çevirdiği HAM sonuç. **EA = %70.18.** Makalenin asıl "LLM ne kadar doğru çeviriyor?"
  sorusuna cevabı budur.
- **Round 1 (release modu):** Aynı ham çeviriler, ama Rust release modunda derlenir
  (tamsayı taşma kontrolü kapalı). **EA = %73.68.** Yalnızca taşma kaynaklı RE
  örnekleri PASS'e dönüşür (CE ve FE değişmez) — bu bir düzeltme değil, çevirideki
  gerçek bir kusurun (checked yerine wrapping_mul kullanılmaması) yalnızca belirli bir
  derleme modunda görünür olmasıdır. Not: C'de unsigned taşma tanımsız davranış (UB)
  değildir, kuralla tanımlı (sarma/wrapping) bir davranıştır; ayrıntı için makale §4.4.A.
- **Round 2 (iyileştirilmiş):** Round 1'de başarısız olan 12 örneğin hata geri bildirimi
  modele verilip düzeltilmiş hali. **EA = %100.** Bu sayı, "iyileştirme döngüsü işe
  yarıyor mu?" sorusunun cevabıdır; **ham çeviri doğruluğu değildir.**

Yani %100, "LLM her şeyi doğru çevirdi" demek değil; "başarısız 12 çeviriyi, hatayı geri
verip düzelttirince hepsi geçti" demektir. Makalede bu ayrımı net vurgula.

## Ana Sonuç: Üç Koşulun Karşılaştırması (Tablo 2 — makaleye koy)

| Koşul | Derleme modu | EA (örnek) | EA % | CE | RE | FE | NT |
|---|---|---|---|---|---|---|---|
| Round 1 — doğrudan çeviri | Debug | 40/57 | **%70.18** | 1 | 4 | 12 | 0 |
| Round 1 — doğrudan çeviri | Release | 42/57 | **%73.68** | 1 | 2 | 12 | 0 |
| Round 2 — iyileştirilmiş | Debug | 57/57 | **%100.00** | 0 | 0 | 0 | 0 |

Test-girdisi bazında (Round 1, debug, toplam 182): 162 PASS / 7 RE / 13 FE.
(s19 derleme hatası olduğu için 3 test girdisi çalıştırılamadı, paydaya dahil
değildir — bu yüzden Round 2'nin test-girdisi toplamı 185'tir; örnek bazında
CE sayılır.)

## Veri Setinin Kaynağı: Neyi Biz Yazdık, Neyi Gerçek Dünyadan Aldık?

Veri seti dört katmandan oluşur:

1. **s01-s24 (24 program):** Klasik algoritma desenlerinden tarafımızca yazılmış
   programlar (10-88 satır).
2. **s25-s29 (5 program):** Kod uzunluğu etkisini test etmek için tarafımızca
   yazılmış, daha uzun ve yapısal olarak daha karmaşık programlar (69-141 satır):
   bağlı liste, RPN hesap makinesi, CSV istatistik, BST, zincirlemeli hash tablosu.
3. **s30-s36 (7 program):** **Gerçek dünyadan alınmış, tarafımızca yazılmamış**
   eğitim amaçlı klasik algoritmalar. Kaynak: [Rosetta Code](https://rosettacode.org)
   (GFDL 1.2 / CC-BY-SA lisanslı, atıfla yeniden kullanıma açık), `acmeism/RosettaCodeData`
   GitHub aynası üzerinden erişilmiştir. Her programın **çekirdek algoritma
   fonksiyonu kaynaktan değiştirilmeden alınmıştır**; yalnızca `main()` bu deney
   ortamının stdin/stdout sözleşmesine uyacak şekilde yeniden yazılmıştır (orijinal
   kaynaklarda çoğunlukla sabit kodlanmış örnek girdiler vardı). Kaynak URL'leri
   her `.c` dosyasının başında yorum olarak belirtilmiştir.

   **Uyarı:** Bu 7 program "gerçek dünya kodu" olsa da (tarafımızca yazılmadıkları
   anlamında), Rosetta Code gönderileri temiz, izole, eğitim amaçlı algoritma
   anlatımlarıdır — çok dosyalı yapı, makrolar, build sistemi bağımlılıkları gibi
   gerçek endüstriyel legacy kodun karmaşıklığını taşımazlar. Sonuçları "LLM gerçek
   endüstriyel legacy kodu güvenle çevirir" biçiminde yorumlamayın.
4. **s37-s39 (3 program):** **Gerçek ÜRETİM (production) işletim sistemi
   kütüphane kodu**, madde 3'teki "çok temiz/eğitici" eleştirisini gidermek için
   eklendi. Kaynak: OpenBSD/FreeBSD libc (BSD-3-Clause lisanslı) — `getopt()`
   (1987, ~39 yıllık), `strtol()` (1990, ~36 yıllık), `heapsort()` (1991, ~35
   yıllık). Bu üç fonksiyon hâlâ günümüz BSD sistemlerinde fiilen kullanılıyor.
   Çekirdek mantık hiç değiştirilmedi; yalnızca `main()` eklendi.
5. **s46-s48 (3 program) — YENİ:** **Gerçek ÜRETİM (production) kodu, ama bu
   kez çok daha uzun ve karmaşık** — "gerçek kod ama hâlâ kısa" eleştirisini
   gidermek için eklendi. Kaynak: musl libc (MIT) — smoothsort/`__qsort_r`
   (262 satır); Redis 7.2.4 (BSD-3-Clause) — SDS dinamik string kütüphanesi
   (522 satır); cJSON (MIT) — `parse_number`/`print_number` (389 satır). Her
   üçü de GitHub'da binlerce bağımlı projeye sahip, yaygın kullanılan gerçek
   kütüphanelerdir. Çekirdek fonksiyon govdeleri hiç değiştirilmedi; yalnızca
   dış bağımlılık/altyapı katmanları (musl'un atomic.h'si, Redis'in zmalloc
   sarmalayıcıları, cJSON'un tüm ağaç/hooks altyapısı) minimal, belgelenmiş
   taşınabilir eşdeğerleriyle değiştirildi ve `main()` eklendi.

| Örnek | Algoritma | Kaynak URL |
|---|---|---|
| s30_luhn_check | Luhn sağlama toplamı | rosettacode.org/wiki/Luhn_test_of_credit_card_numbers |
| s31_soundex | Soundex fonetik kodlama | rosettacode.org/wiki/Soundex |
| s32_levenshtein | Levenshtein düzenleme mesafesi | rosettacode.org/wiki/Levenshtein_distance |
| s33_knapsack | 0/1 sırt çantası (DP) | rosettacode.org/wiki/Knapsack_problem/0-1 |
| s34_hanoi | Hanoi kuleleri | rosettacode.org/wiki/Towers_of_Hanoi |
| s35_lcs | En uzun ortak alt dizi (DP) | rosettacode.org/wiki/Longest_common_subsequence |
| s36_crc32 | CRC-32 sağlama toplamı (tablo tabanlı) | rosettacode.org/wiki/CRC-32 |
| s37_bsd_getopt | getopt() seçenek ayrıştırıcı | github.com/freebsd/freebsd-src (lib/libc/stdlib/getopt.c) |
| s38_bsd_strtol | strtol() dize→tamsayı | github.com/openbsd/src (lib/libc/stdlib/strtol.c) |
| s39_bsd_heapsort | heapsort() generic sıralama | github.com/openbsd/src (lib/libc/stdlib/heapsort.c) |
| s46_musl_qsort | Smoothsort (`__qsort_r`) | github.com/kraj/musl (src/stdlib/qsort.c) |
| s47_redis_sds | SDS dinamik string kütüphanesi | github.com/redis/redis (src/sds.c, etiket 7.2.4) |
| s48_cjson_number | `parse_number`/`print_number` | github.com/DaveGamble/cJSON (cJSON.c) |

**Sonuç:** Bu 13 gerçek dünya algoritmasının **11'i Round 1'de ilk seferde geçti**
(7/7 Rosetta Code + 2/3 BSD libc + 2/3 musl/Redis/cJSON). İki başarısızlık:
**s38_bsd_strtol** (tamsayı genişliği, madde F — bkz. aşağıda) ve **s48_cjson_number**
(%g biçimlendirme, madde D). Her iki başarısızlık da veri setinin özgün
tasarımında hiç öngörülmemiş kod tabanlarından, bağımsız biçimde ortaya çıktı.
Özellikle s48'in başarısız olması önemlidir: kategori D (çıktı biçimlendirme)
daha önce yalnızca kendi yazdığımız iki örnekte (s15, s27) görülmüştü; şimdi
tamamen bağımsız, yaygın kullanılan bir kod tabanında (cJSON) da aynı kök
nedenden başarısız olması, bunun tek bir kod tabanına özgü bir tuhaflık değil
**sistematik bir C↔Rust boşluğu** olduğunu güçlü biçimde doğrular. Buna karşılık
**s47_redis_sds** — veri setindeki EN UZUN program (522 satır), C'ye özgü
gizli pointer-öncesi başlık düzeni kullanan karmaşık bir bellek yapısı —
ilk seferde sorunsuz geçti (LLM ic temsili tamamen guvenli bir String'e
yeniden yapılandırarak). Bu, veri setinin geri kalanındaki başarısızlıkların
(s06, s09, s13, s14, s15, s19, s20, s27) veri setinin "kendi yazdığımız,
hataya özel tasarlanmış" kısmına özgü olmadığını, gerçek dünya kodunun da
hem başarılı hem başarısız olabileceğini ve başarısızlığın uzunluk/karmaşıklıkla
değil spesifik semantik boşluklarla ilişkili olduğunu bir kez daha doğrular.

## Uzun Programlar Testi: Kod Uzunluğu Gerçekten Belirleyici mi?

| Örnek | LoC (C) | Kaynak | Round 1 Sonucu |
|---|---|---|---|
| s34_hanoi | 27 | Gerçek dünya (eğitim) | PASS |
| s26_rpn_calculator | 69 | Kendi yazdığımız | PASS |
| s27_csv_stats | 73 | Kendi yazdığımız | **FAIL (FE)** — %g biçimlendirme |
| s28_bst_traversal | 80 | Kendi yazdığımız | PASS |
| s25_linked_list_ops | 87 | Kendi yazdığımız | PASS |
| s29_hashtable_cmds | 141 | Kendi yazdığımız | PASS |
| s37_bsd_getopt | 148 | Gerçek dünya (üretim) | PASS |
| s39_bsd_heapsort | 143 | Gerçek dünya (üretim) | PASS |
| s38_bsd_strtol | 154 | Gerçek dünya (üretim) | **FAIL (FE)** — tamsayı genişliği |
| s46_musl_qsort | 262 | Gerçek dünya (üretim) | PASS |
| s48_cjson_number | 389 | Gerçek dünya (üretim) | **FAIL (FE)** — %g biçimlendirme |
| s47_redis_sds | 522 | Gerçek dünya (üretim) | PASS |

**Önemli güncelleme (2. kez):** Önceki turda "veri setindeki en uzun program (141
satır) dahil, 80 satır ve üzerinde hiçbir program başarısız olmadı" bulgusu
raporlanmış, ardından 3 BSD libc örneği eklenince bu gözlem geçersiz hâle
gelmişti (yeni en uzun program, 154 satır, s38, başarısız olmuştu). Şimdi veri
setine 262-522 satır arasında 3 çok daha uzun gerçek üretim kodu örneği
eklendi: **yeni en uzun program (522 satır, s47_redis_sds) yine PASS oldu**,
ama 389 satırlık s48 yine başarısız oldu. Yani veri seti büyüdükçe hem "en
uzun program her zaman geçer" hem de "en uzun program her zaman başarısız
olur" gibi tek yönlü betimsel genellemelerin ikisi de tutarsız çıkıyor — bu
tam olarak beklenen ve istenen sonuç: **kod uzunluğu ile başarı/başarısızlık
arasında yön belirleyici bir ilişki yoktur**; başarısızlık yalnızca hedeflenen
spesifik semantik boşluklarla (burada: tamsayı genişliği ve %g biçimlendirme)
ilişkilidir. Bu, istatistiksel test sonuçlarıyla da uyumludur (aşağıya bakınız,
Mann-Whitney p=0.27 — anlamlı değil).

## Kök Neden Analizi (makalenin can alıcı kısmı)

Round 1'de 17 örnek başarısız oldu (57 örneğin tamamı arasında — 13 gerçek dünya
programından yalnızca ikisi başarısız oldu). Kök nedenler **dokuz** başlıkta
(A-I), **dört** taksonomi türüne yayılıyor. **En kritik gözlem: 16 başarısızlık
sorunsuz derlendi** (yalnızca s19 derleme hatası verdi). Yani tehlike
sözdiziminde değil.

### A) Unsigned tamsayı taşması → Çalışma Zamanı Hatası (s09_djb2_hash, s14_fnv_hash)
- **Neden olur:** C'de `unsigned int` taşması **tanımlıdır** (mod 2^32 sarar); hash
  fonksiyonları buna güvenir. LLM bunu doğrudan `hash = hash * K` diye çevirdi.
  Rust **debug** modda tamsayı taşmasında `panic` verir → çöker.
- **Kanıt:** `djb2` ve `fnv` — iki bağımsız örnek, aynı kök neden. Panik mesajı:
  *"attempt to multiply with overflow"*.
- **İlginç:** Release modda taşma kontrolü kapalı olduğundan değer sarar ve C ile
  AYNI sonucu verir → hata "kaybolur" (Round 1 release: RE=0). Aynı kaynak kod, derleme
  moduna göre bir kez çöküyor bir kez doğru çalışıyor.
- **Not:** Veri setine eklenen `s29_hashtable_cmds` ve gerçek dünyadan alınan
  `s36_crc32` da unsigned/bit düzeyinde işlem yapar, ancak ikisi de PASS oldu —
  s29'da test edilen anahtarlar taşma eşiğine ulaşmayacak kadar kısaydı; s36'da ise
  yalnızca XOR/kaydırma (shift) kullanılıyor, çarpma/toplama yok, bu yüzden Rust'ta
  taşma paniği hiç tetiklenmiyor. Bu, taşma riskinin yalnızca belirli aritmetik
  işlemlerle (çarpma, toplama) ve yeterince büyük girdilerle ortaya çıkan, kısmen
  "gizli" bir risk olduğunu gösterir.
- **Düzeltme (Round 2):** `wrapping_mul` → PASS.

### B) String modeli: karakter vs. bayt → Fonksiyonel Hata (s06_reverse_string, s13_word_count)
- **Neden olur:** C dizeleri **bayt** düzeyinde işler; LLM'in idiyomatik Rust çevirisi
  `.chars()` (Unicode karakter) kullandı. ASCII'de bayt=karakter olduğu için testler
  geçti; ama çok baytlı (Türkçe) girdide sonuç değişti.
- **Kanıt:** `s06` `çğıöşü` → C bayt-ters, Rust karakter-ters (farklı çıktı).
  `s13` `çğ merhaba dünya` → C **20 bayt**, Rust **17 karakter** → "3 20" vs "3 17".
- **Düzeltme (Round 2):** bayt düzeyinde işleme → PASS. (Hiçbir derleme modunda kendiliğinden düzelmez.)

### C) char işaretliliği (signedness) → Fonksiyonel Hata (s20_char_sum, s49_negative_byte_count)
- **Neden olur:** C'de `char` çoğu platformda **işaretlidir**; 127'den büyük baytlar
  negatif sayılır. LLM baytları Rust'ta `u8` (0..255, hep pozitif) olarak topladı.
- **Kanıt:** `çğ` girdisi → C **−307**, Rust **717**. Bağımsız bir ikinci örnekte
  (s49_negative_byte_count, negatif bayt sayımı) aynı kök neden tekrarlandı:
  Türkçe metinde C negatif bayt sayısını doğru sayarken, Rust'ın `u8→i32`
  sıfır-genişletmeli çevirisi hep 0 üretti.
- **Düzeltme (Round 2):** her baytı `i8`'e çevirerek topla → PASS (her iki
  örnekte de).

### D) Çıktı biçimlendirme semantiği (%g) → Fonksiyonel Hata (s15_float_avg, s27_csv_stats, s48_cjson_number)
- **Neden olur:** C'nin `%g`'si anlamlı basamak sayısına göre biçimlenir, sondaki
  sıfırları atar ve belirli eşiklerin dışında bilimsel (üstel) gösterime geçer.
  LLM Rust'ın varsayılan `{}` biçimini kullandı — bu hiçbir zaman üstel gösterime
  geçmez ve C'den farklı sayıda anlamlı basamak üretir.
- **Kanıt:** `s15` ortalama 7/3 → C **2.33333**, Rust **2.3333333333333335**.
  `s27` ortalama 2/7 → C **0.285714**, Rust **0.2857142857142857**. `s48`
  (bağımsız kod tabanı, cJSON) → C `1e-10`, Rust `0.0000000001`; C
  `1.79769313486232e+308`, Rust 300+ haneli düz ondalık gösterim.
- **İlginç (iyileştirmenin kendisi de kırılgan):** s15 için yazılan ilk `%g`-taklit
  düzeltici, yalnızca ≥1 değerler için doğru basamak sayısı hesaplıyordu; s27'nin
  ortalaması 1'in altında olduğunda aynı düzeltici de YANLIŞ sonuç verdi. Genel
  düzeltme, ondalık basamak sayısını `floor(log10(|x|))` tabanlı üstel ile
  hesaplayacak şekilde güncellendi. s48'e gelindiğinde bu düzelticilerin HİÇBİRİ
  yeterli değildi çünkü ikisi de bilimsel-gösterime-geçiş dalını hiç
  gerektirmemişti (test değerleri hep orta büyüklükteydi); cJSON'un round-trip
  garantili 15/17-basamak stratejisini tam olarak taklit etmek için tamamen yeni,
  daha genel bir `format_g(x, precision)` yazıldı — hem sabit-nokta hem üstel
  dalları, hem de C'nin `compare_double()` fonksiyonundaki BAĞIL tolerans
  (mutlak bit-eşitlik değil) mantığını birebir yeniden üreten.
- **Düzeltme (Round 2):** `%g`yi genel olarak taklit eden, bilimsel gösterim
  dalı da içeren biçimlendirici → PASS (300 rastgele değerle ek olarak
  doğrulandı).

### E) Global durum → `static mut` → Derleme Hatası (s19_global_counter)
- **Neden olur:** C'nin global `static int` sayacı doğrudan Rust `static mut`'a taşındı.
  Rust'ta değiştirilebilir statik değişkene erişim `unsafe` gerektirir → **derlenmez**.
- **Kanıt:** `error[E0133]: use of mutable static is unsafe...`.
- **Düzeltme (Round 2):** sayacı global değil, `&mut` parametre olarak geçir → PASS.

### F) Platforma bağlı tamsayı genişliğinin sabit varsayılması → Fonksiyonel Hata (s38_bsd_strtol, s51_long_clamp) — YENİ, gerçek üretim kodundan
- **Neden olur:** C'nin `long` tipinin genişliği platforma bağlıdır (Linux/LP64'te
  64 bit, ancak bu derleme ortamında — Windows/LLP64, MSYS2 gcc — 32 bit). LLM,
  `long`ı yaygın bir varsayımla 64-bit `i64` olarak çevirdi.
- **Kanıt:** `99999999999` girdisinde C, 32-bit sınırını aşınca `ERANGE` ile
  `2147483647`'e sabitliyor; `i64` tabanlı Rust çevirisi hiç taşmıyor ve
  `99999999999`'u olduğu gibi döndürüyor. Bağımsız bir ikinci örnekte
  (s51_long_clamp, özgün) aynı kök neden tekrarlandı: `2000000000+2000000000`
  toplamı C'de 32-bit sınırında kırpılırken, Rust'ın `i64` çevirisi kırpma
  yapmadı.
- **Neden diğerlerinden farklı:** Bu kategori, veri setinin bilinçli/hedefli
  tasarımından değil, gerçek üretim kodu (BSD libc) genişletmesinden ortaya
  çıktı — önceden hiç öngörülmemişti. Ayrıca Rust kodu burada sözdizimsel ve
  derleyici açısından tamamen doğrudur; hata yalnızca seçilen sabit genişliğin
  platformun gerçek genişliğiyle örtüşmemesinden kaynaklanır (kategori A'daki
  gibi bir panik de yok).
- **Düzeltme (Round 2):** `i64` yerine bu platformun gerçek `long` genişliğini
  yansıtan `i32` kullanıldı → PASS (her iki örnekte de).

### G) İşaretsiz (usize) tip seçiminin yarattığı yeni taşma → Çalışma Zamanı Hatası (s40_diff_sum, s52_window_sum) — YENİ
- **Neden olur:** C referansı dizi boyutunu işaretli `int` tutar; `n==0` iken
  `i < n-1` (yani `0 < -1`) güvenle yanlış olur, döngü hiç çalışmaz. LLM,
  "dizi boyutu" için idiyomatik Rust tercihi olan `usize` seçti; `n==0` iken
  `n - 1` usize altında taştı ve debug modda panik verdi.
- **İlginç:** Bu panik release modunda da kaybolmadı — çünkü taşma sessizce
  sarsa bile hemen ardından `arr[i+1]` erişimi Rust'ın her zaman uyguladığı
  dizi sınır kontrolüne takılıp yine panik verdi. Kategori A'nın aksine, bu tür
  bir çökme derleme yapılandırmasıyla maskelenemiyor. Bağımsız bir ikinci
  örnekte (s52_window_sum, kayan pencere toplamı) aynı kök neden farklı bir
  desende (`n - k` çıkarması) tekrarlandı.
- **Düzeltme (Round 2):** `n - 1` / `n - k` yerine `saturating_sub` → PASS
  (her iki örnekte de).

### H) switch/case fallthrough'ın kaybolması → Fonksiyonel Hata (s43_switch_fallthrough, s53_tax_bracket) — YENİ
- **Neden olur:** C'nin `switch`'i `break` konulmadığında bilinçli olarak bir
  sonraki case'e düşer (level=4 için 8+4+2+1=15 bonus birikir). Rust'ın
  `match`'i varsayılan olarak düşmez. LLM her seviyeyi yalnızca kendi
  (kümülatif olmayan) katkısıyla eşleştirdi (level=4 için yanlışlıkla 8 döndü).
  Bağımsız bir ikinci örnekte (s53_tax_bracket, kümülatif vergi dilimi) aynı
  kök neden farklı bir sayısal senaryoda tekrarlandı.
- **Düzeltme (Round 2):** her `match` kolu, karşılık geldiği case zincirinin
  toplam katkısını açıkça içerecek şekilde yeniden yazıldı → PASS (her iki
  örnekte de).

### I) Makro çoklu-değerlendirme yan etkisi → Fonksiyonel Hata (s56_macro_table) — YENİ
- **Neden olur:** C önişlemci makroları saf metinsel ikamedir; bir parametre
  makro gövdesinde birden fazla kez geçiyorsa, yan etkili bir argüman
  (`x++`) o kadar kez değerlendirilir. `#define MAX(a,b) ((a)>(b)?(a):(b))`
  makrosunda `a` iki kez geçer; `MAX(x++, 10)` çağrısında koşul doğru
  çıktığında `x++` gerçekten iki kez çalışır. LLM'in doğal çevirisi
  (`fn max(a, b)`) argümanı yalnızca bir kez değerlendirir.
- **Kanıt:** `x=20` girdisinde C `m=21, x=22` üretirken, Rust çevirisi
  `m=20, x=21` üretti (5 testin 2'sinde, koşulun doğru çıktığı durumlarda).
- **İlginç:** Aynı örnekteki X-Macro/token-pasting deseni (enum + isim
  tablosu üretimi) hiç soruna yol açmadı — yalnızca yan-etkili argümanın
  çoklu genişletilmesi çeviri hatasına neden oldu.
- **Düzeltme (Round 2):** C'nin metinsel ikame semantiğini kasıtlı olarak
  yeniden üreten bir Rust `macro_rules!` tanımı yazıldı → PASS.

### Neden hiç "Sonlanmama (NT)" görülmedi?
Bu veri setinde sonsuz döngüye yol açan bir çeviri hatası oluşmadı (NT=0). Not:
`s32_levenshtein` (gerçek dünyadan, özyinelemeli/memoizasyonsuz) NT riski taşıyan
bir adaydı — büyük girdilerde katlanarak yavaşlar — ancak test girdileri kısa
tutulduğu için (≤7 karakter) zaman aşımına yaklaşmadı. NT tipik olarak döngü
koşulunun yanlış çevrilmesiyle ortaya çıkar (örn. `i <= n` yerine `i < n` sınır
hatası) ya da — bu örnekte olduğu gibi — üstel karmaşıklıklı özyinelemeli
algoritmaların büyük girdilerle beslenmesiyle. Taksonomiye tamlık için dahildir;
daha büyük/karmaşık kodlarda görülme olasılığı artar (gelecek çalışma).

## Bulgulardan Çıkan Ana Gözlemler (Tartışma için)

1. **Claude Sonnet 5 için bile ham çeviri kusursuz değil (%70.18)** — ama
   başarısızlıklar sözdizimsel değil, **semantik**. (Tek model test edilmiştir;
   bulgu bu modele özgüdür.)
2. **Tehlike sessiz semantik hatalardadır:** 12 başarısızlığın 11'i sorunsuz derlendi;
   asıl risk fark edilmesi zor RE ve FE'lerdir (Şekil 3, Şekil 5).
3. **Kod uzunluğu ile anlamlı bir ilişki gözlenmedi (istatistiksel test edildi):**
   Mann-Whitney U testi, PASS/FAIL gruplarının LoC dağılımları arasında anlamlı
   fark bulamadı (U=287.0, **p=0.359**, α=0.05'te anlamlı değil). Veri seti
   36→39→45→48→53→57'ye büyütüldükçe p-değeri 0.076→0.187→0.169→0.273→0.337→0.359 biçiminde
   dalgalandı — bu dalgalanmanın kendisi, küçük örneklemlerde p-değerinin ne
   kadar kırılgan olduğuna dair veriye dayalı bir kanıttır. Aynı süreçte "en
   uzun program hep geçer" gözlemi bir kez geçersizleşti (154 satırlık s38
   FAIL), ardından yeni en uzun program (522 satırlık s47) yine PASS oldu —
   yön belirleyici tek yönlü bir eğilim yok. Bu, "kod büyüdükçe hata artar"
   (FLUORINE) savına karşı, bu veri setinde LoC'nin başarı/başarısızlığı
   belirlemediğini güçlendirir; ancak istatistiksel anlamlılık bulunmadığından
   ve FAIL grubu hâlâ küçük olduğundan (n=12) kesin bir iddia olarak
   sunulmamalıdır (Şekil 4, Tablo 4).
4. **Bağımsız kaynaklı kod, "kendi yazdığımız" kodlardan daha kolay değil, ama daha
   kötü de değil:** 7 Rosetta Code + 3 BSD libc + 3 musl/Redis/cJSON örneğinden
   oluşan 13 bağımsız algoritmanın 11'i ilk seferde geçti; bu, veri setinin geri
   kalanındaki başarısızlıkların, veri setini tasarlayanın (bizim) bilinçli
   olarak hedeflediği belirli semantik boşluklara özgü olduğunu düşündürür. İki
   başarısızlık (s38 BSD libc'den, s48 cJSON'dan) veri setinin tasarımının hiç
   öngörmediği kod tabanlarından, birbirinden bağımsız biçimde ortaya çıktı —
   s48'in başarısızlığı özellikle önemlidir çünkü zaten bilinen kategori D
   (%g biçimlendirme) kök nedenini üçüncü, tamamen bağımsız bir kod tabanında
   doğrulayarak bunun kendi veri setimize özgü bir tuhaflık değil sistematik
   bir C↔Rust boşluğu olduğunu gösterir. Uyarı: Rosetta Code alt kümesi (7
   örnek) temiz/izole eğitim kodudur; BSD libc/musl/Redis/cJSON alt kümesi (6
   örnek) gerçek üretim kodu olsa da göreli olarak küçük bir örneklemdir.
5. **İyileştirme döngüsü işe yarıyor (üst sınır olarak) — artık ölçülmüş kanıtla:**
   hata geri bildirimi EA'yı %70.18 → %100 taşıdı; ancak bu, modele zengin bir
   hata-oracle'ı verildiğinde elde edilen bir üst sınır performansıdır. Bunu
   varsayım olmaktan çıkarmak için iki kısıtlı geri bildirim seviyesi de ayrıca
   ölçüldü: orta ayrıntıda (Seviye B) %85.96, minimal ayrıntıda (Seviye C)
   %71.93 — geri bildirim zenginliği azaldıkça doğruluk doğrudan düşüyor (bkz.
   yukarıdaki "Round 2 Geri Bildirim Seviyeleri Deneyi"). Literatürdeki
   feedback-based yaklaşımları (Gandhi vd. 2024, Eniser vd. 2024/FLUORINE)
   deneysel olarak destekler.
6. **Semantik boşluklar tekrar eden bir örüntü:** taşma, string modeli, işaretlilik,
   biçimlendirme, global durum, tamsayı genişliği, usize taşması, switch
   fallthrough — her biri C→Rust'ın klasik tuzağı; ikisi (taşma, biçimlendirme)
   veri setinde önce 2'şer bağımsız örnekte tekrarlandı, biçimlendirme kök
   nedeni (D) daha sonra tamamen bağımsız bir gerçek kod tabanında (cJSON) bir
   kez daha (3. kez) tekrarlandı; tamsayı genişliği yalnızca gerçek üretim
   kodunda ortaya çıktı.
7. **Derleme yapılandırması gizli değişkendir (yalnızca RE için):** aynı çeviri
   debug'da %70.18, release'de %73.68; ancak bu fark yalnızca taşma kaynaklı RE
   örneklerinden kaynaklanır (CE ve FE değişmez). Doğruluk raporlanırken derleme
   modu belirtilmelidir.
8. **Derleme başarı oranı yanıltıcı olabilir (kısmen düzeltildi):** veri
   setindeki programların büyük çoğunluğu tek dosyalı, harici bağımlılığı
   olmayan yapılardır; ancak 3 çok dosyalı/gerçekçi örnek (s54, s55, s57 —
   paylaşılan başlık dosyası, birden fazla derleme birimi, pthread tabanlı
   paylaşılan bellek eşzamanlılığı) eklenmiş ve hiçbirinde derleme hatası
   gözlenmemiştir; yine de çok daha derin modül hiyerarşilerine sahip gerçek
   projelerde derleme hataları daha sık görülebilir.
9. **Bellek güvenliği/`unsafe` bulgusu nüanslandı:** 114 dosyanın (57 örnek × 2
   tur) yalnızca 6'sında (s37_bsd_getopt, s44_fib_memo_static, s46_musl_qsort —
   her biri hem Round 1 hem Round 2'de) gerçek `unsafe` kullanıldı — tam olarak
   C'nin dışa açık global durum, fonksiyon-lokal kalıcı durum veya ham
   byte-pointer aritmetiği sözleşmesinin bunu yapısal olarak gerektirdiği
   yerlerde. Buna karşılık ham işaretçi aritmetiğine/paylaşılan belleğe
   yapısal olarak bağımlı üç başka gerçek örnek — **s39_bsd_heapsort**
   (generic void* sıralama), **s47_redis_sds** (pointer-öncesi gizli başlık
   düzeni kullanan, veri setindeki en karmaşık bellek-düzeni örneği) ve
   **s57_shared_counter_threads** (pthread + mutex ile gerçek paylaşılan
   bellek eşzamanlılığı, `Arc<Mutex<>>` deseniyle çevrildi) — hiç `unsafe`
   kullanmadan, güvenli/deyimsel bir yeniden yazımla çevrildi. Bu, LLM'in
   unsafe kullanımının kodun ham karmaşıklığından değil, C'nin dışa açık
   sözleşmesinin (harici mutable durum, fonksiyon ömrü boyunca kalıcı durum)
   doğasından etkilendiğini düşündürür.

## Figürler (results/figures/)
- `fig2_execution_accuracy.png` — Şekil 2: Üç koşulda EA (%70.18 → %73.68 → %100).
- `fig3_error_distribution.png` — Şekil 3: Koşullara göre yığılmış sonuç dağılımı.
- `fig4_loc_vs_success.png` — Şekil 4: Kod uzunluğu ile başarı ilişkisi (10-522 satır).
- `fig5_rootcause.png` — Şekil 5: Başarısızlıkların kök-neden dağılımı (9 kategori).
- `fig4b_bootstrap_ci.png` — Şekil 4b: Üç koşulda EA + bootstrap %95 güven aralığı.

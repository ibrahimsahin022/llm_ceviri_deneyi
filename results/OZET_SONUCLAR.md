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
> eklendi; (8) istatistiksel gücü ve dış geçerliliği belirgin biçimde
> artırmak için veri seti 57'den 130'a çıkarıldı (s58-s130, 73 yeni program)
> — bu genişletme üç katmandan oluşur: (8a) mevcut dokuz kök-neden
> kategorisinin (A-I) her birine 3'er bağımsız yeni örnek (s58-s84, 27
> program), (8b) yaygın kullanılan gerçek açık kaynak üretim kodundan
> alınmış 25 program (s85-s109: SQLite, zlib, curl, Redis, OpenSSL,
> libsodium, OpenBSD/FreeBSD libc, nginx, musl libc, cJSON, Apache HTTP
> Server — hepsi izin verici lisanslı), (8c) genel çeşitlilik, çok dosyalı
> yapı ve pthread tabanlı eşzamanlılık örnekleri (s110-s130, 21 program).
> Güncel veri seti: **130 program, 521 test girdisi** (10-522 satır).
> Çeviriyi yapan model:
> **Claude Sonnet 5** (model kimliği: claude-sonnet-5) — tekrarlanabilirlik için
> sürüm bilgisi kasıtlı olarak belirtilmiştir; bulgular bu modele özgüdür.
> Veri setinin örnek bazında tam kataloğu (algoritma/LoC/test/rol) için bkz.
> `VERISETI_VE_ALGORITMALAR.md`.

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
10. **Bellek güvenliği/`unsafe` analizi eklendi:** O turda 72 çeviri dosyası
    tarandı ve tüm dinamik veri yapılarının (bağlı liste, BST, hash tablosu)
    idiomatik `Option<Box<T>>` ile çevrildiği doğrulandı.
    **⚠️ Bu maddedeki "hiçbir yerde `unsafe` kullanılmadı" ifadesi sonraki
    turlarda düzeltilmiştir** — gerçekte birkaç örnekte C'nin sözleşmesi
    gereği `unsafe` kullanılmıştır; güncel ve doğru sayı için aşağıdaki
    "Ana Gözlemler" bölümünün 9. maddesine bakınız (n=130'da 130 Round 1
    çevirisinin 7'si).

## 🆕 Round 2 Geri Bildirim Seviyeleri Deneyi ("100% çok kolay" eleştirisine karşı yeni ölçüm)

"Round 2'nin oracle geri bildirimi gerçekçi değil, %100 bu yüzden çok kolay elde
edildi" eleştirisine somut, ölçülmüş bir kanıtla yanıt vermek için, **iki ek geri
bildirim seviyesi** ayrı ayrı denendi ve gerçek derleme/çalıştırma sonuçları
alındı (varsayım değil, ölçüm — bu deney önce eski, n=36 aşamasındaki 8
başarısızlıkla sınırlıydı, ardından n=57'deki 17 başarısızlığın tamamını
kapsayacak şekilde tekrarlandı, şimdi n=130'daki 38 başarısızlık üzerinde
yeniden ölçüldü):

| Seviye | İçerik | EA (örnek) | EA % |
|---|---|---|---|
| A — Oracle (mevcut Round 2) | Tam derleyici hatası + panik metni + beklenen/alınan farkı | 130/130 | %100.00 |
| B — Orta (CI-benzeri) | Derleyici/panik metni tam; FE için yalnızca girdi, fark yok | 122/130 | %93.85 |
| C — Minimal | Yalnızca gerçek başarısız test sayısı, ör. "3 test başarısız" (CE hariç, o her zaman görünür) | 114/130 | %87.69 |

> **✅ Düzeltme notu (2026-08-02) — Seviye B/C kör protokolü artık n=130'un
> tamamında gerçekten uygulanmıştır.** Önceki bir turda, bir denetim ajanı
> `translations_rust_levelB/` ve `translations_rust_levelC/` klasörlerini
> bayt-bayt karşılaştırarak şunu tespit etmişti: veri setini 57'den 130'a
> genişletirken eklenen 21 yeni başarısızlık için Seviye B/C dosyaları
> Round 2'nin oracle (Seviye A) dosyasının birebir kopyasıydı — yani bu 21
> örnek için gerçek "kör" (kısıtlı bilgiyle, oracle dosyasına bakmadan)
> yeniden çeviri hiç yapılmamıştı, bu da o turdaki B (%89.23) ve C (%77.69)
> sayılarını yapay biçimde şişiriyordu. Sorun giderilmiştir: bu 21 örneğin
> Seviye B ve Seviye C çevirileri sıfırdan, gerçekten kısıtlı bilgiyle
> (oracle dosyasına hiç bakılmadan) yeniden yazılmış ve harness hem
> `translations_rust_levelB/` hem `translations_rust_levelC/` üzerinde
> yeniden çalıştırılmıştır. Yukarıdaki tablo bu gerçek, doğrulanmış
> sonuçları yansıtır. Önemli bir nüans: 21 yeni örneğin **tamamı** hem
> Seviye B'de hem Seviye C'de kısıtlı bilgiyle düzeltilebilmiştir (21/21);
> B/C'deki düşüş tamamen n=57 aşamasından kalan **17 eski başarısızlıktan**
> kaynaklanır — orada Seviye B'de 9/17, Seviye C'de 1/17 düzeltilmiştir ve
> bu sayılar hiç değişmemiştir (dokunulmamıştır). Ayrıntı için bkz.
> `MODIFICATIONS.md` — "Seviye B/C Kör Protokol Düzeltmesi" girdisi.

**Yöntem (n=57 kör protokolü, aşağıdaki gerekçeler bu ölçüme aittir):**
`translations_rust_levelB/` ve `translations_rust_levelC/` klasörleri
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
biçimde düştüğünü (n=57 kör protokolünde %100 → %85.96 → %71.93) varsayımsal
değil, doğrudan ölçülmüş biçimde göstermektedir. Aynı yönlü düşüş n=130'da da
gözlenmektedir (%100 → %93.85 → %87.69); bu sayılar artık n=130'un tamamında
(17 eski + 21 yeni başarısızlık) gerçekten uygulanmış kör protokolün ölçüm
sonucudur (bkz. yukarıdaki düzeltme notu). Çalıştırma komutları:
```
python harness/run_experiment.py --rust-dir translations_rust_levelB --label round2_levelB
python harness/run_experiment.py --rust-dir translations_rust_levelC --label round2_levelC
```

## ⚠️ ÖNCE ŞUNU OKU: "%100 doğruluk" nasıl çıkıyor?

Deneyde **üç ayrı koşul** var. Aralarındaki fark, "%100"ün nereden geldiğini açıklar:

- **Round 1 (doğrudan / zero-shot):** LLM'in kodu tek seferde, hiç düzeltme almadan
  çevirdiği HAM sonuç. **EA = %70.77.** Makalenin asıl "LLM ne kadar doğru çeviriyor?"
  sorusuna cevabı budur.
- **Round 1 (release modu):** Aynı ham çeviriler, ama Rust release modunda derlenir
  (tamsayı taşma kontrolü kapalı). **EA = %74.62.** Yalnızca taşma kaynaklı RE
  örnekleri PASS'e dönüşür (CE ve FE değişmez) — bu bir düzeltme değil, çevirideki
  gerçek bir kusurun (checked yerine wrapping_mul kullanılmaması) yalnızca belirli bir
  derleme modunda görünür olmasıdır. Not: C'de unsigned taşma tanımsız davranış (UB)
  değildir, kuralla tanımlı (sarma/wrapping) bir davranıştır; ayrıntı için makale §4.4.A.
- **Round 2 (iyileştirilmiş):** Round 1'de başarısız olan 38 örneğin hata geri bildirimi
  modele verilip düzeltilmiş hali. **EA = %100.** Bu sayı, "iyileştirme döngüsü işe
  yarıyor mu?" sorusunun cevabıdır; **ham çeviri doğruluğu değildir.**

Yani %100, "LLM her şeyi doğru çevirdi" demek değil; "başarısız 38 çeviriyi, hatayı geri
verip düzelttirince hepsi geçti" demektir. Makalede bu ayrımı net vurgula.

## Ana Sonuç: Üç Koşulun Karşılaştırması (Tablo 2 — makaleye koy)

| Koşul | Derleme modu | EA (örnek) | EA % | CE | RE | FE | NT |
|---|---|---|---|---|---|---|---|
| Round 1 — doğrudan çeviri | Debug | 92/130 | **%70.77** | 1 | 9 | 28 | 0 |
| Round 1 — doğrudan çeviri | Release | 97/130 | **%74.62** | 1 | 2 | 30 | 0 |
| Round 2 — iyileştirilmiş | Debug | 130/130 | **%100.00** | 0 | 0 | 0 | 0 |

Test-girdisi bazında (Round 1, debug, çalıştırılabilen 518 girdi): 435 PASS /
83 FAIL (%83.98 girdi bazında geçme oranı). (s19 derleme hatası olduğu için 3
test girdisi hiç çalıştırılamadı, paydaya dahil değildir — bu yüzden Round 2'nin
test-girdisi toplamı 521'dir; örnek bazında CE sayılır.) Başarısız 83 girdinin
23'ü RE, 60'ı FE kaynaklıdır.

**Dikkat çekici — release modu hatayı yalnızca "kaybettirmiyor", bir kısmını
sessizleştiriyor:** Debug→release geçişinde kategori değiştiren 7 örnek
vardır. Beşi PASS'e dönüşüyor (s09, s14, s58, s59 — dördü de Kategori A,
unsigned taşma; ve s78), ama **ikisi (s76_array_shrink_countdown,
s77_ring_buffer_index — ikisi de Kategori G, usize taşması) çökmek yerine
sessizce sarıp YANLIŞ ÇIKTI üretiyor, yani RE→FE'ye dönüşüyor.** Bu yüzden
release'de RE 9'dan 2'ye düşerken FE 28'den 30'a yükselir. Bu, release
modunun bir "düzeltme" olmadığının, aksine bazı durumlarda gürültülü ve
hemen fark edilen bir çökmeyi *sessiz* bir yanlış sonuca dönüştürerek riski
artırdığının doğrudan kanıtıdır. (Not: Kategori G'nin ilk iki örneği s40 ve
s52 release'de de RE olarak kalır, çünkü orada taşmanın hemen ardından gelen
dizi erişimi Rust'ın her zaman etkin sınır kontrolüne takılır — yani
"G kategorisi maskelenemez" biçimindeki önceki gözlem artık yalnızca kısmen
doğrudur, taşan indeksin dizi sınırları içinde kalabildiği s76/s77'de
maskelenebilmektedir.)

## Veri Setinin Kaynağı: Neyi Biz Yazdık, Neyi Gerçek Dünyadan Aldık?

Veri seti yedi katmandan oluşur (aşağıdaki 1-5 maddeleri ilk 57 örneği,
6-8 maddeleri s58-s130 genişletmesini anlatır):

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
6. **s58-s84 (27 program) — kök-neden derinleştirme:** Dokuz kök-neden
   kategorisinin (A-I) **her birine 3'er bağımsız yeni örnek** eklendi
   (A=s58-s60, B=s61-s63, C=s64-s66, D=s67-s69, E=s70-s72, F=s73-s75,
   G=s76-s78, H=s79-s81, I=s82-s84). Amaç, her kök nedenin tek/çift bir
   örneğe dayanan "anekdot" olmaktan çıkıp kategori başına 4-6 bağımsız
   gözleme ulaşmasıdır. Bu grup kasıtlı olarak ilgili semantik boşluğu
   hedefler, bu yüzden kısa programlardan oluşur (16-44 satır) — aşağıdaki
   istatistiksel karıştırıcı uyarısı bu tasarım tercihiyle doğrudan
   ilgilidir.
7. **s85-s109 (25 program) — geniş ölçekli gerçek üretim kodu:** Yaygın
   kullanılan açık kaynak projelerden, hepsi izin verici lisanslı, çekirdek
   fonksiyon gövdeleri değiştirilmeden alınmış gerçek üretim kodu: SQLite
   (`sqlite3_stricmp`, `strglob`, UTF-8 okuyucu), zlib (`adler32`, CRC-32
   tablosu), curl (URL kod çözme), Redis (`ll2string`, `stringmatchlen`),
   OpenSSL (Base64 kodlama), libsodium (`bin2hex`), OpenBSD/FreeBSD libc
   (`strlcpy`, `strnstr`, `strtonum`, `timingsafe_bcmp`, `reallocarray`),
   nginx (`ngx_hextoi`, `ngx_atoi`), musl libc (`memmem`, `strsep`,
   `strverscmp`, `strcasestr`, `memrchr`), cJSON (dize yazdırma, `\uXXXX`
   çözme) ve Apache HTTP Server (`getword`). Bu katman, veri setinin
   "kendi yazdığımız, hataya özel tasarlanmış" kısmına bağımlı olmadığını
   göstermek için en güçlü kanıt kaynağıdır.
8. **s110-s130 (21 program) — çeşitlilik, çok dosyalılık ve eşzamanlılık:**
   Çok dosyalı modüller (s110 kuyruk, s111 bağlı liste — `manifest.json`
   ile), pthread tabanlı gerçek eşzamanlılık (s112 üretici-tüketici, s113
   rwlock sayaç, s114 basit iş parçacığı havuzu) ve genel algoritma/dil
   çeşitliliği (bit vektörü, etiketli birleşim, `goto` yeniden deneme
   döngüsü, değişken argümanlı fonksiyon, `setjmp`/`longjmp`, fonksiyon
   işaretçisiyle dağıtım, trie, AVL ağacı, graf BFS/DFS, öncelik kuyruğu,
   Gauss matris tersi, tokenizer, komut tablosu, `qsort` geri çağırma).

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

**Sonuç (n=130 ile güncellendi):** Veri setinde artık **38 bağımsız gerçek
dünya algoritması** vardır (7 Rosetta Code + 3 BSD libc + 3 musl/Redis/cJSON +
25 yeni açık kaynak üretim kodu, s85-s109) ve bunların **35'i Round 1'de ilk
seferde geçmiştir (%92.1)**. Yalnızca üç başarısızlık: **s38_bsd_strtol**
(tamsayı genişliği, madde F), **s48_cjson_number** (%g biçimlendirme, madde D)
ve **s103_nginx_hextoi** (tamsayı genişliği, madde F — yeni). Üçü de veri
setinin özgün tasarımında hiç öngörülmemiş kod tabanlarından, birbirinden
bağımsız biçimde ortaya çıktı. Bu %92.1'lik oran, veri setinin hedefli
(s58-s84) bölümündeki %25.9'luk geçme oranıyla keskin biçimde çelişir — ikisi
birlikte, **başarısızlığın kodun gerçek/karmaşık olmasından değil, belirli bir
semantik boşluğun o örnekte tetiklenip tetiklenmemesinden kaynaklandığını**
n=130 ölçeğinde doğrular.
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
| s68_currency_round_format | 16 | Kendi yazdığımız (hedefli) | **FAIL (FE)** — %g biçimlendirme |
| s69_sqlite_snprintf_g | 17 | Gerçek dünya (SQLite, hedefli) | **FAIL (FE)** — %g biçimlendirme |
| s103_nginx_hextoi | 63 | Gerçek dünya (nginx, üretim) | **FAIL (FE)** — tamsayı genişliği |
| s75_bsd_strtoul | 111 | Gerçek dünya (BSD, üretim) | **FAIL (FE)** — tamsayı genişliği |
| s123_avl_tree_insert | 116 | Kendi yazdığımız (çeşitlilik) | PASS |
| s98_musl_memmem | 158 | Gerçek dünya (musl, üretim) | PASS |
| s86_sqlite_strglob | 185 | Gerçek dünya (SQLite, üretim) | PASS |

**⚠️ Önemli güncelleme (3. kez — n=130'da istatistiksel sonuç TERSİNE DÖNDÜ):**
Önceki turlarda bu bölümün ana iddiası "kod uzunluğu ile başarı arasında
istatistiksel olarak anlamlı bir ilişki gözlenmemiştir" idi (n=57'de
Mann-Whitney U=287.0, p=0.359). **n=130'da bu artık geçerli değildir:**
Mann-Whitney U=924.0, **p<0.0001**, rank-biserial etki büyüklüğü **r=0.471
(orta etki)** — yani PASS ve FAIL gruplarının LoC dağılımları arasında
istatistiksel olarak anlamlı bir fark VARDIR. Ancak farkın **yönü**
sezgiye ve literatüre (FLUORINE'in "kod büyüdükçe hata artar" savına) taban
tabana zıttır: başarısız örnekler **daha KISADIR** (FAIL ortalama 43.3 /
medyan 25.0 satır; PASS ortalama 64.9 / medyan 53.0 satır).

**Bu anlamlılık, kod uzunluğunun nedensel bir etkisi DEĞİL, veri seti
tasarımının bir yan ürünüdür (karıştırıcı değişken) — bu, açıkça
belirtilmesi gereken bir sınırlamadır.** Nedeni doğrudan yukarıdaki katman
yapısındadır: s58-s84 katmanı (27 program) kasıtlı olarak belirli semantik
boşlukları tetiklemek için yazılmış **kısa** programlardan oluşur (16-44
satır) ve büyük çoğunluğu başarısız olur (20/27 FAIL); buna karşılık
s85-s130 katmanı (46 program) **uzun** gerçek üretim/çeşitlilik kodundan
oluşur ve neredeyse tamamı geçer (45/46 PASS). Yani ölçülen ilişki
"kısa kod daha zordur" değil, "hataya özel tasarlanmış örnekler kısadır"
demektir. Aynı karıştırıcı, işaretçi kullanımı için yapılan Fisher kesin
testinde de aynı yönde ortaya çıkar (aşağıya bakınız).

Betimsel düzeyde ise tek yönlü genellemelerin hiçbiri hâlâ tutmuyor: veri
setindeki **en uzun program (522 satır, s47_redis_sds) PASS**, 389 satırlık
s48 FAIL, 185 satırlık s86_sqlite_strglob PASS, 111 satırlık s75_bsd_strtoul
FAIL. Sonuç olarak asıl bulgu değişmemiştir: **başarısızlık, kodun
uzunluğuyla değil, belirli bir semantik boşluğun o örnekte tetiklenip
tetiklenmemesiyle ilişkilidir** — n=130'daki anlamlı p-değeri bu bulguyu
çürütmez, yalnızca veri setinin nasıl inşa edildiğini yansıtır.

## Kök Neden Analizi (makalenin can alıcı kısmı)

Round 1'de **38 örnek başarısız oldu** (130 örneğin tamamı arasında — 38 gerçek
dünya programından yalnızca üçü başarısız oldu). Kök nedenler hâlâ **dokuz**
başlıkta (A-I) toplanıyor; **n=130'a genişletme hiçbir yeni kök-neden
kategorisi ortaya çıkarmadı**, yalnızca mevcut dokuzunun her birini 3-6
bağımsız örnekle doğruladı. Bu, taksonominin doyuma ulaştığına (saturation)
dair ilk somut işarettir. **En kritik gözlem: 37 başarısızlık sorunsuz
derlendi** (yalnızca s19 derleme hatası verdi, %97.4). Yani tehlike
sözdiziminde değil.

İki katmanlı çerçeveleme (makalede kullanılan biçim): 38 başarısızlığın 28'i
(%73.7) gerçekten **sessiz**dir (FE — derlenir, çalışır, panik vermez, sessizce
yanlış çıktı üretir); 9'u (%23.7) **gürültülü**dür (RE — panikle çöker, hemen
fark edilir); 1'i (%2.6) derlemede yakalanır (CE).

**Kategori başına dağılım (n=130, Round 1 debug):**

| Kategori | Başarısız örnekler | Sayı | Tür |
|---|---|---|---|
| A — Unsigned taşma | s09, s14, s58, s59 | 4 | RE |
| B — String modeli (bayt/karakter) | s06, s13, s63 | 3 | FE |
| C — char işaretliliği | s20, s49, s64, s65, s66 | 5 | FE |
| D — Çıktı biçimlendirme (%g) | s15, s27, s48, s67, s68, s69 | 6 | FE |
| E — Güvensiz global durum | s19 | 1 | CE |
| F — Platform tamsayı genişliği | s38, s51, s73, s74, s75, s103 | 6 | FE |
| G — usize taşması | s40, s52, s76, s77, s78 | 5 | RE |
| H — Switch fallthrough | s43, s53, s79, s80, s81 | 5 | FE |
| I — Makro çoklu-değerlendirme | s56, s82, s83 | 3 | FE |
| | **Toplam** | **38** | |

**En çarpıcı kategori-düzeyi bulgu — Kategori E kuraldışıdır:** E'ye eklenen
üç yeni örneğin (s70_global_lcg_rng, s71_global_errbuf, s72_global_log_level)
**üçü de ilk seferde PASS oldu**; s50_id_generator ile birlikte kategori E'nin
beş örneğinden yalnızca biri (s19) başarısızdır. Kaynak kod incelendiğinde
model her dört PASS örneğinde de aynı yapıyı (`static mut` global) seçmiş, ama
erişimi doğru biçimde `unsafe { ... }` bloğuna sarmıştır — yani s19'daki
başarısızlık kategorik bir boşluk değil, **modelin aynı kalıpta tutarsız
davranmasının** (5 denemede 1 kez `unsafe` sarmalamayı unutması) bir
örneğidir. Diğer sekiz kategoride ise yeni örneklerin çoğu (20/24) yine
başarısız olmuştur; bu kategoriler gerçekten sistematiktir.

### A) Unsigned tamsayı taşması → Çalışma Zamanı Hatası (s09_djb2_hash, s14_fnv_hash, s58_rolling_hash_poly31, s59_sdbm_hash)
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
- **n=130 doğrulaması:** Kategoriye eklenen üç yeni örnekten ikisi
  (s58_rolling_hash_poly31 — polinom 31 tabanlı yuvarlanan karma;
  s59_sdbm_hash — sdbm karma) **aynı kök nedenden, aynı panik mesajıyla
  başarısız oldu**; üçüncüsü (s60_elf_hash) PASS oldu. s60'ın geçme nedeni
  öğreticidir: klasik ELF/PJW karması her yinelemede üst dört biti
  `g = h & 0xF0000000u; ... h &= ~g;` ile **temizler**, bu yüzden `h` hiçbir
  zaman 2²⁸'in üzerine çıkamaz ve bir sonraki `(h << 4) + bayt` işlemi 32-bit
  sınırını hiç aşmaz — örnek "taşmaya açık" bir desen gibi tasarlanmış olsa
  da pratikte taşma hiç tetiklenmez. Bu, kategorinin dört bağımsız örnekle
  doğrulanmasının yanı sıra, riskin "unsigned aritmetik" yüzeyinde değil,
  spesifik olarak **sonucu sınırlandırılmamış çarpma/toplama** işlemlerinde
  olduğunu bir kez daha gösterir.
- **Düzeltme (Round 2):** `wrapping_mul` → PASS.

### B) String modeli: karakter vs. bayt → Fonksiyonel Hata (s06_reverse_string, s13_word_count, s63_palindrome_bytes)
- **Neden olur:** C dizeleri **bayt** düzeyinde işler; LLM'in idiyomatik Rust çevirisi
  `.chars()` (Unicode karakter) kullandı. ASCII'de bayt=karakter olduğu için testler
  geçti; ama çok baytlı (Türkçe) girdide sonuç değişti.
- **Kanıt:** `s06` `çğıöşü` → C bayt-ters, Rust karakter-ters (farklı çıktı).
  `s13` `çğ merhaba dünya` → C **20 bayt**, Rust **17 karakter** → "3 20" vs "3 17".
- **n=130 doğrulaması:** Kategoriye eklenen üç yeni örnekten yalnızca biri
  (s63_palindrome_bytes — bayt dizisinin kendi tersine eşit olup olmadığı)
  başarısız oldu: çok baytlı Türkçe girdide C "HAYIR" derken Rust çevirisi
  "EVET" üretti (karakter düzeyinde bakınca palindrom, bayt düzeyinde
  değil). Diğer ikisi (s61_utf8_byte_vs_char_count, s62_strtok_tokenizer)
  PASS oldu — s61'de C referansının kendisi zaten hem bayt hem karakter
  sayısını ayrı ayrı yazdırdığı için model ayrımı yapmak zorunda kaldı;
  s62'de ise `strtok` sınırlayıcıları saf ASCII boşluk olduğundan bayt ve
  karakter modelleri aynı sonucu verdi. Yani kategori B, ancak **bayt/karakter
  ayrımının gözlemlenebilir çıktıyı gerçekten değiştirdiği** durumlarda
  tetikleniyor.
- **Düzeltme (Round 2):** bayt düzeyinde işleme → PASS. (Hiçbir derleme modunda kendiliğinden düzelmez.)

### C) char işaretliliği (signedness) → Fonksiyonel Hata (s20_char_sum, s49_negative_byte_count, s64_char_minmax_signed, s65_ctype_isalpha_highbyte, s66_xor_checksum_signed_extend)
- **Neden olur:** C'de `char` çoğu platformda **işaretlidir**; 127'den büyük baytlar
  negatif sayılır. LLM baytları Rust'ta `u8` (0..255, hep pozitif) olarak topladı.
- **Kanıt:** `çğ` girdisi → C **−307**, Rust **717**. Bağımsız bir ikinci örnekte
  (s49_negative_byte_count, negatif bayt sayımı) aynı kök neden tekrarlandı:
  Türkçe metinde C negatif bayt sayısını doğru sayarken, Rust'ın `u8→i32`
  sıfır-genişletmeli çevirisi hep 0 üretti.
- **n=130 doğrulaması (kategorinin en tutarlı doğrulaması):** Eklenen üç yeni
  örneğin **üçü de** başarısız oldu — kategori C artık beş bağımsız örnekle
  doğrulanmıştır ve yeni örneklerde %100 tetiklenme oranına sahiptir:
  **s64_char_minmax_signed** (en küçük/en büyük bayt değeri) → C `-80 117`
  derken Rust `97 196`; **s65_ctype_isalpha_highbyte** (`isalpha()` ile
  127-üstü baytların sayımı) → C `0 3` derken Rust `6 3`; ve
  **s66_xor_checksum_signed_extend** (XOR sağlama toplamının işaretli
  genişletilmesi) → C `4294967247` derken Rust `207`. Üçü de aynı temel
  hatadan kaynaklanır: modelin baytları `u8`/`i32` olarak sıfır-genişletmesi,
  C'nin işaretli-`char` sözleşmesini yok sayması.
- **Düzeltme (Round 2):** her baytı `i8`'e çevirerek topla → PASS (beş
  örnekte de).

### D) Çıktı biçimlendirme semantiği (%g) → Fonksiyonel Hata (s15_float_avg, s27_csv_stats, s48_cjson_number, s67_stats_stddev_format, s68_currency_round_format, s69_sqlite_snprintf_g)
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
- **n=130 doğrulaması (kategorinin en güçlü doğrulaması):** Eklenen üç yeni
  örneğin **üçü de** başarısız oldu; kategori D artık **altı bağımsız örnekle
  ve dört ayrı kod tabanıyla** (kendi yazdıklarımız + cJSON + SQLite)
  doğrulanmıştır: **s67_stats_stddev_format** (standart sapma) → C
  `2 0.816497` derken Rust `2 0.816496580927726`;
  **s68_currency_round_format** (para birimi yuvarlama) → C `0.3` derken Rust
  `0.30000000000000004` — kayan noktalı sayının klasik ikili gösterim
  artığının C tarafından `%g` ile gizlenip Rust tarafından açığa
  çıkarılması; **s69_sqlite_snprintf_g** (SQLite'ın `snprintf` `%g` sarmalayıcısı,
  gerçek üretim kodu) → C `3.14286` derken Rust `3.142857142857143`. Bu,
  kategori D'nin veri setindeki **en yaygın ve en model-bağımsız kök neden**
  olduğunu pekiştirir (bkz. çoklu model bölümü: Gemini de yeni üç örneğin
  üçünde birden başarısızdır).
- **Düzeltme (Round 2):** `%g`yi genel olarak taklit eden, bilimsel gösterim
  dalı da içeren biçimlendirici → PASS (300 rastgele değerle ek olarak
  doğrulandı).

### E) Global durum → `static mut` → Derleme Hatası (s19_global_counter)
- **Neden olur:** C'nin global `static int` sayacı doğrudan Rust `static mut`'a taşındı.
  Rust'ta değiştirilebilir statik değişkene erişim `unsafe` gerektirir → **derlenmez**.
- **Kanıt:** `error[E0133]: use of mutable static is unsafe...`.
- **n=130 doğrulaması — kategori E artık bir "kural" değil, bir istisnadır:**
  Eklenen üç yeni örneğin (s70_global_lcg_rng — global tohumlu doğrusal
  eşleşik üreteç; s71_global_errbuf — global hata dizgesi tamponu;
  s72_global_log_level — global günlük seviyesi) **üçü de ilk seferde PASS
  oldu**. Kaynak dosyalar incelendiğinde model üçünde de yine C'ye sadık
  biçimde `static mut` seçmiş, ancak erişimleri doğru şekilde `unsafe { ... }`
  bloklarına sarmıştır (`translations_rust/s70-s72*.rs`). Daha önce eklenen
  s50_id_generator ile birlikte kategori E'nin **beş örneğinden dördü PASS**;
  yalnızca s19 başarısızdır. Sonuç: bu bir C↔Rust semantik boşluğu değil,
  **modelin aynı yapısal kalıpta tutarsız davranmasıdır** (5 denemede 1 kez
  `unsafe` sarmalamayı unutma). Ayrıca bu, veri setindeki tek CE örneğinin
  neden hâlâ tek olduğunu da açıklar.
- **Düzeltme (Round 2):** sayacı global değil, `&mut` parametre olarak geçir → PASS.

### F) Platforma bağlı tamsayı genişliğinin sabit varsayılması → Fonksiyonel Hata (s38_bsd_strtol, s51_long_clamp, s73_bsd_atoi_overflow, s74_platform_loop_counter, s75_bsd_strtoul, s103_nginx_hextoi) — gerçek üretim kodundan
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
- **n=130 doğrulaması — kategorinin en geniş ve platform açısından en kritik
  hale gelmesi:** Eklenen üç yeni hedefli örneğin **üçü de** başarısız oldu
  (**s73_bsd_atoi_overflow** → C `705032704` derken Rust `5000000000`;
  **s74_platform_loop_counter** → C `2147483647` derken Rust `2500000000`;
  **s75_bsd_strtoul**, 111 satırlık gerçek BSD libc kodu → C
  `result=4294967295 errno=ERANGE` derken Rust `result=5000000000 errno=OK`).
  Bunlara ek olarak, hedeflenmemiş gerçek üretim kodu katmanından
  **s103_nginx_hextoi** (nginx'in `ngx_hextoi()` fonksiyonu, BSD-2-Clause)
  aynı kök nedenden bağımsız biçimde başarısız oldu: C geçersiz/taşan girdide
  `-1` döndürürken Rust çevirisi `4294967295` üretti. Kategori F artık **altı
  bağımsız örnekle** temsil edilmektedir ve bunların ikisi (s38, s103) veri
  setinin tasarımında hiç öngörülmemiş gerçek kod tabanlarından gelmiştir.
- **⚠️ Platform duyarlılığı artık altı örneğe yayılmıştır:** Bu altı örneğin
  tamamı Windows (LLP64, 32-bit `long`) ile Linux (LP64, 64-bit `long`)
  arasında **PASS/FAIL durumunu tam olarak yer değiştirir** — Round 1'de
  Windows'ta başarısız olup Linux'ta geçerler, Round 2'nin Windows'a özgü
  `i32` "düzeltmesi" ise Linux'ta hepsini başarısız hale getirir (bkz.
  `platform_comparison.md`). Bu, "hata geri bildirimiyle düzeltilmiş bir
  çeviri evrensel olarak doğrudur" varsayımının yanlışlığına dair kanıtı 2
  örnekten 6 örneğe çıkarır.
- **Düzeltme (Round 2):** `i64` yerine bu platformun gerçek `long` genişliğini
  yansıtan `i32` kullanıldı → PASS (altı örnekte de, ancak yalnızca
  Windows'ta).

### G) İşaretsiz (usize) tip seçiminin yarattığı yeni taşma → Çalışma Zamanı Hatası (s40_diff_sum, s52_window_sum, s76_array_shrink_countdown, s77_ring_buffer_index, s78_sliding_window_min)
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
- **n=130 doğrulaması ve önceki bir gözlemin düzeltilmesi:** Eklenen üç yeni
  örneğin **üçü de** başarısız oldu (s76_array_shrink_countdown,
  s77_ring_buffer_index, s78_sliding_window_min — hepsi RE/panik), kategori
  G artık beş bağımsız örnekle doğrulanmıştır. **Ancak bu üç yeni örnek,
  yukarıda kategori tanımında yapılan "bu tür bir çökme derleme
  yapılandırmasıyla maskelenemez" gözlemini kısmen çürütmüştür:** release
  modunda s78 PASS'e, s76 ve s77 ise **FE'ye (sessiz yanlış çıktı)**
  dönüşmüştür. Yani maskelenememe, yalnızca taşan indeksin hemen ardından
  Rust'ın dizi sınır kontrolüne takıldığı durumlarda (s40, s52) geçerlidir;
  taşan değerin sınırlar içinde kalabildiği veya döngünün hiç
  çalışmadığı durumlarda taşma sessizce sarabilmektedir. Bu, kategori G'yi
  kategori A'dan daha tehlikeli kılar: aynı hata, derleme moduna göre hem
  gürültülü hem sessiz olabilmektedir.
- **Düzeltme (Round 2):** `n - 1` / `n - k` yerine `saturating_sub` → PASS
  (beş örnekte de).

### H) switch/case fallthrough'ın kaybolması → Fonksiyonel Hata (s43_switch_fallthrough, s53_tax_bracket, s79_http_status_class, s80_state_machine_fallthrough, s81_grade_bucket_fallthrough)
- **Neden olur:** C'nin `switch`'i `break` konulmadığında bilinçli olarak bir
  sonraki case'e düşer (level=4 için 8+4+2+1=15 bonus birikir). Rust'ın
  `match`'i varsayılan olarak düşmez. LLM her seviyeyi yalnızca kendi
  (kümülatif olmayan) katkısıyla eşleştirdi (level=4 için yanlışlıkla 8 döndü).
  Bağımsız bir ikinci örnekte (s53_tax_bracket, kümülatif vergi dilimi) aynı
  kök neden farklı bir sayısal senaryoda tekrarlandı.
- **n=130 doğrulaması:** Eklenen üç yeni örneğin **üçü de** başarısız oldu
  (**s79_http_status_class** → beklenen `7`, alınan `4`;
  **s80_state_machine_fallthrough** → beklenen `2`, alınan `1`;
  **s81_grade_bucket_fallthrough** → beklenen `4`, alınan `1`). Kategori H
  artık beş bağımsız örnekle doğrulanmıştır ve yeni örneklerde %100
  tetiklenme oranına sahiptir. Özellikle s80, fallthrough'un yalnızca
  kümülatif sayısal birikimde değil **durum makinesi geçişlerinde** de
  kullanıldığını ve modelin bu bağlamda da düşme davranışını
  yeniden üretmediğini gösterir.
- **Düzeltme (Round 2):** her `match` kolu, karşılık geldiği case zincirinin
  toplam katkısını açıkça içerecek şekilde yeniden yazıldı → PASS (beş
  örnekte de).

### I) Makro çoklu-değerlendirme yan etkisi → Fonksiyonel Hata (s56_macro_table, s82_macro_minmax_sideeffect, s83_macro_swap_no_temp)
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
- **n=130 doğrulaması:** Eklenen üç yeni örnekten ikisi başarısız oldu ve
  bunlar kategoriyi **yeni bir yan etki türüne** genişletti:
  **s82_macro_minmax_sideeffect** (`MAX`/`MIN` makrolarına `x++` benzeri
  yan etkili argüman) → beklenen `6 10 8`, alınan `5 10 7`; ve daha ilginç
  olan **s83_macro_swap_no_temp** → burada yan etkili olan makro
  argümanının *değeri* değil **erişim yolunun kendisidir**
  (`SWAP(arr[i++], arr[1], tmp)`): C'de `a` makro gövdesinde iki kez geçtiği
  için `i` iki kez artar ve ikinci atama **farklı bir dizi elemanına** yazar
  (beklenen `10 10 30 40 50 / i=2`, alınan `20 10 30 40 50 / i=1`). Üçüncü
  yeni örnek (s84_xmacro_enum_strings, saf X-Macro/token-pasting) PASS oldu
  — bu, s56'da ilk kez gözlenen ayrımı üçüncü kez doğrular: **sorun karmaşık
  makro kullanımının kendisi değil, yalnızca yan etkili bir argümanın çoklu
  genişletilmesidir.**
- **Düzeltme (Round 2):** C'nin metinsel ikame semantiğini kasıtlı olarak
  yeniden üreten bir Rust `macro_rules!` tanımı yazıldı → PASS (üç örnekte de).

### Neden hiç "Sonlanmama (NT)" görülmedi?
Bu veri setinde sonsuz döngüye yol açan bir çeviri hatası oluşmadı (NT=0). Not:
`s32_levenshtein` (gerçek dünyadan, özyinelemeli/memoizasyonsuz) NT riski taşıyan
bir adaydı — büyük girdilerde katlanarak yavaşlar — ancak test girdileri kısa
tutulduğu için (≤7 karakter) zaman aşımına yaklaşmadı. NT tipik olarak döngü
koşulunun yanlış çevrilmesiyle ortaya çıkar (örn. `i <= n` yerine `i < n` sınır
hatası) ya da — bu örnekte olduğu gibi — üstel karmaşıklıklı özyinelemeli
algoritmaların büyük girdilerle beslenmesiyle. Taksonomiye tamlık için dahildir;
daha büyük/karmaşık kodlarda görülme olasılığı artar (gelecek çalışma).
**n=130 güncellemesi:** Veri seti 57'den 130'a çıkarıldıktan sonra da NT=0
olarak kalmıştır — 73 yeni örnek arasında döngü sınırı, `goto` ile yeniden
deneme döngüsü (s117), graf dolaşımı (s124, s125) ve iş parçacığı havuzu
(s114) gibi NT'ye aday desenler bulunmasına rağmen hiçbirinde zaman aşımı
gözlenmemiştir. Bu, NT'nin taksonomide teorik bir kategori olarak kalmaya
devam ettiğini ve bu ölçekte bile gözlenmediğini gösterir.

## Bulgulardan Çıkan Ana Gözlemler (Tartışma için)

1. **Claude Sonnet 5 için bile ham çeviri kusursuz değil (%70.77)** — ama
   başarısızlıklar sözdizimsel değil, **semantik**. n=57'den n=130'a
   genişletmede bu oran neredeyse hiç değişmemiştir (%70.18 → %70.77), yani
   bulgu örneklem büyüklüğüne karşı dayanıklıdır.
2. **Tehlike sessiz semantik hatalardadır:** 38 başarısızlığın 37'si sorunsuz
   derlendi (%97.4); bunların 28'i (%73.7) gerçekten sessizdir (FE), 9'u
   gürültülü RE'dir. Yalnızca derleme başarısına dayanan bir kabul kriteri
   bu hataların hiçbirini yakalayamaz (Şekil 3, Şekil 5).
3. **⚠️ Kod uzunluğu bulgusu n=130'da tersine döndü — ama nedeni veri seti
   tasarımıdır:** n=57'de Mann-Whitney U testi anlamlı bir fark
   bulamamıştı (U=287.0, p=0.359). n=130'da test artık **anlamlıdır**
   (U=924.0, **p<0.0001**, r=0.471, orta etki), ancak yön sezgiye zıttır:
   **başarısız örnekler daha kısadır** (FAIL medyan 25.0 vs PASS medyan 53.0
   satır). Bu, "kısa kod daha zordur" anlamına gelmez; **veri setinin
   inşasından kaynaklanan bir karıştırıcı değişkendir**: kök-neden
   kategorilerini derinleştirmek için eklenen s58-s84 katmanı kasıtlı olarak
   kısa ve hataya özel tasarlanmış programlardan oluşur (20/27 FAIL), buna
   karşılık s85-s130 katmanı uzun gerçek üretim/çeşitlilik kodundan oluşur
   (45/46 PASS). Dolayısıyla **asıl bulgu değişmemiştir** — başarısızlık kod
   uzunluğuyla değil, belirli bir semantik boşluğun tetiklenip
   tetiklenmemesiyle ilişkilidir — ama makalede artık "anlamlı ilişki
   yoktur" DENMEMELİ; bunun yerine "gözlenen anlamlı ilişkinin yönü
   örnekleme tasarımının bir yan ürünüdür" biçiminde, karıştırıcı açıkça
   belirtilerek raporlanmalıdır. Aynı karıştırıcı, işaretçi kullanımı için
   yapılan Fisher kesin testini de n=130'da anlamlı hale getirmiştir
   (OR=5.77, p<0.0001, %95 GA=[2.54, 13.09]) — işaretçi kullanan kod daha
   çok geçmektedir, çünkü işaretçi kullanan kod ağırlıklı olarak uzun
   gerçek üretim kodudur (Şekil 4, Tablo 4).
4. **Bağımsız kaynaklı gerçek kod belirgin biçimde daha güvenilir çevriliyor
   (n=130'da çok daha güçlü kanıt):** Veri setindeki 38 bağımsız gerçek dünya
   algoritmasının **35'i (%92.1)** ilk seferde geçti; buna karşılık kök-neden
   kategorilerini hedefleyen s58-s84 katmanının yalnızca 7/27'si (%25.9)
   geçti. Bu keskin ayrım, veri setindeki başarısızlıkların ezici
   çoğunluğunun **veri setini tasarlayanın bilinçli olarak hedeflediği
   belirli semantik boşluklara özgü** olduğunu n=130 ölçeğinde doğrular.
   Gerçek koddan gelen üç başarısızlık (s38 BSD libc, s48 cJSON, s103 nginx)
   veri setinin tasarımının hiç öngörmediği kod tabanlarından, birbirinden
   bağımsız biçimde ortaya çıktı — s48 kategori D'yi, s103 ise kategori F'yi
   tamamen bağımsız kod tabanlarında doğrulayarak bunların kendi veri
   setimize özgü tuhaflıklar değil **sistematik C↔Rust boşlukları** olduğunu
   gösterir. Uyarı: Rosetta Code alt kümesi (7 örnek) temiz/izole eğitim
   kodudur; gerçek üretim kodu alt kümesi (31 örnek) artık istatistiksel
   olarak anlamlı bir büyüklüğe ulaşmıştır, ancak hepsi tek dosyaya
   indirgenebilen, kendi içinde kapalı fonksiyonlardır — derin modül
   hiyerarşisine veya build-sistemi bağımlılığına sahip gerçek endüstriyel
   projelere hâlâ genellenemez.
5. **İyileştirme döngüsü işe yarıyor (üst sınır olarak) — artık ölçülmüş kanıtla:**
   hata geri bildirimi EA'yı %70.77 → %100 taşıdı; ancak bu, modele zengin bir
   hata-oracle'ı verildiğinde elde edilen bir üst sınır performansıdır. Bunu
   varsayım olmaktan çıkarmak için iki kısıtlı geri bildirim seviyesi de ayrıca
   ölçüldü: n=57'nin kör protokolünde orta ayrıntıda (Seviye B) %85.96,
   minimal ayrıntıda (Seviye C) %71.93 — geri bildirim zenginliği azaldıkça
   doğruluk doğrudan düşüyor. n=130'da aynı yönlü düşüş sürüyor (%93.85,
   %87.69); kör protokol artık n=130'un tamamında (17 eski + 21 yeni
   başarısızlık) gerçekten uygulanmıştır, bu iki sayı da gerçek ölçümdür
   (bkz. yukarıdaki düzeltme notu). Literatürdeki
   feedback-based yaklaşımları (Gandhi vd. 2024, Eniser vd. 2024/FLUORINE)
   deneysel olarak destekler.
6. **Semantik boşluklar tekrar eden bir örüntü — ve taksonomi doyuma ulaştı:**
   taşma, string modeli, işaretlilik, biçimlendirme, global durum, tamsayı
   genişliği, usize taşması, switch fallthrough, makro çoklu-değerlendirme —
   her biri C→Rust'ın klasik tuzağı. **n=130'a genişletmenin en önemli
   yapısal bulgusu, 73 yeni örneğin hiçbirinin onuncu bir kök-neden
   kategorisi ortaya çıkarmamış olmasıdır**; bunun yerine mevcut dokuz
   kategorinin her biri artık 3-6 bağımsız örnekle temsil edilmektedir
   (kategori E hariç — o artık bir kural değil, modelin tutarsızlığından
   kaynaklanan bir istisna olarak görünmektedir). Bu, taksonominin bu kod
   sınıfı için doyuma (saturation) ulaştığına dair ilk somut işarettir.
7. **Derleme yapılandırması gizli değişkendir — ve bazen hatayı
   sessizleştirir:** aynı çeviri debug'da %70.77, release'de %74.62. Bu fark
   ağırlıklı olarak taşma kaynaklı RE örneklerinden kaynaklanır, ancak n=130
   yeni bir nüans ortaya çıkardı: release modunda iki örnek (s76, s77 —
   kategori G) PASS'e değil **FE'ye** dönüşür, yani gürültülü bir çökme
   sessiz bir yanlış sonuca dönüşür (bu yüzden release'de FE sayısı 28'den
   30'a çıkar). Doğruluk raporlanırken derleme modu mutlaka belirtilmelidir.
8. **Derleme başarı oranı yanıltıcı olabilir (n=130'da daha güçlü kanıt):**
   veri setindeki programların çoğunluğu hâlâ tek dosyalıdır, ancak artık 8
   çok dosyalı/gerçekçi örnek (s54, s55, s57, s110-s114 — paylaşılan başlık
   dosyası, birden fazla derleme birimi, pthread tabanlı paylaşılan bellek
   eşzamanlılığı, rwlock, iş parçacığı havuzu) bulunmaktadır ve
   **hiçbirinde derleme hatası gözlenmemiştir (8/8 PASS)**. Bu, "çok dosyalı
   yapı CE oranını artırır" hipotezini n=8 ölçeğinde de doğrulamamaktadır;
   yine de çok daha derin modül hiyerarşilerine sahip gerçek projelerde
   derleme hataları daha sık görülebilir.
9. **Bellek güvenliği/`unsafe` bulgusu n=130'da güçlendi ve netleşti:**
   Round 1'in 130 çevirisinin yalnızca **7'sinde** (s37_bsd_getopt,
   s44_fib_memo_static, s46_musl_qsort, s50_id_generator, s70_global_lcg_rng,
   s71_global_errbuf, s72_global_log_level) gerçek `unsafe` kullanıldı
   (%5.4) — yedisi de tam olarak C'nin dışa açık global durum,
   fonksiyon-lokal kalıcı durum veya ham byte-pointer aritmetiği
   sözleşmesinin bunu yapısal olarak gerektirdiği yerlerdedir. Yeni eklenen
   üçü (s70-s72) kategori E'nin yeni örnekleridir ve **`unsafe`'in doğru
   kullanımının aslında bir başarı göstergesi olduğunu** ortaya koyar: bu üç
   örnek `unsafe` kullandıkları için PASS olmuştur, s19 ise kullanmadığı için
   CE vermiştir. Buna karşılık ham işaretçi aritmetiğine/paylaşılan belleğe
   yapısal olarak bağımlı örnekler — **s39_bsd_heapsort** (generic void*
   sıralama), **s47_redis_sds** (pointer-öncesi gizli başlık düzeni),
   **s57/s112/s113/s114** (pthread + mutex/rwlock ile gerçek paylaşılan
   bellek eşzamanlılığı, `Arc<Mutex<>>`/`RwLock` desenleriyle çevrildi) ve
   yeni üretim kodu katmanındaki `memmem`/`strsep`/`strlcpy` gibi klasik
   pointer-yoğun libc fonksiyonları — hiç `unsafe` kullanmadan,
   güvenli/deyimsel bir yeniden yazımla çevrildi. Bu, LLM'in `unsafe`
   kullanımının kodun ham karmaşıklığından değil, C'nin dışa açık
   sözleşmesinin (harici mutable durum, fonksiyon ömrü boyunca kalıcı durum)
   doğasından etkilendiğini n=130 ölçeğinde doğrular.
10. **Çoklu model ve çoklu platform:** Bulgular tek modele özgü değildir ama
   model×kategori etkileşimi belirleyicidir (bkz. aşağıdaki çoklu model
   bölümü): Claude %70.77, Claude Haiku %72.31 (130/130), Gemini %86.87
   (99/130, kısmi kapsam). Platform tarafında ise Round 2'nin Windows'taki
   %100'ü Linux'ta %94.62'ye düşmektedir — kategori F'nin altı örneğinin
   tamamı platformlar arasında PASS/FAIL yer değiştirir.

## Çoklu Model Karşılaştırması (n=130)

Üç model, aynı 130 program üzerinde, aynı zero-shot istemle ölçüldü:

| Model | Kapsam | EA (örnek) | EA % | CE | RE | FE | NT |
|---|---|---|---|---|---|---|---|
| Claude Sonnet 5 (referans, Round 1) | 130/130 | 92/130 | %70.77 | 1 | 9 | 28 | 0 |
| Claude Haiku | 130/130 | 94/130 | %72.31 | 8 | 5 | 23 | 0 |
| Google Gemini (`gemini-flash-latest`) | **99/130 (kısmi)** | 86/99 | %86.87 | 9 | 0 | 4 | 0 |

**⚠️ Gemini kapsamı kısmidir:** Google AI Studio ücretsiz katmanının günlük
kota sınırı (20 istek/gün/model) nedeniyle 130 örneğin 99'u çevrilebilmiştir;
kalan 31 örnek (s80-s84, s85-s99 ve s120-s130) kota sıfırlandıkça kademeli
olarak tamamlanmaktadır. Gemini'nin %86.87'si bu 99
örneklik alt küme üzerinden hesaplanmıştır ve diğer iki modelin tam-kapsam
sayılarıyla **doğrudan karşılaştırılamaz**.

**Sessiz hata oranı modele göre belirgin biçimde değişiyor** — bu, makalenin
en aktarılabilir bulgularından biridir:
- Claude Sonnet 5: 38 başarısızlığın 37'si derlenir (%97.4 sessiz veya
  gürültülü ama derlenen); yalnızca 1'i derlemede yakalanır.
- Claude Haiku: 36 başarısızlığın 8'i derlemede yakalanır (%22.2), 28'i
  derlenir.
- Gemini: 13 başarısızlığın 9'u derlemede yakalanır (%69.2), yalnızca 4'ü
  sessizdir.

Yani Gemini daha yüksek ham doğruluk gösterse de, **hataları büyük ölçüde
derleyici tarafından yakalanan türdendir**; Claude Sonnet 5'in hataları ise
neredeyse tamamen derleyiciden kaçan sessiz semantik hatalardır. Bir CI
hattında bu iki hata profilinin pratik riski taban tabana zıttır.

**Kategori D iki modelde de kör nokta:** Gemini, kategori D'nin üç yeni
örneğinin (s67, s68, s69) **üçünde de** başarısızdır (üçü de CE — `%g`
taklidini yazarken geçersiz Rust biçim dizesi üretmiştir). Kategori D artık
altı örnek ve iki bağımsız model boyunca doğrulanmış, veri setindeki **tek
gerçekten model-bağımsız kör noktadır**.

**Kategori F'de Gemini ilk kez düştü:** Önceki turlarda Gemini, `long` tipini
`std::os::raw::c_long` ile çevirerek s38/s51'i taşınabilir biçimde geçmişti.
Yeni eklenen **s103_nginx_hextoi**'de ise Gemini de Claude ile aynı hataya
düşmüştür (FE) — yani `c_long` tercihi tutarlı bir çözüm değil, örneğe bağlı
bir tesadüftür.

**McNemar testi (Claude vs Gemini):** Ortak değerlendirilen 78 örnek
üzerinden — ikisi de PASS 43, ikisi de FAIL 7, yalnızca Claude FAIL 25,
yalnızca Gemini FAIL 3; McNemar kesin iki-yönlü p<0.0001.
**⚠️ Bu analiz geçicidir:** Gemini kapsamı kısmi olduğu için ortak örnek
sayısı (78) nihai değildir ve **Gemini'nin kalan 31 örneği tamamlandığında
yeniden hesaplanıp güncellenmesi gerekmektedir.**

## Çoklu Platform Karşılaştırması (n=130)

Windows (MSYS2/UCRT64 gcc 16.1.0, `long`=32-bit/LLP64) ile Linux (Docker
ubuntu:24.04, gcc 13.3.0, `long`=64-bit/LP64) — **her iki ortamda da rustc
1.97.1, birebir aynı sürüm/commit**, bu yüzden gözlenen tüm fark C tarafındaki
`long` genişliğinden ve stdio davranışından kaynaklanır:

| Koşul | Windows EA | Linux EA | Fark |
|---|---|---|---|
| Round 1 — doğrudan, debug | %70.77 (92/130) | %74.62 (97/130) | +3.85 puan |
| Round 1 — doğrudan, release | %74.62 (97/130) | %78.46 (102/130) | +3.84 puan |
| Round 2 — iyileştirilmiş, debug | **%100.00 (130/130)** | **%94.62 (123/130)** | **−5.38 puan** |

Platforma göre farklılaşan örnek sayısı 3'ten **7'ye** çıkmıştır: altısı
kategori F'nin tamsayı-genişliği örnekleridir (s38, s51, s73, s74, s75 ve
yeni **s103_nginx_hextoi**) — hepsi Round 1'de Windows'ta FAIL/Linux'ta PASS
iken, Round 2'nin Windows'a özgü `i32` düzeltmesinden sonra tam tersine
döner. Yedincisi **s47_redis_sds**'tir ve nedeni farklıdır: test girdisindeki
CRLF satır sonu yüzünden **C referansının kendisi** platformlar arasında
taşınabilir değildir (Windows CRT'si stdio'yu metin modunda açıp `\r\n`'i
`\n`'e çevirir, Linux/glibc çevirmez) — burada "kırılan" taraf Rust çevirisi
değil C kaynağıdır. Ayrıntı: `results/platform_comparison.md`.

## Figürler (results/figures/)
- `fig2_execution_accuracy.png` — Şekil 2: Üç koşulda EA (%70.77 → %74.62 → %100).
- `fig3_error_distribution.png` — Şekil 3: Koşullara göre yığılmış sonuç dağılımı.
- `fig4_loc_vs_success.png` — Şekil 4: Kod uzunluğu ile başarı ilişkisi (10-522 satır).
- `fig5_rootcause.png` — Şekil 5: Başarısızlıkların kök-neden dağılımı (9 kategori).
- `fig4b_bootstrap_ci.png` — Şekil 4b: Üç koşulda EA + bootstrap %95 güven aralığı.

> Not: Figürler `harness/make_figures.py` ile üretilir ve güncel n=130
> verisinden yeniden üretilmelidir; yukarıdaki EA değerleri n=130'un gerçek
> sayılarıdır.

# Detaylı Sorun Analizi: C→Rust Çevirisinde Yaşanan Tüm Hatalar

Bu belge, makaleden **bağımsız**, yalnızca deneyde karşılaşılan sorunları
derinlemesine incelemek için hazırlanmıştır. **57 örneğin tamamı** ele
alınmıştır: Round 1'de başarısız olan 17 örneğin her biri (ne yapılmaya
çalışıldı → ne oldu → neden oldu → nasıl çözüldü) ve ilk seferde geçen 40
örneğin her biri (ne test ediliyordu → neden sorunsuz geçti → dikkat çekici
yönü varsa), sonrasında yapılan tüm ek deneyler (release/debug, çoklu
platform, çoklu model, kısıtlı geri bildirim, bellek güvenliği) ve bunlardan
çıkan genel örüntüler ele alınmaktadır. Tüm sayılar gerçek harness
çalıştırmalarından (`results/*.json`) alınmıştır; hiçbir rakam uydurulmamıştır.

---

## 1. Genel Çerçeve

**Veri seti:** 57 C programı, 229 test girdisi, 10-522 satır arası. Programlar
dokuz kaynak/rol kategorisine ayrılır (bkz. `VERISETI_VE_ALGORITMALAR.md`).
**Çeviri:** Claude Sonnet 5, zero-shot (tek geçiş, hiçbir insan düzeltmesi
veya derleme/test geri bildirimi olmadan).
**Değerlendirme:** Diferansiyel test — C referansı ile Rust çevirisi aynı
girdilerde çalıştırılır, çıktılar karşılaştırılır.

**Round 1 (ham çeviri, debug modu) sonucu:** 40/57 = %70.18 EA (Execution
Accuracy). 17 örnek başarısız: 1 CE (derleme hatası), 4 RE (çalışma zamanı
hatası/panik), 12 FE (fonksiyonel hata — derlenip çalışıyor ama çıktı yanlış).

Bu 17 başarısızlığın **her biri** §2'de, geri kalan 40 PASS örneğin **her biri** §3'te tek tek ele alınmıştır.

---

## 2. Başarısız Örneklerin Tam Vaka Analizi (17/17)

Her vaka dört soruya yanıt verir: **Ne yapılıyordu? → Ne oldu? → Neden oldu? → Nasıl çözüldü?**

### 2.1 s09_djb2_hash — Çalışma Zamanı Hatası (RE)

- **Ne yapılıyordu:** djb2 karma (hash) fonksiyonu — `hash = hash * 33 + byte` formülüyle bir dizgiyi 32-bit tamsayıya indirger (17 satır, 3 test).
- **Ne oldu:** 3 testin 3'ü de çöktü. Beklenen çıktı `261238937`/`2704141334` iken Rust programı hiç çıktı üretmeden panikledi (`attempt to multiply with overflow`).
- **Neden oldu:** C standardına göre **unsigned tamsayı taşması tanımsız davranış değildir** — mod 2³² sarma (wraparound) olarak kuralla tanımlıdır ve djb2 gibi karma fonksiyonları bu garantiye kasıtlı olarak dayanır. LLM, çarpmayı `hash = hash * 33 + b as u32` biçiminde, yani Rust'ın varsayılan (debug modda taşmada panik veren) aritmetik operatörüyle yazdı. Kaynağın "sessizce sar" sözleşmesini korumak için Rust'ın açık `wrapping_mul`/`wrapping_add` yapısı gerekiyordu — model bunu kullanmadı.
- **Nasıl çözüldü (Round 2):** `hash.wrapping_mul(33).wrapping_add(b as u32)`. Test edilince 3/3 PASS.
- **Ek not:** Release modda (taşma kontrolü kapalı) bu hata kendiliğinden kaybolur çünkü rustc aynı operatörü ikinin tümleyeni yöntemiyle sarar — ama bu bir düzeltme değildir, yalnızca farklı bir derleme yapılandırmasında tesadüfen doğru sonuç.
- **Kısıtlı geri bildirimle düzeltilebilir mi?** Evet, Seviye B'de (yalnızca panik metni verildiğinde) düzeltildi — panik metni "attempt to multiply with overflow" tasma türünü açıkça belirtiyor.

### 2.2 s14_fnv_hash — Çalışma Zamanı Hatası (RE)

- **Ne yapılıyordu:** FNV-1a karma fonksiyonu, djb2 ile aynı ailede farklı sabitler kullanan ikinci bir unsigned-taşma örneği (17 satır, 3 test).
- **Ne oldu:** 3/3 test çöktü (beklenen `1335831723`/`22495912`, alınan: panik).
- **Neden oldu:** s09 ile birebir aynı kök neden — çarpma/XOR zincirinde varsayılan (panik-eden) aritmetik kullanılmış.
- **Nasıl çözüldü:** `wrapping_mul` (Round 2'de 3/3 PASS).
- **Kısıtlı geri bildirim:** Seviye B'de düzeltildi (aynı gerekçe).
- **Önemi:** İki bağımsız örnekte aynı kök nedenin görülmesi, bu boşluğun rastlantısal değil sistematik olduğunu doğrular.

### 2.3 s06_reverse_string — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** Bir dizgiyi ters çevirme (19 satır, 4 test).
- **Ne oldu:** ASCII girdilerde 3/4 test geçti; çok baytlı (Türkçe: "çğıöşü" gibi) girdide (04.txt) çıktı farklıydı — beklenen bayt-tersine-çevrilmiş dizi, alınan Unicode-karakter-tersine-çevrilmiş dizi (görsel olarak farklı bayt dizilimi).
- **Neden oldu:** C dizeleri **bayt düzeyinde** işlenir. LLM'in ürettiği "idiyomatik" Rust çevirisi `.chars().rev()` kullandı — bu **Unicode karakter düzeyinde** ters çevirir. ASCII'de bayt=karakter olduğundan fark görünmez, ama UTF-8 çok baytlı karakterlerde (`ç` = 2 bayt) iki yöntem farklı sonuç verir.
- **Nasıl çözüldü:** Bayt düzeyinde işleme (`bytes().rev()` mantığı). Round 2: 4/4 PASS.
- **Kısıtlı geri bildirim:** Seviye B'de düzeltildi — başarısız girdinin çok baytlı Türkçe karakterler içerdiğinin görülmesi, ilgili semantik boşluğu (string modeli) tahmin etmeye yetti.

### 2.4 s13_word_count — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** `wc` benzeri kelime/karakter sayımı (19 satır, 4 test).
- **Ne oldu:** 3/4 geçti; 03.txt'de beklenen "3 20" (3 kelime, 20 bayt), alınan "3 17" (3 kelime, 17 karakter).
- **Neden oldu:** s06 ile aynı kök neden (Kategori B: string modeli) — `input.chars().count()` Unicode karakter sayar, C ise bayt sayar. Çok baytlı girdide (20 bayt = 17 gerçek karakter, çünkü bazı karakterler 2 bayt) sayılar ayrışır.
- **Nasıl çözüldü:** Bayt sayımı. Round 2: 4/4 PASS.
- **Kısıtlı geri bildirim:** Seviye B'de düzeltildi (aynı gerekçe — çok baytlı girdi ipucu).

### 2.5 s20_char_sum — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** Bir dizgideki bayt değerlerinin toplamı (17 satır, 3 test).
- **Ne oldu:** 2/3 geçti; 03.txt'de ("çğ" girdisi) beklenen `-307`, alınan `717`.
- **Neden oldu:** C'de `char` çoğu platformda **işaretlidir** (signed); 127'den büyük bayt değerleri negatif sayılır. LLM baytları Rust'ta `u8` (0-255, hep pozitif) olarak topladı — kaynağın işaretli-char sözleşmesini yok saydı.
- **Nasıl çözüldü:** Her baytı önce `i8`'e (işaretli) çevirip sonra topla. Round 2: 3/3 PASS.
- **Kısıtlı geri bildirim:** Seviye B'de düzeltildi (çok baytlı girdi ipucu → char işaretliliği tahmini).

### 2.6 s15_float_avg — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** Sayı ortalaması hesaplayıp `%g` formatıyla yazdırma (17 satır, 4 test).
- **Ne oldu:** 3/4 geçti; 02.txt'de (7/3 ortalaması) beklenen `2.33333`, alınan `2.3333333333333335`.
- **Neden oldu:** C'nin `%g` biçimlendiricisi **6 anlamlı basamağa** göre biçimlenir, sondaki sıfırları atar ve belirli eşiklerin dışında bilimsel gösterime geçer. LLM, Rust'ın varsayılan `{}` biçimini kullandı — bu tam kayan-nokta hassasiyetini (17 basamağa kadar) olduğu gibi yazdırır, hiçbir zaman kırpmaz.
- **Nasıl çözüldü:** `%g` davranışını taklit eden özel bir `format_g` fonksiyonu yazıldı. Round 2: 4/4 PASS. **Önemli ayrıntı:** İlk yazılan düzeltici yalnızca ≥1 değerler için doğru basamak sayısı hesaplıyordu; aynı kök nedenin tekrarlandığı s27'nin ortalaması 1'in altında olduğunda bu "düzeltilmiş" kod da yanlış sonuç verdi ve genelleştirilmesi gerekti (dar test girdileriyle doğrulanan bir düzeltmenin bile eksik kalabileceğinin kanıtı).
- **Kısıtlı geri bildirim:** Seviye B ve C'de düzeltilemedi — başarısız girdi yalnızca sayılardan oluştuğu için biçimlendirme hatasına dair gözlemlenebilir hiçbir ipucu yok.
- **Bağımsız model doğrulaması:** Gemini de aynı örnekte, aynı kök nedenden başarısız oldu (57/57 tam ölçümde, Kategori D'nin üç bağımsız kod tabanının hepsinde iki modelin de ortak düştüğü kör nokta — bkz. §7.4) — bu boşluğun modele özgü olmadığının kanıtı.

### 2.7 s27_csv_stats — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** CSV ayrıştırma + istatistik (ortalama, min, max) — 73 satır, 3 test.
- **Ne oldu:** 2/3 geçti; 03.txt'de beklenen `avg=0.285714`, alınan `avg=0.2857142857142857`.
- **Neden oldu:** s15 ile **birebir aynı kök neden** (Kategori D, `%g` biçimlendirme) — veri setine sonradan, bağımsız olarak eklenen bu örnekte de aynı boşluk tekrarlandı.
- **Nasıl çözüldü:** Aynı `format_g` yaklaşımı, ama s15 için yazılan ilk sürüm (yalnızca ≥1 değerler için doğru) burada yetersiz kaldı çünkü 2/7≈0.286 < 1; genelleştirilmiş `format_g` ile Round 2: 3/3 PASS.
- **Kısıtlı geri bildirim:** Düzeltilemedi (aynı gerekçe — sade sayısal girdi, ipucu yok).
- **Önemi:** İki bağımsız örnekte (biri ilk tasarımda, biri sonradan eklenmiş) aynı hatanın görülmesi, bu boşluğun rastlantısal değil sistematik olduğunu doğrular; üçüncü kez cJSON'da (s48) tekrarlanması bunu daha da pekiştirir.

### 2.8 s19_global_counter — Derleme Hatası (CE)

- **Ne yapılıyordu:** Global bir statik sayaç tutan, her çağrıda arttıran bir fonksiyon (21 satır, 3 test).
- **Ne oldu:** Hiç derlenmedi. Gerçek derleyici hatası: `error[E0133]: use of mutable static is unsafe and requires unsafe function or block` (`CALL_COUNT += 1;` satırında).
- **Neden oldu:** C'nin `static int` global sayacı doğrudan Rust `static mut`'a taşındı. Rust'ta değiştirilebilir statik değişkene erişim, veri yarışı riski taşıdığından `unsafe` gerektirir — model bunu eklemedi.
- **Nasıl çözüldü:** Sayacı global değişken olarak değil, fonksiyona `&mut` parametre olarak geçirme (güvenli, deyimsel çözüm — `unsafe` eklemek yerine tasarımı değiştirme). Round 2: 3/3 PASS.
- **Kısıtlı geri bildirim:** Hem Seviye B hem Seviye C'de düzeltildi — bu veri setindeki **tek CE örneği**; bir CI derleme adımı hatayı hiçbir zaman gizleyemeyeceğinden, en minimal geri bildirim koşulunda (Seviye C) dahi derleyici çıktısı zaten görünür kabul edildi.

### 2.9 s38_bsd_strtol — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** OpenBSD'nin gerçek üretim `strtol()` fonksiyonu — dize→tamsayı dönüştürücü, taşma durumunda `ERANGE` bildirir (154 satır, 4 test).
- **Ne oldu:** 3/4 geçti; 02.txt'de büyük girdilerde (`99999999999`) beklenen `result=2147483647 errno=ERANGE`, alınan `result=99999999999 errno=OK` (hiç taşma bildirilmedi).
- **Neden oldu:** C'nin `long` tipinin genişliği **platforma bağlıdır**: bu derleme ortamında (Windows/LLP64, MSYS2 gcc) 32 bit, Linux/LP64'te 64 bit. LLM, `long`'u — birçok modern sistemde geçerli yaygın bir varsayımla — 64-bit `i64` olarak çevirdi. Bu derleme ortamının gerçek 32-bit sınırını aşan girdilerde, C referansı taşma bildirirken (`ERANGE`, sonucu `2147483647`'e sabitler), `i64` tabanlı çeviri hiç taşmaz.
- **Nasıl çözüldü:** `i64` yerine, bu platformun gerçek `long` genişliğini yansıtan `i32` kullanıldı. Round 2: 4/4 PASS (bu derleme ortamında).
- **Kısıtlı geri bildirim:** Düzeltilemedi (Seviye B/C) — girdi büyük sayılar içerse de, beklenen kırpma/taşma davranışı (beklenen/alınan farkı) gösterilmeden doğru genişlik tahmin edilemedi.
- **⚠️ Kritik ek bulgu (Linux/Docker tekrarı):** Bu "düzeltme" **platforma özgüdür**! Aynı deney gerçek bir Linux/LP64 ortamında (Docker, Ubuntu 24.04) tekrarlandığında, Linux'ta C'nin `long`'u gerçekten 64-bit olduğundan, Windows için yazılan bu `i32` "düzeltmesi" Linux'ta **yanlış sonuç üretmeye başladı** (gereksiz yere 32-bit sınırında kırpıyor) — iki platform arasında PASS/FAIL durumu tam olarak yer değiştirdi. Bu, "hata geri bildirimiyle düzeltilmiş bir çeviri evrensel olarak doğrudur" varsayımının yanlış olabileceğinin doğrudan kanıtıdır.

### 2.10 s40_diff_sum — Çalışma Zamanı Hatası (RE)

- **Ne yapılıyordu:** Ardışık fark toplamı — `n` elemanlı bir dizide `i < n-1` aralığında dolaşma (27 satır, 4 test).
- **Ne oldu:** 3/4 geçti; 01.txt'de (`n=0` durumu) beklenen çıktı `0`, alınan: panik (`attempt to subtract with overflow`).
- **Neden oldu:** C referansı `n`'yi işaretli `int` tutar; `n==0` iken `i < n-1` karşılaştırması (`0 < -1`) güvenle yanlış olur, döngü hiç çalışmaz. LLM, "dizi boyutu" kavramını Rust'a çevirirken idiyomatik bir tercih olan `usize` (işaretsiz) tipini seçti; bu durumda `n==0` iken `n - 1` ifadesi **usize altında taşar** (underflow) ve debug modda panikler.
- **Nasıl çözüldü:** `n - 1` yerine `n.saturating_sub(1)` — `n==0` durumunda döngü aralığı boş (`0..0`) olur. Round 2: 4/4 PASS.
- **İlginç ayrıntı:** Bu panik, release modunda dahi ortadan kalkmaz — taşma kontrolü kapatıldığında `n - 1` sessizce `usize::MAX`'e sarar, ama hemen ardından dizi erişimi Rust'ın her zaman uyguladığı sınır kontrolüne takılıp yine panikler. Yani bu tür bir çökme derleme yapılandırmasıyla "maskelenemez" (Kategori A'daki aritmetik taşma çökmelerinin aksine).
- **Kısıtlı geri bildirim:** Seviye B'de düzeltildi — panik metni tam olarak "attempt to subtract with overflow" tasma türünü belirtiyor.

### 2.11 s43_switch_fallthrough — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** Kademeli bonus hesaplama — `switch` ile `case 4`'ün `case 3, 2, 1` bloklarının kodunu da kümülatif çalıştırdığı (bilinçli fallthrough) bir C deseni (25 satır, 5 test).
- **Ne oldu:** 2/5 geçti; level=4 için beklenen `15` (=8+4+2+1), alınan `8` (yalnızca kendi katkısı); level=3 için beklenen `7`, alınan `4`.
- **Neden oldu:** C'nin `switch` yapısı `break` konulmadığında bir sonraki case'e "düşer" (fallthrough) — kümülatif değer hesaplamak için klasik bir legacy deyimi. Rust'ın `match` yapısı varsayılan olarak düşmez; her kol bağımsızdır. LLM, her seviyeyi yalnızca kendi (kümülatif olmayan) katkısıyla eşleştirdi — C'deki kasıtlı düşme davranışını yeniden üretmedi.
- **Nasıl çözüldü:** Her `match` kolu, karşılık geldiği C case zincirinin toplam katkısını açıkça içerecek şekilde yeniden yazıldı. Round 2: 5/5 PASS.
- **Kısıtlı geri bildirim:** Düzeltilemedi — girdi yalnızca tek bir tamsayı (level) olduğundan kontrol-akışı hatasına dair hiçbir ipucu yok.

### 2.12 s48_cjson_number — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** Yaygın kullanılan bir C JSON kütüphanesinin (cJSON) sayı ayrıştırma/yazdırma fonksiyonları — round-trip garantili `%g` mantığı (389 satır, 5 test).
- **Ne oldu:** 1/5 geçti (en düşük geçme oranı); 02.txt'de beklenen `1e-10`, alınan `0.0000000001`; 03.txt'de beklenen `1`, alınan `1.0000000002` (floating-point round-trip hatası).
- **Neden oldu:** Yine Kategori D (çıktı biçimlendirme) — ama bu kez cJSON'un sayı yazdırma mantığı **bilimsel-gösterime-geçiş dalını** da içeriyor (C `1e-10` derken Rust'ın varsayılan biçimi `0.0000000001` üretti; C `1.79769313486232e+308` derken Rust 300'den fazla haneli düz ondalık bir dize üretti). s15/s27 için yazılan önceki düzelticilerin hiçbiri bu dalı gerektirmediğinden yetersiz kaldı.
- **Nasıl çözüldü:** cJSON'un round-trip garantili 15/17-basamak stratejisini ve `compare_double()` fonksiyonundaki bağıl tolerans mantığını birebir yeniden üreten, tamamen yeni ve daha genel bir biçimlendirici yazıldı. Round 2: 5/5 PASS.
- **Önemi:** Kategori D'nin **üçüncü, tamamen bağımsız bir gerçek kod tabanında** (cJSON, MIT lisanslı, yaygın kullanılan üçüncü parti kütüphane) da tekrarlanması, bu boşluğun veri setine özgü bir tuhaflık değil sistematik bir C↔Rust farkı olduğuna dair en güçlü kanıttır.
- **Kısıtlı geri bildirim:** Düzeltilemedi.

### 2.13 s49_negative_byte_count — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** s20 ile aynı kök nedenin (char işaretliliği) ikinci, bağımsız bir deseni — negatif bayt sayımı (27 satır, 5 test).
- **Ne oldu:** 3/5 geçti; 02.txt'de beklenen `12 12`, alınan `12 0`; 05.txt'de beklenen `38 8`, alınan `38 0`.
- **Neden oldu:** İlk çeviri `b as i32` (u8→i32 sıfır-genişletme) kullandığı için hiçbir zaman negatif üretmedi — s20'deki aynı kök neden, farklı bir kod deseninde.
- **Nasıl çözüldü:** Önce `as i8` ile işaretli dönüşüm, sonra `i32`'ye genişletme. Round 2: 5/5 PASS.
- **Kısıtlı geri bildirim:** Seviye B'de düzeltildi (çok baytlı Türkçe girdi ipucu).

### 2.14 s51_long_clamp — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** s38 ile aynı kök nedenin (platform tamsayı genişliği) ikinci, bağımsız bir deseni — `long` aralık sınırlama (33 satır, 5 test).
- **Ne oldu:** 3/5 geçti; 02.txt'de beklenen `2147483647`, alınan `4000000000`; 03.txt'de beklenen `-2147483648`, alınan `-4000000000`.
- **Neden oldu:** İlk çeviri `long`'u yaygın bir varsayımla 64-bit `i64` seçmişti; bu derleme ortamının gerçek 32-bit sınırını aşan toplamlarda hiç kırpma yapmıyordu.
- **Nasıl çözüldü:** `i64` yerine bu platformun gerçek `long` genişliğini yansıtan `i32`. Round 2: 5/5 PASS (Windows'ta).
- **⚠️ Aynı platform-özgü kırılganlık:** s38 gibi, bu düzeltme de Linux/LP64'te (64-bit `long`) geçersiz hale gelir — Docker deneyinde bu örnek de Windows↔Linux arasında PASS/FAIL yer değiştirmiştir.
- **Kısıtlı geri bildirim:** Düzeltilemedi.

### 2.15 s52_window_sum — Çalışma Zamanı Hatası (RE)

- **Ne yapılıyordu:** s40 ile aynı kök nedenin (usize taşması) ikinci, bağımsız bir deseni — kayan pencere toplamı (33 satır, 5 test).
- **Ne oldu:** 3/5 geçti; 02.txt/03.txt'de panik (`n < k` durumunda `n - k` usize altında taşıyor).
- **Neden oldu:** C referansı `start = n - k`'yi işaretli int ile hesaplayıp negatifse 0'a kırpıyordu; ilk çeviri aynı çıkarmayı `usize` ile birebir yaptığı için `n < k` olduğunda taşma paniği verdi.
- **Nasıl çözüldü:** `saturating_sub` ile çıkarma — usize altına hiç inmiyor. Round 2: 5/5 PASS.
- **Kısıtlı geri bildirim:** Seviye B'de düzeltildi (panik metni "attempt to subtract with overflow" tasma türünü belirtti).

### 2.16 s53_tax_bracket — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** s43 ile aynı kök nedenin (switch fallthrough) ikinci, bağımsız bir deseni — kademeli vergi dilimi hesabı (31 satır, 5 test).
- **Ne oldu:** 2/5 geçti; bracket=3 için beklenen `300`, alınan `200`; bracket=4 için beklenen `700`, alınan `400`.
- **Neden oldu:** C'nin switch'i break olmadan bir sonraki case'e kasıtlı olarak düşer (bracket N için N, N-1, ..., 1 katkıları birikir); ilk çeviri her dilimi yalnızca kendi katkısıyla eşleştirmişti.
- **Nasıl çözüldü:** Her match kolu, karşılık geldiği düşme zincirinin toplam katkısını içerecek şekilde yeniden yazıldı. Round 2: 5/5 PASS.
- **Kısıtlı geri bildirim:** Düzeltilemedi (girdi yalnızca bir tamsayı, ipucu yok).

### 2.17 s56_macro_table — Fonksiyonel Hata (FE)

- **Ne yapılıyordu:** X-Macro token-pasting deseni + klasik bir C makrosu "çoklu-değerlendirme" yan-etki tuzağı (`#define MAX(a,b) ((a)>(b)?(a):(b))`, 97 satır, 5 test).
- **Ne oldu:** 3/5 geçti; `x=20` girdisinde beklenen `m=21 x=22`, alınan `m=20 x=21` (yan etki bir kez yerine gerektiği gibi iki kez uygulanmadı).
- **Neden oldu:** C önişlemcisi makroları saf metinsel ikamedir (textual substitution): `MAX` makrosunda `a` parametresi iki kez geçer (karşılaştırma + seçilen dal); yan etkili bir argümanla (`MAX(x++, 10)`) çağrıldığında ve koşul doğru çıktığında, `x++` gerçekten iki kez çalışır. LLM'in doğal çevirisi bunu bir Rust fonksiyonuna (`fn max(a, b)`) dönüştürdü; Rust'ta fonksiyon argümanları her zaman tam olarak bir kez değerlendirilir.
- **Nasıl çözüldü:** C'nin metinsel ikame semantiğini kasıtlı olarak yeniden üreten bir Rust `macro_rules!` tanımı yazıldı. Round 2: 5/5 PASS.
- **İlginç ayrıntı:** Aynı örnekteki X-Macro/token-pasting deseni (enum ve isim tablosunun tek bir listeden türetilmesi) hiçbir soruna yol açmadı — LLM bunu doğrudan bir `enum`+`match`'e sorunsuz eşledi. Yani karmaşık makro kullanımının kendisi değil, özellikle *yan-etkili bir argümanın çoklu genişletilmesi* çeviri hatasına yol açtı.
- **Kısıtlı geri bildirim:** Düzeltilemedi (girdi bir komut adı + sayılardan oluşur, makro yan etkisine dair ipucu yok).

---

## 3. Başarılı Örneklerin Analizi (40/57)

Yalnızca başarısızlıklar değil, ilk seferde geçen 40 örneğin her biri de
analiz edilmiştir: ne test ediliyordu, neden sorunsuz geçti (hangi risk
gerçekleşmedi ya da hangi tasarım tercihi C'nin sözleşmesiyle zaten
örtüşüyordu) ve varsa dikkat çekici yönü.

### 3.1 Temel algoritmalar — düşük semantik risk (13 örnek)

Bu grup, C↔Rust arasında bilinen bir boşluğu kasıtlı hedeflemeyen, sayısal/
dizi tabanlı klasik algoritmalardır; PASS olmalarının nedeni basittir: girdi/
çıktı sözleşmeleri zaten tip-güvenli, taşma riski düşük veya test aralığının
dışında.

- **s01_sum** (dizi toplama, 16 satır): Temel doğrulama; taşma riski yok (küçük değerler).
- **s02_gcd** (Öklid EBOB, 19 satır): Negatif/modulo davranışı kasıtlı hedeflendi, ama C ve Rust'ın `%` operatörü işaretli sayılarda aynı işareti üretir — davranış zaten örtüşüyor.
- **s03_factorial** (faktöriyel, u64, 14 satır): Tamsayı sınırı test edildi; seçilen girdiler u64 sınırını aşmıyor.
- **s04_fibonacci** (iteratif Fibonacci, u64, 16 satır): Büyük değer testi; u64 yeterli genişlikte.
- **s05_count_primes** (deneme bölmesiyle asallık, 22 satır): Döngü/koşul çevirisi dümdüz, taşma/işaretlilik riski yok.
- **s07_bubble_sort** (kabarcık sıralama, 26 satır): Dizi/takas mantığı bire bir çevrilebilir, semantik boşluk yok.
- **s08_binary_search** (ikili arama, 22 satır): İndeks aritmetiği (`(low+high)/2`) küçük dizilerde taşma riski taşımaz.
- **s10_caesar_cipher** (Caesar şifreleme, 23 satır): Karakter aritmetiği yalnızca ASCII harfler üzerinde, bayt/karakter ayrımı devreye girmiyor.
- **s11_collatz** (Collatz adım sayımı, 17 satır): Döngü + büyük ara değer, u64 ile taşma yok.
- **s12_matrix_mult** (matris çarpımı, 36 satır): İç içe döngü, küçük matrisler — taşma/performans riski yok.
- **s16_rle_encode** (çalışma-uzunluğu kodlama, 22 satır): String tarama bayt bazlı yapılabiliyor, ASCII test girdileriyle bayt/karakter farkı tetiklenmedi.
- **s17_determinant** (Laplace açılımı, özyinelemeli, 35 satır): Orta boy özyineleme, taşma riski yok.
- **s18_expr_eval** (özyinelemeli inişli ifade ayrıştırıcı, 88 satır): Veri setindeki ilk "uzun" program; özyinelemeli ayrıştırma mantığı doğrudan çevrilebiliyor, semantik boşluk hedeflenmiyor.

### 3.2 Bilinçli tuzak, ama tetiklenmedi (4 örnek)

- **s21_matrix_transpose** (matris transpozu, 20 satır) ve **s23_histogram** (harf frekansı, 18 satır), **s24_roman** (Roma rakamı, açgözlü, 19 satır): İndeksleme/sayaç dizisi/string kurma — hepsi C ve Rust'ta aynı davranan (taşma riski taşımayan) işlemler.
- **s22_gray_code** (Gray kodu, `n ^ n>>1`, 10 satır): Bit işlemleri — XOR/kaydırma, C ve Rust'ta bit-bit özdeş davranır (taşma riski taşıyan çarpma/toplama yok); bu, s36_crc32'nin de PASS olma nedeniyle aynıdır (bkz. §3.4).

### 3.3 Uzun özgün programlar — dinamik bellek/pointer, ama güvenli çevrildi (4 örnek)

Bu programlar kasıtlı olarak kod uzunluğu-başarı ilişkisini test etmek için
eklendi (69-141 satır); hepsi C'de `malloc`/ham işaretçilerle kurulan dinamik
veri yapıları kullanır.

- **s25_linked_list_ops** (bağlı liste kurma/ters çevirme, 87 satır, 4 test): Model, `struct Node*` zincirini `Option<Box<Node>>` desenine sadakatle çevirdi — hiç `unsafe` kullanmadan, ilk seferde PASS.
- **s26_rpn_calculator** (RPN hesap makinesi, 69 satır, 5 test): String tokenizasyonu + yığın (stack) — Rust'ın `Vec` tabanlı yığını C'nin dizi-tabanlı yığınıyla davranışsal olarak özdeş.
- **s28_bst_traversal** (BST kurma/dolaşma, 80 satır, 4 test): Özyineleme + dinamik bellek; `Option<Box<T>>` deseni burada da sorunsuz.
- **s29_hashtable_cmds** (zincirlemeli hash tablosu + komutlar, 141 satır, 3 test): Aynı djb2-tarzı karma fonksiyonunu kullanır (bkz. s09/s14, Kategori A) ama test edilen anahtarlar kısa olduğundan (1-3 karakter) taşma eşiğine hiç ulaşılmadı — **taşma hatasının yalnızca yeterince büyük girdilerle tetiklenen, kısmen "gizli" bir risk olduğunun kanıtı.**

### 3.4 Rosetta Code — bağımsız gerçek dünya algoritmaları (7/7 PASS)

AS7'yi yanıtlamak için eklenen, tarafımızca yazılmamış 7 eğitim-amaçlı
algoritmanın **tamamı** ilk seferde geçti (18/18 test girdisi):

- **s30_luhn_check** (Luhn sağlama toplamı, 34 satır): Rakam bazlı aritmetik, taşma riski yok.
- **s31_soundex** (Soundex fonetik kodlama, 71 satır): Karakter dizisi işleme, yalnızca ASCII harfler üzerinde.
- **s32_levenshtein** (Levenshtein mesafesi, özyinelemeli, 50 satır): Üstel özyineleme — test girdileri (≤7 karakter) üstel patlamayı tetikleyecek kadar büyük değildi (zaman aşımı riski gerçekleşmedi).
- **s33_knapsack** (0/1 sırt çantası, DP, 71 satır): Dinamik programlama tablosu, taşma riski yok.
- **s34_hanoi** (Hanoi kuleleri, 27 satır): Klasik özyineleme, semantik boşluk hedeflenmiyor.
- **s35_lcs** (en uzun ortak alt dizi, DP, 67 satır): s33 ile aynı aile, taşma riski yok.
- **s36_crc32** (tablo tabanlı CRC-32, 61 satır): Bit/unsigned işlemler — yalnızca XOR/kaydırma kullanır, **taşma tetikleyen çarpma/toplama içermez** (bu yüzden Kategori A'daki djb2/FNV'nin aksine PASS oldu — aynı "unsigned aritmetik" yüzeyi, farklı operatör seti).

**Yorum:** Bu 7/7 sonucu, LLM'in genel olarak klasik algoritmaları çevirmede sistematik bir zayıflığı olmadığını destekler — başarısızlıklar veri setinin kasıtlı olarak hedeflediği belirli semantik boşluklara özgüdür, rastgele bir genel yetersizlik değildir.

### 3.5 Gerçek üretim (production) kodu — çoğu PASS (6/9 arası ilgili alt gruplar)

- **s37_bsd_getopt** (getopt(), 148 satır, 5 test): OpenBSD/FreeBSD'nin gerçek, ~39 yıllık komut satırı ayrıştırıcısı. Dışa açık değiştirilebilir global durum (`optarg`/`optind`/`optopt`/`opterr`) gerektirir — model bunu Rust'ta `static mut` + `unsafe fn` ile **sadakatle** yansıttı (bu veri setindeki 8 gerçek-`unsafe`-kullanımından biri) çünkü C kodunun kendisi bu sözleşmeyi yapısal olarak dayatıyor. İlk seferde PASS.
- **s39_bsd_heapsort** (heapsort(), 143 satır, 4 test): Generic `void*` yığın sıralaması — model, çağıran kodun (`main()`) yalnızca tamsayı sıralaması ihtiyacını tanıyarak genel `void*` imzasını taklit etmek yerine güvenli, deyimsel bir Rust dizi/dilim (slice) tabanlı sıralama yazdı — **hiç `unsafe` kullanmadan**. İlk seferde PASS.
- **s46_musl_qsort** (musl libc'nin smoothsort'u, 262 satır, 5 test): Bit-düzeyinde Leonardo-sayı kodlamasıyla çalışan, veri setindeki en karmaşık tek algoritma; genel-amaçlı `void*` imzasına gerçekten bağımlı olduğundan model burada `unsafe` kullandı (yapısal gereklilik). PASS.
- **s47_redis_sds** (Redis'in SDS dinamik string kütüphanesi, 522 satır, 5 test): Veri setindeki **en uzun program**; başlık bilgisini (uzunluk/kapasite/tür bayrağı) pointer'ın hemen öncesinde gizli tutan, C'ye özgü bir bellek düzeni kullanır. Model bu düzeni ham pointer aritmetiğiyle yeniden üretmeyi denemek yerine tamamen güvenli bir `String` tabanlı iç temsille yeniden yapılandırdı ve gözlemlenebilir API davranışını (uzunluk/kırpma/aralık/karşılaştırma) hiç `unsafe` kullanmadan birebir korudu. İlk seferde PASS — **ama bkz. §6, bu örnek Linux/Docker'da bir CRLF/stdio bulgusuyla farklı sonuç vermiştir (C referansının kendisi platforma bağlı davranıyor, Rust çevirisi değil).**

### 3.6 Hedeflenmemiş boşluklar grubundan PASS olanlar (4 örnek)

- **s41_float_bits** (float→bit örüntüsü, union/type punning, 21 satır): C'nin `union` ile IEEE-754 bit-düzeyinde yeniden yorumlaması, model tarafından Rust'ın güvenli `to_bits()`/`from_bits()` fonksiyonlarıyla birebir doğru eşlendi — `unsafe` transmute'a hiç gerek kalmadı.
- **s42_bitfields** (bit-alanı kırpma, 25 satır): C bit-alanlarının (bit-fields) atamada kırpma davranışı, model tarafından maskeleme/kaydırma operasyonlarıyla doğru yeniden üretildi.
- **s44_fib_memo_static** (fonksiyon-lokal static memoizasyon, 29 satır): Çağrılar arasında kalıcı, değiştirilebilir durum gerektiren bir C deseni (`static int cache[]`); model bunu Rust'ta `static mut` + `unsafe fn` ile **doğru ve gerekçeli** biçimde yansıttı (veri setindeki 8 gerçek-`unsafe`-kullanımından biri — s19'un aksine burada durum gerçekten fonksiyon-lokal ve tek-thread varsayımına uygun tasarlandığından model haklı olarak `unsafe`'i seçti).
- **s45_goto_cleanup** (goto ile kaynak temizleme, 43 satır): C'nin `goto cleanup;` deseni (RAII benzeri kaynak serbest bırakma), model tarafından Rust'ın doğal kapsam-tabanlı (scope-based) temizlik mantığına (veya erken `return` + düzenli serbest bırakma) doğru biçimde yeniden yapılandırıldı — kontrol akışı farklı ama gözlemlenebilir davranış özdeş.

### 3.7 İkinci-örnek kök neden testlerinden PASS olan (1/5)

- **s50_id_generator** (ardışık kimlik üretici, global durum, 30 satır): s19_global_counter ile **aynı kök nedeni** (güvensiz global durum) ikinci, bağımsız bir desenle sınamak için eklendi — model bu kez de aynı yapıyı (`static mut COUNTER: i32`) seçti, ama s19'un aksine erişimi doğru biçimde `unsafe { ... }` bloğuna sararak yazdı, bu yüzden derleme hatası hiç oluşmadı (kod: `translations_rust/s50_id_generator.rs`). **Bu, aynı kök nedenin ve aynı yapısal kalıbın (static mut) farklı çağrılarda farklı sonuç verebileceğinin, yani modelin `unsafe` blok ekleme davranışının deterministik/tutarlı olmadığının bir kanıtıdır** — diğer 4 ikinci-örnek (s49, s51, s52, s53) hepsi ilk örnekleriyle (s20, s38, s40, s43) aynı şekilde başarısız olurken, s50 farklı davranmıştır.

### 3.8 Çok dosyalı kod ve eşzamanlılık (3/3 PASS)

- **s54_stack_module** (yığın modülü, başlık+uygulama+kullanım, 125 satır, çok dosyalı): Model, fonksiyon imzalarını dosyalar arasında tutarlı tuttu ve `stack.h`/`stack.c` ayrımını Rust'ın `mod` sistemiyle doğru eşledi. İlk seferde PASS.
- **s55_config_parser** (paylaşılan struct + 2 derleme birimi, 139 satır, çok dosyalı): Paylaşılan bir struct tanımının iki ayrı derleme birimi tarafından kullanıldığı desen, sorunsuz çevrildi.
- **s57_shared_counter_threads** (N pthread + mutex korumalı paylaşılan sayaç, 71 satır, çok dosyalı): Model bunu ilk seferde `Arc<Mutex<i64>>` + `thread::spawn` + `join` desenine doğru çevirdi — ne derleme hatası ne veri yarışı oluştu. Bu, "Rust'ın tip sistemi paylaşılan-durum çevirisini imkânsız kılar mı" sorusuna, en azından bu tek örnekte, "LLM zaten doğru soyutlamayı biliyor" yanıtını verir.

### 3.9 Genel gözlem: 40 PASS'ın ortak noktası

Başarılı örneklerin hiçbirinde dinamik veri yapısı (bağlı liste, ağaç, hash
tablosu — s25, s28, s29) veya karmaşık bellek düzeni (s39, s46, s47) tek
başına başarısızlık nedeni olmamıştır; PASS/FAIL ayrımı kod karmaşıklığından
değil, **belirli bir semantik boşluğun o örnekte tetiklenip
tetiklenmediğinden** kaynaklanır (bkz. §4, §10). Bu, §4'teki dokuz kök neden
listesinin veri setinin geri kalanına genellenebilir bir tehdit olmadığını,
yalnızca belirli, tanımlanabilir kod kalıplarında ortaya çıktığını
doğrulamaktadır.

---

## 4. Dokuz Kök Neden — Özet Tablo

| # | Kategori | Örnekler | Tür | C sözleşmesi → LLM'in seçimi |
|---|---|---|---|---|
| A | Unsigned taşma | s09, s14 | RE | Kasıtlı mod-2ⁿ sarma → panik-eden varsayılan aritmetik |
| B | String modeli | s06, s13 | FE | Bayt-düzeyi işleme → `.chars()` (Unicode) |
| C | char işaretliliği | s20, s49 | FE | İşaretli char (127+ negatif) → `u8`/`i32` (hep pozitif) |
| D | Çıktı biçimlendirme | s15, s27, s48 | FE | `%g` anlamlı-basamak/üstel mantığı → varsayılan `{}` |
| E | Güvensiz global durum | s19 | CE | Değiştirilebilir global → `static mut` (unsafe gerekir) |
| F | Platform tamsayı genişliği | s38, s51 | FE | Platformun gerçek `long` genişliği → sabit `i64` |
| G | usize taşması | s40, s52 | RE | İşaretli int ile güvenli çıkarma → `usize` altında taşma |
| H | Switch fallthrough | s43, s53 | FE | Kümülatif düşme → bağımsız `match` kolları |
| I | Makro çoklu-değerlendirme | s56 | FE | Metinsel ikame (çoklu değerlendirme) → fn (tek değerlendirme) |

**Ortak örüntü:** Sekiz kategoride (E hariç) Rust kodu sözdizimsel olarak
geçerlidir ve "daha deyimsel/modern" görünen bir tercihi yansıtır — model,
kaynağın C'de gerçekten sahip olduğu bit genişliği/işaretlilik/sarma/kontrol
akışı sözleşmesini korumak yerine, hedef dilin idiyomatik varsayılanını
seçmiştir. Yalnızca kategori E'de aynı türden bir birebir aktarım Rust'ın tip
güvenliği tarafından derleme aşamasında yakalanmıştır.

---

## 5. Kısıtlı Geri Bildirim Deneyi (Tablo 5'in arkasındaki tam veri)

Aynı 17 başarısızlığa üç farklı ayrıntı seviyesinde geri bildirim verildi:

- **Seviye A (Oracle):** Tam derleyici hatası + panik metni + beklenen/alınan fark → **57/57 = %100**
- **Seviye B (Orta/CI-benzeri):** Derleyici/panik metni tam; FE için yalnızca girdi (fark yok) → **49/57 = %86.0**
- **Seviye C (Minimal):** Yalnızca gerçek başarısız test sayısı (CE hariç) → **41/57 = %71.9**

**Seviye B'de düzeltilen 9 örnek:** s19 (CE), s09/s14/s40/s52 (RE — panik metni
tasma türünü açıkça belirtti), s06/s13/s20/s49 (FE — girdideki çok baytlı
Türkçe karakterler ilgili semantik boşluğu tahmin etmeye yetti).

**Seviye B'de düzeltilemeyen 8 örnek:** s15/s27/s48 (%g biçimlendirme),
s38/s51 (platform tamsayı genişliği), s43/s53 (switch fallthrough), s56
(makro çoklu-değerlendirme) — hepsinde başarısız girdi (sade sayılar/komutlar)
ilgili kök nedene dair gözlemlenebilir bir ipucu taşımıyordu.

**Örüntü:** Düzeltilebilenlerin ortak özelliği, sınırlı bilginin (panik türü
veya girdinin görünür karakter özellikleri) yine de ilgili semantik
kategoriye işaret etmesi; düzeltilemeyenlerin ortak özelliği ise sınırlı
bilginin kategoriyi ayırt edici hiçbir iz taşımamasıdır.

---

## 6. Çoklu Platform Analizi (Windows LLP64 vs Linux/Docker LP64)

Windows: MSYS2/UCRT64 gcc 16.1.0 + rustc 1.97.1, `long`=32-bit.
Linux: Docker ubuntu:24.04, gcc 13.3.0 + rustc 1.97.1 (**birebir aynı rustc
sürümü** — gözlenen farkın Rust derleyicisinden değil C tarafındaki `long`
genişliğinden kaynaklandığını netleştirir).

| Koşul | Windows EA | Linux EA | Fark |
|---|---|---|---|
| Round 1 — debug | %70.91 (39/55) | %72.73 (40/55) | +1.82 puan |
| Round 1 — release | %74.55 (41/55) | %76.36 (42/55) | +1.81 puan |
| Round 2 — iyileştirilmiş | %100.00 (55/55) | **%94.55 (52/55)** | **−5.45 puan** |

**s38_bsd_strtol ve s51_long_clamp (Round 2, "düzeltilmiş"):** Windows için
yazılan `i32` düzeltmesi, Linux'ta `long` gerçekten 64-bit olduğundan artık
**yanlış** sonuç üretiyor — iki platform arasında PASS/FAIL durumu tam olarak
yer değiştirdi. Bu, "iyileştirilmiş" bir çevirinin evrensel değil platforma
özgü olabileceğinin doğrudan kanıtıdır.

**s47_redis_sds (beklenmedik yön):** Fark `long` genişliğinden değil,
**C referansının kendisinin** platforma bağlı davranmasından kaynaklanıyor:
test girdisi `05.txt` CRLF (`\r\n`) satır sonu içeriyor; Windows'un CRT'si
stdio'yu metin modunda açıp `\r\n`'i otomatik `\n`'e çeviriyor (klasik Windows
davranışı), Linux/glibc bunu yapmıyor. C'nin tek `getchar()` çağrısı
Windows'ta satır sonunu tam tüketirken, Linux'ta yalnızca `\r`'yi tüketip
kalan `\n`'i sonraki `fgets()`'e bırakıyor — bu da C referansının kendisinin
son komutu (`CAT END`) atlamasına yol açıyor. Rust çevirisi `BufRead::lines()`
kullandığından (her iki satır-sonu türünü de sorunsuz işler) bu sorunu hiç
yaşamıyor — yani burada **asıl "kırılan" taraf C referansıdır, Rust çevirisi
değil**. Test dosyası kasıtlı olarak düzeltilmedi çünkü bu gerçek ve
tekrarlanabilir bir bulgu.

---

## 7. Çoklu Model Analizi (Google Gemini, 57/57 TAM ölçüm)

Gemini (`gemini-flash-latest`) ile 57 örneğin **tamamı** gerçek API
çağrısıyla çevrildi (ücretsiz katmanın günlük kota sınırı nedeniyle ölçüm
birden fazla gün boyunca kademeli olarak tamamlanmıştır, 2026-07-22 ile
2026-07-26 arasında — kota her sıfırlandığında bir sonraki grup
çevrilerek; son 3 örnek — s55, s56, s57 — kota sıfırlandıktan sonraki bir
oturumda tamamlanmıştır); nihai EA = **%89.47 (51/57)**, Claude'un tam
veri setindeki EA'sı ise **%70.18 (40/57)**'dir. **Gemini ham sayı olarak
Claude'dan yüksek bir doğruluk göstermiştir** ve bu fark, iki model AYNI
57 program üzerinde ölçüldüğü için eşleştirilmiş (paired) bir tasarıma
uygun **McNemar testiyle** sınandığında istatistiksel olarak anlamlıdır
(yalnızca Claude'un başarısız olduğu 14 örnek vs. yalnızca Gemini'nin
başarısız olduğu 3 örnek; McNemar kesin iki-yönlü **p=0.013**,
`harness/stats_report.py` ile hesaplanmıştır) — ama aşağıdaki kategori
kırılımı ve vaka analizleri, bu genel anlamlılığın "Gemini genel olarak
daha iyi bir model" anlamına gelmediğini, aksine model×kod-kategorisi
etkileşiminin belirleyici olduğunu ve genel farkın yönünü kategoriye göre
değiştirdiğini göstermektedir.

### 7.1 Karşılaştırma Tablosu (57 örneğin tamamı)

| Örnek | Claude (Round 1) | Gemini | Durum |
|---|---|---|---|
| Kalan 37 örnek (s01-s05,s07-s08,s10-s12,s16-s18,s21-s25,s28-s37,s39,s41,s42,s44,s45,s50,s54,s55,s57) | pass | pass | Her iki model de geçti |
| s06_reverse_string | **functional_error** | pass | Yalnızca Claude başarısız |
| s09_djb2_hash | **runtime_error** | pass | Yalnızca Claude başarısız |
| s13_word_count | **functional_error** | pass | Yalnızca Claude başarısız |
| s14_fnv_hash | **runtime_error** | pass | Yalnızca Claude başarısız |
| s19_global_counter | **compilation_error** | pass | Yalnızca Claude başarısız |
| s20_char_sum | **functional_error** | pass | Yalnızca Claude başarısız |
| s38_bsd_strtol | **functional_error** | pass | Yalnızca Claude başarısız |
| s40_diff_sum | **runtime_error** | pass | Yalnızca Claude başarısız |
| s43_switch_fallthrough | **functional_error** | pass | Yalnızca Claude başarısız |
| s49_negative_byte_count | **functional_error** | pass | Yalnızca Claude başarısız (kategori C, 2. örnek) |
| s51_long_clamp | **functional_error** | pass | Yalnızca Claude başarısız (kategori F, 2. örnek) |
| s52_window_sum | **runtime_error** | pass | Yalnızca Claude başarısız (kategori G, 2. örnek) |
| s53_tax_bracket | **functional_error** | pass | Yalnızca Claude başarısız (kategori H, 2. örnek) |
| s56_macro_table | **functional_error** | pass | Yalnızca Claude başarısız (kategori I) |
| s15_float_avg | functional_error | **functional_error** | İkisi de başarısız (ortak kök neden) |
| s27_csv_stats | functional_error | **compilation_error** | İkisi de başarısız (farklı kök neden) |
| s26_rpn_calculator | pass | **compilation_error** | Yalnızca Gemini başarısız |
| s46_musl_qsort | pass | **compilation_error** | Yalnızca Gemini başarısız |
| s47_redis_sds | pass | **functional_error** | Yalnızca Gemini başarısız |
| s48_cjson_number | functional_error | **compilation_error** | İkisi de başarısız (farklı kök neden) |

**Çarpıcı bulgu #1 (veri setinin özgün/hedefli bölümü):** Claude'un
başarısız olduğu 14 örnekte (s06, s09, s13, s14, s19, s20, s38, s40, s43,
s49, s51, s52, s53, s56) — yani Kategori A, B, C, E, F, G, H, I'nin
**hepsinde**, hem ilk örneklerinde hem de hakem geri bildirimiyle eklenen
bağımsız ikinci örneklerinde (s49, s51, s52, s53) — **Gemini ilk seferde
geçmiştir.** Bu, ilk turda gözlenen Claude başarısızlıklarının tesadüfi
olmadığının, s49-s53 ile bağımsız olarak yeniden test edildiğinde de aynı
kök nedenin tekrarlandığının (§4, Tablo IV) doğrudan kanıtıdır: Claude bu
5 kategoriden 4'ünü ikinci örnekte de tekrar kaçırmıştır (yalnızca s50,
kategori E'nin 2. örneği, PASS olmuştur).

**Çarpıcı bulgu #2 (gerçek üretim kodu, musl/Redis/cJSON):** Tam tersi
yönde, en uzun/karmaşık üç gerçek üretim kodu örneğinde (s46, s47, s48 —
262-522 satır) **Claude 2/3 geçerken Gemini 0/3 geçmiştir.** Bu, "Gemini
genel olarak daha iyi" okumasını doğrudan çürütür: Gemini'nin üstünlüğü
yalnızca kısa, iyi tanımlanmış sızıntı kategorilerinde gözlenmekte, uzun ve
karmaşık gerçek üretim kodunda tersine dönmektedir (bkz. §7.2b).

Aşağıda her iki yönün vakaları ayrı ayrı incelenmiştir.

### 7.2 Gemini'nin geçtiği, Claude'un kaldığı 14 örnek — nasıl farklı davrandı?

- **s09_djb2_hash / s14_fnv_hash (Kategori A, unsigned taşma):** Gemini'nin
  çevirisi doğrudan `hash.wrapping_mul(33).wrapping_add(b as u32)` kullandı —
  yani Claude'un Round 2'de yapmak zorunda kaldığı düzeltmeyi Gemini **ilk
  geçişte, kendiliğinden** yaptı. Karma (hash) fonksiyonlarının taşma
  semantiğine dayandığını modelin doğrudan tanıdığı görülüyor.
- **s06_reverse_string (Kategori B, string modeli):** Gemini, dizgiyi hiç
  `String`/`.chars()` üzerinden değil, doğrudan `Vec<u8>` bayt tamponu
  üzerinden okuyup (`read_until(b'\n', ...)`) ters çevirdi (`buf.reverse()`).
  Yani sorunu Claude gibi "yanlış" bir soyutlama (Unicode karakter) seçip
  çözmedi — bayt modelini hiç terk etmediği için sorun baştan oluşmadı.
- **s19_global_counter (Kategori E, güvensiz global durum):** Gemini,
  `static mut i32` yerine `static CALL_COUNT: AtomicI32 = AtomicI32::new(0);`
  ve `fetch_add(1, Ordering::SeqCst)` kullandı — Rust standart kütüphanesinin
  **kilitsiz, tamamen güvenli** atomik türlerini seçerek `unsafe` gereksinimini
  en baştan ortadan kaldırdı. Bu, Claude'un s50_id_generator'da bulduğu
  çözümden (parametre olarak geçirme) farklı ama eşit derecede güvenli, daha
  da idiyomatik bir çözümdür.
- **s20_char_sum (Kategori C, char işaretliliği):** Gemini kodun bu kez
  baytları doğrudan işaretli yorumlayan bir yol izledi (Claude'un Round 2'de
  yapmak zorunda kaldığı `i8` dönüşümünü ilk geçişte yaptı).
- **s38_bsd_strtol (Kategori F, platform tamsayı genişliği):** Gemini, C'nin
  `long` tipini keyfi bir sabit genişlik (`i64`) yerine
  **`std::os::raw::c_long`** ile çevirdi — bu tür, derlendiği platformun
  gerçek C `long` genişliğine otomatik olarak uyum sağlar (Windows'ta 32-bit,
  Linux'ta 64-bit). Bu, Claude'un Round 2'de elle yaptığı platform-özgü
  düzeltmeden (§6'da Linux'ta geçersiz olduğu gösterilen `i32` sabiti) daha
  **taşınabilir/doğru** bir çözümdür — teorik olarak her iki platformda da
  doğru çalışması beklenir (bu çalışmada Gemini çevirisi Linux'ta ayrıca test
  edilmemiştir, bu ek bir doğrulama gerektirir).
- **s40_diff_sum (Kategori G, usize taşması):** Gemini döngü değişkenini
  `usize` yerine C'deki gibi **işaretli `i32`** olarak tuttu (`let mut i: i32 = 0`)
  — kaynağın işaretli-int sözleşmesini hiç terk etmediği için `n==0`
  durumunda taşma riski baştan oluşmadı.
- **s43_switch_fallthrough (Kategori H, switch fallthrough):** Gemini, her
  seviyenin kümülatif toplamını `match` kolunda **doğrudan açık aritmetik**
  olarak yazdı (`4 => 8+4+2+1`, `3 => 4+2+1` ...) — Claude'un ilk geçişte
  atladığı "düşme" davranışını, dolaylı da olsa doğru sonuca ulaştıracak
  şekilde yeniden ifade etti.

Hakem geri bildirimiyle eklenen ve her kategoriyi bağımsız bir ikinci
örnekle güçlendiren s49-s53, s56'da da **aynı örüntü aynen tekrarlandı** —
bu, §7.2'nin ilk 6 maddesinin tesadüf olmadığını, modelin gerçekten
tutarlı bir idiyomatik tercih kümesi kullandığını doğrular:

- **s49_negative_byte_count (Kategori C, char işaretliliği, 2. örnek):**
  Gemini yine baytı `byte as i8` ile doğrudan işaretli yorumladı — s20 ile
  birebir aynı çözüm deseni, farklı bir kod üzerinde.
- **s51_long_clamp (Kategori F, platform tamsayı genişliği, 2. örnek):**
  Gemini yine `std::os::raw::c_long` kullandı (`use std::os::raw::c_long;`
  ve tüm fonksiyon imzaları bu türle) — s38 ile birebir aynı çözüm.
- **s52_window_sum (Kategori G, usize taşması, 2. örnek):** Gemini pencere
  başlangıcını `let mut start = n - k;` biçiminde **işaretli `i32`**
  üzerinde hesaplayıp negatifse sıfırladı, yalnızca dizi indekslerken
  `arr[i as usize]` ile `usize`'a geçti — taşma riskini oluşturacak
  çıkarmayı hiç `usize` alanında yapmadı.
- **s53_tax_bracket (Kategori H, switch fallthrough, 2. örnek):** Gemini
  yine her dilimin kümülatif toplamını `match` kolunda açık aritmetikle
  yazdı (`4 => 800+400+200+100` ...) — s43 ile birebir aynı desen.
- **s56_macro_table (Kategori I, makro çoklu-değerlendirme):** Gemini,
  C'nin yan-etkili `INC_AND_GET` makrosunu **gerçek bir fonksiyona**
  (`fn post_inc(x: &mut i32) -> i32`) çevirdi — Tablo IV'te önerilen tam
  düzeltmeyi (fn ile tek-değerlendirme) kendiliğinden, ilk geçişte
  uyguladı. İlginç bir ayrıntı: zararsız `MAX(a,b)` makrosunu ise yine bir
  Rust makrosu (`macro_rules! max`) olarak bıraktı — yani model, "makroyu
  fonksiyona çevir" genel bir kuralı değil, yalnızca yan etkili çağrıyı
  ayırt edip düzeltmiştir.

**Ortak örüntü:** Bu 14 örnekte Gemini'nin "doğru" sonuca ulaşması, kaynak
kodun anlamını derinlemesine çözümlemesinden değil, **farklı ve tutarlı
bir idiyomatik varsayılan kümesi** kullanmasından kaynaklanıyor gibi
görünmektedir (bayt tamponu yerine `String`, `usize` yerine işaretli
`i32`, sabit `i64` yerine `c_long`, `static mut` yerine `AtomicI32`). Bu
tercihler tesadüfen (ya da modelin eğitim verisindeki farklı yaygın
kalıplar nedeniyle) C'nin sözleşmesiyle örtüşmüştür — ama bu, Gemini'nin
bu boşlukları "anladığı" anlamına gelmez; §7.2b ve §7.3'teki kendi
başarısızlıkları bunun sınırlarını gösterir.

### 7.2b Claude'un geçtiği, Gemini'nin kaldığı 2 örnek — tersine dönen desen

Yön tersine döndüğünde (Claude PASS, Gemini FAIL) örüntü de tersine
döner — bu kez Claude'un daha dayanıklı çıktığı örnekler, veri setinin en
uzun/karmaşık **gerçek üretim kodu** üyeleridir:

- **s46_musl_qsort (CE, Claude'da PASS):** Gemini'nin çevirisi
  `let mut ar = [*mut u8::null(); AR_LEN];` gibi geçersiz bir Rust söz
  dizimi içeriyor (`*mut u8::null()` şeklinde bir ifade Rust'ta yok —
  muhtemelen `std::ptr::null_mut::<u8>()` kastedilmiş ama yanlış
  yazılmış); smoothsort'un ham işaretçi aritmetiğini Rust'a aktarırken
  birden fazla söz dizimi hatası üretti. Claude'un çevirisi ise aynı
  algoritmayı dilim/slice tabanlı indekslemeyle, hiç `unsafe` gerekmeden
  ilk seferde doğru çevirmiştir (§8).
- **s47_redis_sds (FE, Claude'da PASS):** Gemini'nin çevirisi, `sdscatlen`
  benzeri bir ekleme işleminde büyüme mantığını yanlış uygulamış; test
  girdisi 05'te beklenen `LEN=5 STR=start / LEN=45 ... / LEN=295 ...`
  yerine `LEN=0 STR=` (boş) döndürmüştür — SDS'in gizli başlık/pointer
  düzenini (§Genel Çerçeve) yeniden üretirken bir yerde string tamponu
  sıfırlanmış veya yanlış referanslanmıştır. Claude'un çevirisi bu
  karmaşık bellek düzenini `String` tabanlı güvenli bir iç temsile
  başarıyla dönüştürmüştür.

Bu iki vaka, §7.2'de gözlenen "Gemini daha dayanıklı" örüntüsünün
kategoriye özgü olduğunu, uzun/karmaşık gerçek üretim kodunda tersine
döndüğünü doğrudan kanıtlar (bkz. §7.5 sentezi).

### 7.3 Gemini'nin kendine özgü başarısızlıkları (3 örnek)

Gemini'nin Claude'da hiç görülmeyen (yani Claude'un PASS olduğu) toplam 3
başarısızlığı vardır: s46 ve s47 (yukarıda §7.2b'de, gerçek üretim kodu
bağlamında incelendi) ve aşağıdaki s26 — kısa, özgün bir program olduğundan
kod uzunluğu/karmaşıklığıyla değil, tamamen farklı bir hata sınıfıyla
ilgilidir:

- **s26_rpn_calculator (CE, Claude'da PASS):** Gemini, yığın (stack)
  işlemlerini iki ayrı kapanış (closure) olarak yazdı — `push` ve `pop`,
  ikisi de aynı `sp`/`stack` değişkenlerini `FnMut` ile yakalıyor. Rust'ın
  ödünç denetleyicisi, iki kapanışın **aynı anda** aynı değişkenleri
  değiştirilebilir biçimde yakalamasına izin vermez (E0499) — kapanışlar
  hiç çağrılmasa bile, ikisinin birden var olması yeterli. Claude'un
  çevirisi bunun yerine düz fonksiyonlar veya doğrudan dizi indeksleme
  kullandığından bu tuzağa hiç düşmedi.

Not: s27_csv_stats, Gemini'de de başarısız olsa da (CE), Claude'da da
başarısızdır (FE) — bu yüzden Gemini'ye özgü değil, §7.4'te ele alınan
**ortak** başarısızlıklardan biridir.

### 7.4 Ortak başarısızlıklar: s15, s27 ve s48 (Kategori D'nin üç örneği)

- **s15_float_avg:** Her iki model de **aynı** kök nedenden (Kategori D,
  `%g` biçimlendirme) başarısız oldu — ikisi de Rust'ın varsayılan `{}`
  biçimini kullanıp C'nin 6-anlamlı-basamak/sondaki-sıfır-atma davranışını
  yeniden üretmedi.
- **s27_csv_stats:** Claude derlenen ama yanlış sonuç veren bir kod (FE)
  üretti; Gemini ise `%g` biçimlendirmesini taklit etmeye çalışırken
  geçersiz bir format string söz dizimi (`format!("{}e{:+=03}", m,
  exp_num)`) üretti — Rust'ın biçim dizesi söz diziminde `:+=03` diye bir
  belirteç yok, bu yüzden hiç derlenmedi (CE). İki model aynı köke
  (biçimlendirme) çarpar ama farklı yüzeysel hata türleriyle tökezler.
- **s48_cjson_number:** Kategori D'nin üçüncü bağımsız tekrarında (cJSON,
  gerçek üretim kodu) her iki model de yine başarısız oldu, ama **farklı
  hata türleriyle**: Claude derlenen ama yanlış sonuç veren bir kod (FE)
  üretirken, Gemini derleme aşamasında takılan geçersiz bir söz dizimi
  (CE, `let mut newbuffer: *mut u8;` etrafında sözdizimi hataları) üretti
  — s27'dekiyle aynı örüntü: iki model aynı köke (biçimlendirme) çarpar
  ama farklı yüzeysel hata türleriyle tökezler.

Bu üç örnek birlikte, Kategori D'nin (çıktı biçimlendirme) veri setindeki
**tek gerçekten model-bağımsız kör nokta** olduğunu güçlü biçimde
doğrular: 3 bağımsız kod tabanının (kendi yazdığımız iki örnek + cJSON)
3'ünde de, iki farklı modelin ikisi de aynı temel hataya düşmüştür.

### 7.5 Genel yorum (57/57 tam veriyle)

Tam veri seti üzerinde dört farklı model-etkileşim türü gözlenmiştir:
(a) **ortak** C↔Rust semantik boşluğu — Kategori D, üç bağımsız kod
tabanında iki modelin de aynı kör noktaya düşmesi (§7.4); (b) **model-özgü,
kaynak-koddan bağımsız** hata sınıfları — Gemini'nin E0499 ödünç hatası ve
geçersiz format string'i (§7.3), yalnızca modelin kendi kod-üretim
tarzından kaynaklanan, C'nin belirli bir davranışıyla ilgisi olmayan
hatalar; (c) **aynı boşluğu farklı idiyomatik tercihlerle tesadüfen
atlatma** — Gemini'nin Kategori A, B, C, E, F, G, H, I'de (14 örnek,
§7.2) sistematik olarak farklı varsayılanlar kullanması; ve (d) **model×
kod-karmaşıklığı etkileşimi** — aynı iki modelin, kısa hedeflenmiş
kategorilerde (Gemini üstün, §7.2) ile uzun/karmaşık gerçek üretim
kodunda (Claude üstün, §7.2b) taban tabana zıt biçimde sıralanması.

Dördüncü gözlem özellikle önemlidir çünkü EA'nın toplam sayısal
karşılaştırmasını (Gemini %89.47 > Claude %70.18) yanıltıcı kılar: bu
fark, esasen veri setinin çoğunluğunu oluşturan kısa/orta uzunluktaki
hedefli örneklerden (Kategori A-I) kaynaklanır; veri setinin en uzun ve
en karmaşık 3 örneğinde (musl/Redis/cJSON) sıralama tam tersine
dönmektedir. Bir modelin belirli bir kategori için "güvenli" görünmesi, o
kategoriyi anladığı anlamına gelmeyebilir — yalnızca o modelin varsayılan
kod-üretim tarzının o kod deseninde tesadüfen doğru sonuç vermesi
olabilir. Bu, çoklu-model karşılaştırmalarının toplam EA rakamlarını
yüzeysel biçimde sıralamak yerine, hem kategori bazında kırılımı hem de
altta yatan kod-üretim tercihlerini incelemesi gerektiğini göstermektedir.

---

## 8. Bellek Güvenliği ve `unsafe` Kullanımı

114 çeviriden (57 örnek × Round 1/Round 2; çok dosyalı üç örnek gerçek
dosya sayısını 122'ye çıkarır) `unsafe`/ham işaretçi kullanımı için
taranmıştır: yalnızca **8'i** (4 örnek × 2 tur: s37_bsd_getopt,
s44_fib_memo_static, s46_musl_qsort, s50_id_generator) gerçek `unsafe`
kullanıldı — dördü de C kodunun kendisinin yapısal olarak dayattığı bir
gerekliliği yansıtır:
- **s37 (getopt):** `optarg`/`optind`/`optopt`/`opterr` — çağıran kod
  tarafından okunması beklenen, dışa açık değiştirilebilir global durum.
- **s44 (fonksiyon-lokal static memoizasyon):** Çağrılar arasında kalıcı,
  değiştirilebilir durum gerektirir.
- **s46 (musl smoothsort):** Genel-amaçlı `void*` imzasına dayanan bit-düzeyi
  byte-pointer aritmetiği.
- **s50 (id_generator, Kategori E'nin 2. örneği):** s19 ile aynı
  `static mut` yapısı, ama bu kez erişim doğru biçimde `unsafe` bloğuna
  sarılmış (§3.7) — modelin `unsafe` ekleme davranışının aynı kalıpta bile
  tutarlı olmadığının kanıtı.

**Daha çarpıcı bulgu:** Ham `void*` işaretçi aritmetiğine yapısal olarak
bağımlı `heapsort()` (s39_bsd_heapsort) ve veri setindeki en karmaşık
bellek-düzenine sahip Redis'in SDS string kütüphanesi (s47_redis_sds) —
ikisi de gerçek üretim kodu — model tarafından **hiç `unsafe` kullanılmadan**,
tamamen güvenli/deyimsel Rust'a (sırasıyla dilim/slice tabanlı sıralama,
`String` tabanlı iç temsil) çevrildi ve ilk seferde PASS oldu. Bağlı liste
(s25), BST (s28), hash tablosu (s29) gibi C'de `malloc`/ham işaretçilerle
kurulan veri yapıları da tutarlı biçimde `Option<Box<T>>` deseniyle güvenli
Rust'a çevrildi.

**Sonuç:** `unsafe` kullanımı kodun karmaşıklığından değil, C kodunun dışa
açık davranış sözleşmesinin doğasından (harici mutable durum, fonksiyon ömrü
boyunca kalıcı durum, generic `void*` imzası) etkileniyor.

---

## 9. İstatistiksel Bulgular

- **Kod uzunluğu ile başarı ilişkisi:** Mann-Whitney U testi, PASS (n=40) ve
  FAIL (n=17) gruplarının LoC dağılımları arasında istatistiksel olarak
  anlamlı bir fark bulamadı (U=287.0, p=0.359). Rank-biserial etki büyüklüğü
  r=0.156 (küçük etki).
- **Duyarlılık analizi (post-hoc/gözlemlenen güç yerine önerilen):**
  Bootstrap-tabanlı gerçekleşen güç (achieved power, α=0.05, 5000 tekrar)
  yalnızca **%15.0**'tır, ama bu ölçüt p-değerinin tekdüze bir dönüşümüdür
  ve bağımsız bilgi taşımaz (Hoenig & Heisey 2001, "The Abuse of Power").
  Bunun yerine hesaplanan duyarlılık analizi: n(FAIL)=17, n(PASS)=40,
  α=0.05 ile %80 güçte saptanabilecek en küçük etki büyüklüğü, anlamlılığı
  ölçmek için kullanılan AYNI Mann-Whitney U istatistiğinden ampirik
  rank-biserial formülüyle (r=1-2U/(n1·n2), normal/AUC yaklaşık dönüşümü
  değil) hesaplandığında rank-biserial |r|≈0.46'dır (U'nun bağ düzeltmeli
  standart sapması üzerinden).
  Gözlemlenen
  r=0.156 bu eşiğin belirgin altındadır — veri seti bu büyüklükte küçük-orta
  etkileri saptayacak güce sahip değildir; "anlamlı fark yok" sonucu kesin
  bir ilişkisizlik kanıtı değil, düşük güçle tutarlı bir gözlem olarak
  okunmalıdır (`harness/stats_report.py` ile hesaplanmıştır).
- **p-değerinin istikrarsızlığı:** Veri seti n=36→39→45→48→53→57'ye
  büyüdükçe p-değeri 0.076 → 0.187 → 0.169 → 0.273 → 0.337 → 0.359 olarak
  dalgalandı — küçük örneklemlerde p-değerinin ne denli oynak olabileceğinin
  doğrudan, kendi verimizden bir kanıtı.
- **İşaretçi kullanımı ile başarı ilişkisi:** Fisher'in kesin testi, işaretçi
  kullanımı ile PASS/FAIL arasında anlamlı bir ilişki bulamadı (tablo=
  [[23,7],[17,10]], olasılık oranı=1.93, p=0.385, %95 GA=[0.61, 6.11] —
  aralığın 1.0'ı içermesi anlamsızlığı doğrular).
- **Betimsel örüntü (n=57 üzerinden, `harness/stats_report.py` ile
  yeniden hesaplandı):** Ortalama LoC PASS=67.0/FAIL=59.8, medyan LoC
  PASS=34.5/FAIL=27.0. FAIL grubunun string-fonksiyonu kullanım oranı
  (%47.1) PASS grubundan (%37.5) yüksek; FAIL grubunda hiç
  `malloc`/`calloc` kullanılmıyor (%0.0 vs PASS'te %22.5) — dinamik bellek
  yönetimi arıza riskini artırmıyor, aksine düşürüyor gibi görünüyor (küçük
  örneklemde ön bulgu). İşaretçi kullanımı PASS'te %57.5, FAIL'de %41.2.
- **Model karşılaştırması (McNemar):** Claude ve Gemini AYNI 57 program
  üzerinde ölçüldüğünden, genel EA farkının anlamlılığı eşleştirilmiş
  (paired) bir tasarıma uygun McNemar testiyle sınanmıştır: yalnızca
  Claude'un başarısız olduğu 14 örnek vs. yalnızca Gemini'nin başarısız
  olduğu 3 örnek üzerinden McNemar kesin iki-yönlü **p=0.013** — genel fark
  istatistiksel olarak anlamlıdır, ama §7'deki kategori kırılımı bu farkın
  yönünün kategoriye göre değiştiğini göstermektedir.

---

## 10. Ekstra Analiz: Zaman İçinde Bulguların Evrimi

Veri seti kademeli olarak genişletildikçe (24→29→36→39→45→48→53→57 örnek),
kök neden kategorilerinin **4'ü (F, G, H, I) başlangıçta hiç öngörülmemiş,
sonradan ortaya çıkan bulgulardır**:

| Aşama | Eklenen | Ortaya çıkan yeni bulgu |
|---|---|---|
| s01-s24 (ilk çekirdek) | 24 özgün program | Kategoriler A-E (taşma, string modeli, işaretlilik, biçimlendirme, global durum) |
| s25-s29 | 5 uzun özgün program | (Kod uzunluğu testi — yeni kök neden yok) |
| s30-s36 | 7 Rosetta Code | (Bağımsız doğrulama — yeni kök neden yok, 7/7 PASS) |
| s37-s39 | 3 BSD libc | **Kategori F** (platform tamsayı genişliği) — s38'de ortaya çıktı |
| s40-s45 | 6 yeni özgün | **Kategori G** (usize taşması, s40) ve **H** (switch fallthrough, s43) |
| s46-s48 | 3 musl/Redis/cJSON | Kategori D'nin 3. bağımsız doğrulaması (s48) |
| s49-s53 | 5 ikinci-örnek | Kategorilerin (C,E,F,G,H) 2. örnekle doğrulanması |
| s54-s57 | çok dosya + makro + eşzamanlılık | **Kategori I** (makro çoklu-değerlendirme, s56) |

Bu, veri setinin başlangıçta "bilinen" boşlukları hedefleyerek tasarlanmış
olsa da, **gerçek dünya kodu eklendikçe önceden tahmin edilemeyen yeni hata
modlarının ortaya çıkmaya devam ettiğini** gösterir — 9 kategoriden 4'ü
tasarım aşamasında yoktu.

---

## 11. Genel Çıkarımlar

1. **Sessiz hata oranı çok yüksek:** 17 başarısızlığın 16'sı (%94.1)
   derleyiciden hiçbir uyarı almadan geçti — yalnızca derleme başarısına
   dayanan bir kabul kriteri bu hataların hiçbirini yakalayamaz.
2. **Tek bir örüntü, dokuz görünümü açıklıyor:** Sekiz kategoride model,
   C kaynağının gerçek bit-düzeyi/kontrol-akışı sözleşmesini korumak yerine
   Rust'ın "daha temiz" görünen varsayılanını seçiyor.
3. **Düzeltmeler kırılgan olabilir:** Aynı düzeltme (i32 kullanımı) bir
   platformda (Windows) doğru, başka bir platformda (Linux) yanlış olabilir.
   Bir düzeltmenin dar test girdileriyle doğrulanması (s15→s27 vakası) bile
   yetersiz kalabilir.
4. **Geri bildirim zenginliği belirleyici:** Aynı 17 hatanın düzeltilme oranı,
   verilen bilginin ayrıntısına göre %100'den %71.9'a kadar değişebiliyor —
   gerçek bir CI/CD ortamındaki geri bildirim kalitesi, iyileştirme
   döngüsünün pratikteki etkinliğini doğrudan belirler.
5. **`unsafe` kullanımı isteğe bağlı değil, sözleşmeye bağlı:** Model, C
   kodunun kendisi gerektirmediği sürece hiç `unsafe` kullanmıyor — karmaşık
   bellek düzenlerini bile güvenli soyutlamalarla yeniden yapılandırabiliyor.
6. **Bulgular tek modele özgü olabilir; "en iyi model" tek boyutlu bir soru
   değildir:** Gemini ile 57/57 tam karşılaştırma, bazı hataların modeller
   arası ortak (Kategori D, sistematik C↔Rust boşluğu — §7.4), bazılarının
   modele özgü (Gemini'nin E0499 ödünç hatası — §7.3) olduğunu gösteriyor.
   Daha da önemlisi: Gemini, Claude'un ikinci bağımsız örnekte de tekrar
   kaçırdığı 5 kategoride (§7.2) tekdüze biçimde daha dayanıklı, ama en
   uzun/karmaşık gerçek üretim kodu üçlüsünde (musl/Redis/cJSON, §7.2b)
   tekdüze biçimde daha kırılgandır — "model A modelden B'den daha iyidir"
   sorusunun yanıtı, kod tabanının niteliğine göre değişir.

---

*Kaynaklar: `results/results_round1.json`, `results/results_round2*.json`,
`results/results_gemini.json`, `results/platform_comparison.md`,
`results/model_comparison.md`, `results/stats_report.md`,
`translations_rust_refined/*.rs` (IYILESTIRME yorumları),
`translations_rust__gemini/*.rs`, `MODIFICATIONS.md`.*

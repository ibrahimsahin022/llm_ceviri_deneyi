# Detaylı Sorun Analizi: C→Rust Çevirisinde Yaşanan Tüm Hatalar

Bu belge, makaleden **bağımsız**, yalnızca deneyde karşılaşılan sorunları
derinlemesine incelemek için hazırlanmıştır. 57 örneğin tamamı, Round 1'de
başarısız olan 17 örneğin her biri (ne yapılmaya çalışıldı → ne oldu → neden
oldu → nasıl çözüldü), sonrasında yapılan tüm ek deneyler (release/debug,
çoklu platform, çoklu model, kısıtlı geri bildirim, bellek güvenliği) ve
bunlardan çıkan genel örüntüler ele alınmaktadır. Tüm sayılar gerçek harness
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

Bu 17 başarısızlığın **her biri** aşağıda tek tek ele alınmıştır.

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
- **Bağımsız model doğrulaması:** Gemini de aynı örnekte, aynı kök nedenden başarısız oldu (44/57 kısmi ölçümde tek ortak başarısızlık) — bu boşluğun modele özgü olmadığının kanıtı.

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

## 3. Dokuz Kök Neden — Özet Tablo

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

## 4. Kısıtlı Geri Bildirim Deneyi (Tablo 5'in arkasındaki tam veri)

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

## 5. Çoklu Platform Analizi (Windows LLP64 vs Linux/Docker LP64)

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

## 6. Çoklu Model Analizi (Google Gemini, 44/57 kısmi ölçüm)

Gemini (`gemini-flash-latest`) ile 57 örnekten 44'ü gerçek API çağrısıyla
çevrildi (API kotası kalan 13'ü engelledi); EA = **%93.18 (41/44)**.

**3 başarısızlık:**
1. **s15_float_avg** — Claude ile **birebir aynı kök neden** (Kategori D, `%g`
   biçimlendirme). Bağımsız bir modelin de aynı boşluğa düşmesi, bunun
   modele özgü olmayabileceğinin güçlü bir işaretidir.
2. **s26_rpn_calculator** — Claude'da hiç görülmeyen, **Gemini'ye özgü** bir
   derleme hatası: aynı `sp` değişkenini iki farklı `FnMut` kapanışının
   (`push`/`pop`) eşzamanlı ödünç almaya çalışması (Rust ödünç denetleyicisi
   hatası, E0499).
3. **s27_csv_stats** — Yine Gemini'ye özgü: geçersiz bir biçim dizesi söz
   dizimi (`format!("{}e{:+=03}", ...)`) üretilmesi — Rust'ta böyle bir
   biçimlendirme belirteci yok, derleme hatası.

**Yorum:** Farklı modeller hem **ortak** (aynı C↔Rust semantik boşluğuna
düşme) hem de **birbirinden bağımsız** (modele özgü hallüsinasyon/API
kullanım hatası) hata sınıfları üretebilir.

---

## 7. Bellek Güvenliği ve `unsafe` Kullanımı

114 çeviri dosyasının (57 örnek × Round 1/Round 2) tamamı `unsafe`/ham
işaretçi kullanımı için tarandı: yalnızca **6 dosyada** (s37_bsd_getopt,
s44_fib_memo_static, s46_musl_qsort'un her iki turu) gerçek `unsafe`
kullanıldı — üçü de C kodunun kendisinin yapısal olarak dayattığı bir
gerekliliği yansıtır:
- **s37 (getopt):** `optarg`/`optind`/`optopt`/`opterr` — çağıran kod
  tarafından okunması beklenen, dışa açık değiştirilebilir global durum.
- **s44 (fonksiyon-lokal static memoizasyon):** Çağrılar arasında kalıcı,
  değiştirilebilir durum gerektirir.
- **s46 (musl smoothsort):** Genel-amaçlı `void*` imzasına dayanan bit-düzeyi
  byte-pointer aritmetiği.

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

## 8. İstatistiksel Bulgular

- **Kod uzunluğu ile başarı ilişkisi:** Mann-Whitney U testi, PASS (n=40) ve
  FAIL (n=17) gruplarının LoC dağılımları arasında istatistiksel olarak
  anlamlı bir fark bulamadı (U=287.0, p=0.359). Rank-biserial etki büyüklüğü
  r=0.156 (küçük etki). Bootstrap-tabanlı gerçekleşen güç (achieved power,
  α=0.05, 5000 tekrar): yalnızca **%15.0** — düşük güç, "anlamlı fark yok"
  sonucunun bir Tip II hatası olabileceği anlamına gelir, kesin bir "ilişki
  yoktur" iddiası değildir.
- **p-değerinin istikrarsızlığı:** Veri seti n=36→39→45→48→53→57'ye
  büyüdükçe p-değeri 0.076 → 0.187 → 0.169 → 0.273 → 0.337 → 0.359 olarak
  dalgalandı — küçük örneklemlerde p-değerinin ne denli oynak olabileceğinin
  doğrudan, kendi verimizden bir kanıtı.
- **İşaretçi kullanımı ile başarı ilişkisi:** Fisher'in kesin testi, işaretçi
  kullanımı ile PASS/FAIL arasında anlamlı bir ilişki bulamadı (olasılık
  oranı=1.93, p=0.385, %95 GA=[0.61, 6.11] — aralığın 1.0'ı içermesi
  anlamsızlığı doğrular).
- **Betimsel örüntü (Tablo 4):** FAIL grubunun string-fonksiyonu kullanım
  oranı (%58.3) PASS grubundan (%36.1) yüksek; FAIL grubunda hiç
  `malloc`/`calloc` kullanılmıyor (dinamik bellek yönetimi arıza riskini
  artırmıyor, aksine düşürüyor gibi görünüyor — küçük örneklemde ön bulgu).

---

## 9. Ekstra Analiz: Zaman İçinde Bulguların Evrimi

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

## 10. Genel Çıkarımlar

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
6. **Bulgular tek modele özgü olabilir:** Gemini ile kısmi karşılaştırma, bazı
   hataların modeller arası ortak (sistematik C↔Rust boşluğu), bazılarının
   ise modele özgü (hallüsinasyon) olduğunu gösteriyor.

---

*Kaynaklar: `results/results_round1.json`, `results/results_round2*.json`,
`results/platform_comparison.md`, `results/model_comparison.md`,
`results/stats_report.md`, `translations_rust_refined/*.rs` (IYILESTIRME
yorumları), `MODIFICATIONS.md`.*

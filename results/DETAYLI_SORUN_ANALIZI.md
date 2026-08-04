# Detaylı Sorun Analizi: C→Rust Çevirisinde Yaşanan Tüm Hatalar

Bu belge, makaleden **bağımsız**, yalnızca deneyde karşılaşılan sorunları
derinlemesine incelemek için hazırlanmıştır. **130 örneğin tamamı** ele
alınmıştır: Round 1'de başarısız olan 38 örneğin her biri (ne yapılmaya
çalışıldı → ne oldu → neden oldu → nasıl çözüldü) ve ilk seferde geçen 92
örneğin grup bazında analizi (ne test ediliyordu → neden sorunsuz geçti →
dikkat çekici yönü varsa), sonrasında yapılan tüm ek deneyler (release/debug,
çoklu platform, çoklu model, kısıtlı geri bildirim, bellek güvenliği) ve
bunlardan çıkan genel örüntüler ele alınmaktadır. Tüm sayılar gerçek harness
çalıştırmalarından (`results/*.json`) alınmıştır; hiçbir rakam uydurulmamıştır.

---

## 1. Genel Çerçeve

**Veri seti:** 130 C programı, 521 test girdisi, 10-522 satır arası (toplam
7.618 satır C kodu). Programların örnek bazında tam kataloğu
(algoritma/LoC/test/rol) için bkz. `VERISETI_VE_ALGORITMALAR.md`.
**Çeviri:** Claude Sonnet 5, zero-shot (tek geçiş, hiçbir insan düzeltmesi
veya derleme/test geri bildirimi olmadan).
**Değerlendirme:** Diferansiyel test — C referansı ile Rust çevirisi aynı
girdilerde çalıştırılır, çıktılar karşılaştırılır.

**Round 1 (ham çeviri, debug modu) sonucu:** 92/130 = %70.77 EA (Execution
Accuracy). 38 örnek başarısız: 1 CE (derleme hatası), 9 RE (çalışma zamanı
hatası/panik), 28 FE (fonksiyonel hata — derlenip çalışıyor ama çıktı yanlış).
Test-girdisi bazında: çalıştırılabilen 518 girdinin 435'i geçti (%83.98);
başarısız 83 girdinin 23'ü RE, 60'ı FE kaynaklıdır.

**Veri setinin üç katmanlı genişletilmesi (s58-s130, 73 yeni örnek)** bu
belgenin bu sürümünde ilk kez ele alınmaktadır ve sonuçları katmana göre
çarpıcı biçimde farklıdır:

| Katman | Örnek | Amaç | Round 1 sonucu |
|---|---|---|---|
| s58-s84 (27) | Dokuz kök-neden kategorisinin (A-I) her birine 3'er bağımsız örnek | Kategorileri anekdot olmaktan çıkarmak | **7/27 PASS (%25.9)** |
| s85-s109 (25) | Gerçek açık kaynak üretim kodu (SQLite, zlib, curl, Redis, OpenSSL, libsodium, BSD/musl libc, nginx, cJSON, Apache) | Dış geçerlilik | **24/25 PASS (%96.0)** |
| s110-s130 (21) | Çeşitlilik, çok dosyalı yapı, pthread eşzamanlılığı | Kapsam genişliği | **21/21 PASS (%100)** |

Bu tablo, belgenin geri kalanının ana tezini tek başına özetler:
**başarısızlık kodun gerçekliği/uzunluğu/karmaşıklığıyla değil, belirli bir
semantik boşluğun o örnekte tetiklenip tetiklenmediğiyle ilişkilidir.**

Bu 38 başarısızlığın **her biri** §2'de (ilk 17'si tek tek §2.1-§2.17'de,
sonradan eklenen 21'i kategori bazında §2.18'de), 92 PASS örneğin analizi ise
§3'te ele alınmıştır.

---

## 2. Başarısız Örneklerin Tam Vaka Analizi (38/38)

Her vaka dört soruya yanıt verir: **Ne yapılıyordu? → Ne oldu? → Neden oldu? → Nasıl çözüldü?**

§2.1-§2.17, veri setinin n=57 aşamasındaki 17 başarısızlığını tek tek ele
alır. §2.18, n=130'a genişletmede ortaya çıkan 21 yeni başarısızlığı kök-neden
kategorisine göre gruplayarak inceler (her biri için gerçek beklenen/alınan
çıktı farkı verilmiştir).

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

### 2.18 Veri Seti Genişletmesinde Ortaya Çıkan 21 Yeni Başarısızlık (s58-s130)

Veri seti 57'den 130'a çıkarıldığında 21 yeni başarısızlık gözlendi. **Bu 21
başarısızlığın hiçbiri yeni bir kök neden ortaya çıkarmadı** — hepsi mevcut
dokuz kategoriden birine düştü. Bu, taksonominin bu kod sınıfı için doyuma
ulaştığına dair en güçlü kanıttır. Aşağıda kategori bazında, her örnek için
gerçek beklenen/alınan farkıyla birlikte ele alınmaktadır.

#### 2.18.A Kategori A (unsigned taşma) — 2 yeni başarısızlık, 1 PASS

- **s58_rolling_hash_poly31** (RE, 18 satır, 2/4 test): Polinom-31 tabanlı
  yuvarlanan karma. Beklenen `3685539155`, alınan: **çıktı yok** (panik,
  `attempt to multiply with overflow`). s09/s14 ile birebir aynı kök neden:
  model `hash * 31 + b` ifadesini Rust'ın varsayılan (debug'da panikleyen)
  operatörüyle yazdı.
- **s59_sdbm_hash** (RE, 19 satır, 1/4 test): sdbm karma fonksiyonu
  (`h = c + (h<<6) + (h<<16) - h`). Beklenen `684824882`, alınan: panik.
  Buradaki ilginç ayrıntı, taşmanın çarpmadan değil **kaydırma + çıkarma**
  zincirinden kaynaklanmasıdır — yani kategori A yalnızca `*` operatörüne
  özgü değildir.
- **s60_elf_hash PASS oldu ve nedeni öğreticidir:** Klasik ELF/PJW karması
  her yinelemede üst dört biti `g = h & 0xF0000000u; ... h &= ~g;` ile
  temizler; bu yüzden `h` hiçbir zaman 2²⁸'i aşmaz ve bir sonraki
  `(h << 4) + bayt` işlemi 32-bit sınırına hiç ulaşmaz. Örnek "taşmaya açık"
  bir desen olarak tasarlanmış olsa da taşma pratikte hiç tetiklenmez.
  Bu, kategori A'nın "unsigned aritmetik" yüzeyine değil, **sonucu
  sınırlandırılmamış** aritmetiğe bağlı olduğunu gösterir.
- **Release modu notu:** s58 ve s59 release'de PASS olur (taşma sarar, C ile
  aynı sonuç) — s09/s14 ile aynı davranış.

#### 2.18.B Kategori B (string modeli: bayt vs. karakter) — 1 yeni başarısızlık, 2 PASS

- **s63_palindrome_bytes** (FE, 27 satır, 3/4 test): Bir metnin **bayt
  dizisi** olarak palindrom olup olmadığı. 04.txt'de (çok baytlı Türkçe
  girdi) beklenen `HAYIR`, alınan `EVET`. Neden: model `.chars()` üzerinden
  karakter düzeyinde karşılaştırdı; girdi karakter düzeyinde palindromken
  bayt düzeyinde değildi (UTF-8'de çok baytlı karakterlerin bayt sırası
  tersine çevrildiğinde eşleşmez).
- **s61_utf8_byte_vs_char_count ve s62_strtok_tokenizer PASS oldu:** s61'de
  C referansının kendisi hem bayt hem karakter sayısını **ayrı ayrı**
  yazdırdığı için model ayrımı yapmak zorunda kaldı ve doğru yaptı; s62'de
  ise `strtok` sınırlayıcıları saf ASCII boşluk olduğundan bayt ve karakter
  modelleri özdeş sonuç üretti. **Örüntü:** Kategori B yalnızca bayt/karakter
  ayrımının gözlemlenebilir çıktıyı gerçekten değiştirdiği durumlarda
  tetikleniyor.

#### 2.18.C Kategori C (char işaretliliği) — 3 yeni başarısızlık, 0 PASS

Kategorinin **üç yeni örneğinin üçü de** başarısız oldu; C artık beş bağımsız
örnekle temsil edilen, en tutarlı tetiklenen kategorilerden biridir.

- **s64_char_minmax_signed** (FE, 29 satır, 2/4 test): Bir metindeki en küçük
  ve en büyük bayt değeri. Beklenen `-80 117`, alınan `97 196`. Model
  baytları `u8` olarak karşılaştırdı; C'de `char` işaretli olduğundan
  127-üstü baytlar negatiftir ve hem minimum hem maksimum değişir.
- **s65_ctype_isalpha_highbyte** (FE, 24 satır, 2/4 test): `isalpha()` ile
  harf sayımı. Beklenen `0 3`, alınan `6 3`. Burada ek bir incelik vardır:
  C'de `isalpha()`'ya negatif bir `char` geçirmek (EOF dışında) tanımsız
  davranıştır ve glibc/UCRT pratikte bu baytları harf saymaz; modelin
  `u8`-tabanlı çevirisi ise 127-üstü baytları harf saydı.
- **s66_xor_checksum_signed_extend** (FE, 22 satır, 4/5 test): XOR sağlama
  toplamının işaretli genişletilmesi. Beklenen `4294967247`, alınan `207` —
  C'de `char` işaretli olduğu için `(unsigned)(int)(char)0xCF` işlemi
  işaret genişletmesiyle `4294967247` üretirken, Rust'ın `u8→u32`
  sıfır-genişletmesi `207` üretti.

#### 2.18.D Kategori D (çıktı biçimlendirme, %g) — 3 yeni başarısızlık, 0 PASS

Üçünün de başarısız olmasıyla kategori D **altı bağımsız örneğe ve dört ayrı
kod tabanına** ulaştı; veri setindeki en yaygın kök nedendir.

- **s67_stats_stddev_format** (FE, 28 satır, 2/4 test): Standart sapma.
  Beklenen `2 0.816497`, alınan `2 0.816496580927726`.
- **s68_currency_round_format** (FE, 16 satır, 2/4 test): Para birimi
  yuvarlama. Beklenen `0.3`, alınan `0.30000000000000004` — bu vaka
  özellikle nettir: ikili kayan nokta gösteriminin klasik artığı C'nin
  `%g`'si tarafından (6 anlamlı basamağa yuvarlanarak) gizlenirken, Rust'ın
  varsayılan `{}` biçimi tam hassasiyeti yazdırıp artığı açığa çıkarıyor.
- **s69_sqlite_snprintf_g** (FE, 17 satır, 1/4 test): SQLite'ın `%g`
  kullanan `snprintf` sarmalayıcısı (**gerçek üretim kodu**). Beklenen
  `3.14286`, alınan `3.142857142857143`. Kategori D'nin bağımsız bir
  dördüncü kod tabanında (SQLite) doğrulanması, bunun sistematik bir C↔Rust
  boşluğu olduğunu bir kez daha pekiştirir.
- **Model-bağımsızlık:** Gemini de bu üç örneğin **üçünde birden**
  başarısızdır (üçü de CE — `%g` taklidini yazarken geçersiz Rust biçim
  dizesi ürettiği için hiç derlenmedi). Bkz. §7.4.

#### 2.18.E Kategori E (güvensiz global durum) — 0 yeni başarısızlık, 3 PASS

**Bu, genişletmenin en beklenmedik bulgusudur.** Kategoriye eklenen üç
örneğin (**s70_global_lcg_rng** — global tohumlu LCG üreteç;
**s71_global_errbuf** — global hata dizgesi tamponu; **s72_global_log_level**
— global günlük seviyesi) **üçü de ilk seferde PASS oldu.** Çeviri dosyaları
incelendiğinde model üçünde de C'ye sadık biçimde `static mut` seçmiş, ancak
erişimleri doğru şekilde `unsafe { ... }` bloklarına sarmıştır
(`translations_rust/s70_global_lcg_rng.rs` vb.).

s50_id_generator ile birlikte kategori E'nin **beş örneğinden dördü PASS**
olmuştur; yalnızca s19 başarısızdır. **Sonuç:** Kategori E artık sistematik
bir C↔Rust semantik boşluğu olarak değil, **modelin aynı yapısal kalıpta
tutarsız davranmasının** (5 denemede 1 kez `unsafe` sarmalamayı unutma) bir
örneği olarak yorumlanmalıdır. Bu aynı zamanda veri setindeki tek CE
örneğinin neden hâlâ tek olduğunu da açıklar ve makalede kategori E'nin
"düşük öncelikli/kararsız" bir kategori olarak nitelendirilmesini gerektirir.

#### 2.18.F Kategori F (platform tamsayı genişliği) — 4 yeni başarısızlık, 0 PASS

Üç hedefli örnek **ve** hedeflenmemiş gerçek üretim kodundan bağımsız olarak
gelen bir dördüncü örnek başarısız oldu; kategori F artık altı örnekle veri
setinin en geniş ikinci kategorisidir.

- **s73_bsd_atoi_overflow** (FE, 22 satır, 2/4 test): Elle yazılmış sayısal
  ayrıştırıcı, `unsigned long` biriktirici kullanır. Beklenen `705032704`,
  alınan `5000000000`. Aritmetik tam olarak doğrulanabilir:
  5000000000 mod 2³² = 705032704 — yani bu ortamda 32-bit olan
  `unsigned long` **tanımlı biçimde sarar** (C'de unsigned taşma UB
  değildir), oysa `u64` seçen çeviride sarma hiç gerçekleşmez. Bu vaka
  kategori A ile F'nin kesiştiği ilginç bir noktadır: sarma davranışı
  doğrudur, ama **hangi genişlikte** saracağı platforma bağlıdır.
- **s74_platform_loop_counter** (FE, 24 satır, 2/4 test): `LONG_MAX`/
  `LONG_MIN` sınırında kırpma yapan bir çarpma (s51'in toplama yerine
  çarpma yapan eşi). Beklenen `2147483647`, alınan `2500000000` — `i64`
  seçen çeviri 32-bit sınırında hiç kırpmaz.
- **s75_bsd_strtoul** (FE, 111 satır, 2/4 test): **Gerçek BSD libc üretim
  kodu.** Beklenen `result=4294967295 errno=ERANGE consumed=10`, alınan
  `result=5000000000 errno=OK consumed=10` — C referansı `ULONG_MAX`'e
  sabitleyip `ERANGE` bildirirken çeviri hiç taşma bildirmiyor.
- **s103_nginx_hextoi** (FE, 63 satır, 3/4 test): **nginx'in gerçek
  `ngx_hextoi()` fonksiyonu** (BSD-2-Clause), hedeflenmemiş üretim kodu
  katmanından bağımsız biçimde ortaya çıktı. Beklenen `-1` (geçersiz/taşma
  göstergesi), alınan `4294967295`. C kodundaki `value > cutoff` taşma
  kontrolü `LONG_MAX/16`'ya dayanır; `long` 32-bit olduğunda bu eşik çok
  daha düşüktür, `i64`/`u32` tabanlı çeviride ise kontrol hiç tetiklenmez.
- **⚠️ Platform duyarlılığı artık altı örneğe yayılmıştır:** Bu dört yeni
  örnek de (s38, s51 ile birlikte) Windows (LLP64) ile Linux (LP64)
  arasında **PASS/FAIL durumunu tam olarak yer değiştirir** — Round 1'de
  Windows'ta FAIL/Linux'ta PASS, Round 2'nin Windows'a özgü `i32`
  düzeltmesinden sonra ise tam tersi (bkz. §6). "Hata geri bildirimiyle
  düzeltilmiş bir çeviri evrensel olarak doğrudur" varsayımına karşı kanıt
  böylece 2 örnekten 6 örneğe çıkmıştır.

#### 2.18.G Kategori G (usize taşması) — 3 yeni başarısızlık, 0 PASS

- **s76_array_shrink_countdown** (RE, 26 satır, 2/4 test): Beklenen
  `10 20 30`, alınan: panik.
- **s77_ring_buffer_index** (RE, 25 satır, 1/4 test): Halka tampon indeks
  aritmetiği; beklenen `3\n2\n3`, alınan: panik.
- **s78_sliding_window_min** (RE, 30 satır, 0/4 test — **hiçbir testi
  geçmedi**): Beklenen `2`, alınan: panik.
- **⚠️ Önceki bir gözlemin düzeltilmesi:** §2.10'da (s40) ve §4'te, kategori
  G'nin paniklerinin "release modunda dahi maskelenemediği" belirtilmişti —
  çünkü s40/s52'de taşmanın hemen ardından gelen dizi erişimi Rust'ın her
  zaman etkin sınır kontrolüne takılıyordu. **Yeni üç örnek bunu kısmen
  çürütmüştür:** release modunda s78 PASS'e, **s76 ve s77 ise FE'ye (sessiz
  yanlış çıktı)** dönüşür. Yani maskelenememe yalnızca taşan indeksin hemen
  bir sınır kontrolüne çarptığı durumlarda geçerlidir; taşan değerin
  sınırlar içinde kalabildiği veya döngünün hiç çalışmadığı durumlarda taşma
  sessizce sarabilmektedir. **Bu, kategori G'yi kategori A'dan daha
  tehlikeli kılar:** aynı hata, derleme moduna göre hem gürültülü (RE) hem
  sessiz (FE) olabilmektedir.

#### 2.18.H Kategori H (switch fallthrough) — 3 yeni başarısızlık, 0 PASS

Üçünün de başarısız olmasıyla kategori H beş bağımsız örneğe ulaştı ve yeni
örneklerde %100 tetiklenme oranı gösterdi.

- **s79_http_status_class** (FE, 26 satır, 1/4 test): HTTP durum kodu sınıfı;
  beklenen `7`, alınan `4`.
- **s80_state_machine_fallthrough** (FE, 23 satır, 1/4 test): Beklenen `2`,
  alınan `1`. **Bu örnek kategoriyi genişletir:** fallthrough burada
  kümülatif sayısal birikim için değil, bir **durum makinesi geçişi** için
  kullanılıyor — model bu bağlamda da düşme davranışını yeniden üretmedi.
- **s81_grade_bucket_fallthrough** (FE, 25 satır, 1/4 test): Beklenen `4`,
  alınan `1`.

#### 2.18.I Kategori I (makro çoklu-değerlendirme) — 2 yeni başarısızlık, 1 PASS

- **s82_macro_minmax_sideeffect** (FE, 22 satır, **0/4 test**): `MAX`/`MIN`
  makrolarına yan etkili argüman (`x++`). Beklenen `6 10 8`, alınan
  `5 10 7` — s56 ile aynı kök neden, farklı makro ailesi.
- **s83_macro_swap_no_temp** (FE, 31 satır, **0/4 test**) — **kategoriyi yeni
  bir yan etki türüne genişletir:** Burada yan etkili olan, makro
  argümanının *değeri* değil **erişim yolunun kendisidir**. C kodu
  `SWAP(arr[i++], arr[1], tmp)` çağırır; `SWAP` gövdesinde `a` iki kez geçer
  (`tmp = a` ve `a = b`), bu yüzden `i` **iki kez artar** ve ikinci atama
  dizinin **farklı bir elemanına** yazar. Beklenen `10 10 30 40 50 / i=2`,
  alınan `20 10 30 40 50 / i=1`. Bir Rust fonksiyonu argümanı bir kez
  değerlendirdiğinden bu davranış yeniden üretilemez; yalnızca kasıtlı bir
  `macro_rules!` ile taklit edilebilir.
- **s84_xmacro_enum_strings PASS oldu** (saf X-Macro/token-pasting, enum +
  isim tablosu üretimi). Bu, s56'da ilk kez gözlenen ayrımı **üçüncü kez**
  doğrular: sorun karmaşık makro kullanımının kendisi değil, yalnızca
  **yan etkili bir argümanın çoklu genişletilmesidir.**

#### 2.18.J Bu 21 vakadan çıkan üç genel sonuç

1. **Taksonomi doyuma ulaştı:** 73 yeni örnek, dokuz kategoriye ek olarak
   onuncu bir kök neden ortaya çıkarmadı.
2. **Kategoriler eşit güçte değil:** C, D, F, G, H kategorilerinin yeni
   örneklerinde tetiklenme oranı %100 (3/3 veya 4/4) iken, A ve I'de %67
   (2/3), B'de %33 (1/3), E'de **%0** (0/3). Makalede kategoriler bu
   tetiklenme oranıyla birlikte sunulmalıdır — hepsi aynı derecede
   sistematik değildir.
3. **Gerçek üretim kodu neredeyse dokunulmamıştır:** 46 örneklik
   s85-s130 katmanından yalnızca bir örnek (s103) başarısız oldu ve o da
   zaten bilinen bir kategoriye (F) düştü.

---

## 3. Başarılı Örneklerin Analizi (92/130)

Yalnızca başarısızlıklar değil, ilk seferde geçen örnekler de analiz
edilmiştir: ne test ediliyordu, neden sorunsuz geçti (hangi risk
gerçekleşmedi ya da hangi tasarım tercihi C'nin sözleşmesiyle zaten
örtüşüyordu) ve varsa dikkat çekici yönü. §3.1-§3.9 n=57 aşamasındaki 40
PASS örneği tek tek ele alır; §3.10 ise genişletmeyle gelen 52 yeni PASS
örneğini grup bazında inceler.

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

- **s37_bsd_getopt** (getopt(), 148 satır, 5 test): OpenBSD/FreeBSD'nin gerçek, ~39 yıllık komut satırı ayrıştırıcısı. Dışa açık değiştirilebilir global durum (`optarg`/`optind`/`optopt`/`opterr`) gerektirir — model bunu Rust'ta `static mut` + `unsafe fn` ile **sadakatle** yansıttı (bu veri setindeki 7 gerçek-`unsafe`-kullanımından biri, bkz. §8) çünkü C kodunun kendisi bu sözleşmeyi yapısal olarak dayatıyor. İlk seferde PASS.
- **s39_bsd_heapsort** (heapsort(), 143 satır, 4 test): Generic `void*` yığın sıralaması — model, çağıran kodun (`main()`) yalnızca tamsayı sıralaması ihtiyacını tanıyarak genel `void*` imzasını taklit etmek yerine güvenli, deyimsel bir Rust dizi/dilim (slice) tabanlı sıralama yazdı — **hiç `unsafe` kullanmadan**. İlk seferde PASS.
- **s46_musl_qsort** (musl libc'nin smoothsort'u, 262 satır, 5 test): Bit-düzeyinde Leonardo-sayı kodlamasıyla çalışan, veri setindeki en karmaşık tek algoritma; genel-amaçlı `void*` imzasına gerçekten bağımlı olduğundan model burada `unsafe` kullandı (yapısal gereklilik). PASS.
- **s47_redis_sds** (Redis'in SDS dinamik string kütüphanesi, 522 satır, 5 test): Veri setindeki **en uzun program**; başlık bilgisini (uzunluk/kapasite/tür bayrağı) pointer'ın hemen öncesinde gizli tutan, C'ye özgü bir bellek düzeni kullanır. Model bu düzeni ham pointer aritmetiğiyle yeniden üretmeyi denemek yerine tamamen güvenli bir `String` tabanlı iç temsille yeniden yapılandırdı ve gözlemlenebilir API davranışını (uzunluk/kırpma/aralık/karşılaştırma) hiç `unsafe` kullanmadan birebir korudu. İlk seferde PASS — **ama bkz. §6, bu örnek Linux/Docker'da bir CRLF/stdio bulgusuyla farklı sonuç vermiştir (C referansının kendisi platforma bağlı davranıyor, Rust çevirisi değil).**

### 3.6 Hedeflenmemiş boşluklar grubundan PASS olanlar (4 örnek)

- **s41_float_bits** (float→bit örüntüsü, union/type punning, 21 satır): C'nin `union` ile IEEE-754 bit-düzeyinde yeniden yorumlaması, model tarafından Rust'ın güvenli `to_bits()`/`from_bits()` fonksiyonlarıyla birebir doğru eşlendi — `unsafe` transmute'a hiç gerek kalmadı.
- **s42_bitfields** (bit-alanı kırpma, 25 satır): C bit-alanlarının (bit-fields) atamada kırpma davranışı, model tarafından maskeleme/kaydırma operasyonlarıyla doğru yeniden üretildi.
- **s44_fib_memo_static** (fonksiyon-lokal static memoizasyon, 29 satır): Çağrılar arasında kalıcı, değiştirilebilir durum gerektiren bir C deseni (`static int cache[]`); model bunu Rust'ta `static mut` + `unsafe fn` ile **doğru ve gerekçeli** biçimde yansıttı (veri setindeki 7 gerçek-`unsafe`-kullanımından biri, bkz. §8 — s19'un aksine burada durum gerçekten fonksiyon-lokal ve tek-thread varsayımına uygun tasarlandığından model haklı olarak `unsafe`'i seçti).
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

### 3.10 Genişletmeden gelen 52 yeni PASS örneği (s58-s130)

Genişletmenin 73 örneğinden 52'si ilk seferde geçti. Bu grup, önceki
bölümdeki gözlemi çok daha büyük bir örneklemde doğrular.

**3.10.a Hedefli katmandan geçenler (7/27):** s60_elf_hash (§2.18.A),
s61_utf8_byte_vs_char_count ve s62_strtok_tokenizer (§2.18.B),
s70/s71/s72 global durum üçlüsü (§2.18.E) ve s84_xmacro_enum_strings
(§2.18.I). Bunların her biri yukarıda ilgili kategori altında
açıklanmıştır; ortak noktaları, hedeflenen semantik boşluğun o özel kod
deseninde **gözlemlenebilir çıktıyı değiştirmemesidir** (maskeleme
nedeniyle taşmanın hiç tetiklenmemesi, ASCII girdiyle bayt=karakter
olması, modelin `unsafe`'i doğru sarması, yan etkisiz makro kullanımı).

**3.10.b Gerçek üretim kodu katmanı (24/25 PASS — %96):** Bu, veri
setindeki en güçlü dış geçerlilik kanıtıdır. Yaygın kullanılan gerçek
kütüphane kodu neredeyse tamamen doğru çevrilmiştir:

- **SQLite:** `sqlite3_stricmp` (s85), `strglob` desen eşleme (s86, 185
  satır — veri setindeki en uzun üçüncü program), UTF-8 okuyucu (s87).
- **zlib:** `adler32` (s88), tablo tabanlı CRC-32 (s89) — ikisi de yoğun
  unsigned aritmetik içermesine rağmen PASS; s88'de model `%` mod
  aritmetiğini, s89'da ise XOR/kaydırmayı doğru korudu (kategori A'nın
  neden yalnızca sınırlandırılmamış çarpma/toplamada tetiklendiğinin bir
  başka kanıtı).
- **curl** URL kod çözme (s90), **Redis** `ll2string` (s91) ve
  `stringmatchlen` (s95, 152 satır), **OpenSSL** Base64 kodlama (s92),
  **libsodium** `bin2hex` (s93), **FreeBSD** `reallocarray` (s94).
- **cJSON:** dize yazdırma (s96) ve `\uXXXX` Unicode çözme (s97) — dikkat
  çekici: cJSON'un **sayı** yazdırma tarafı (s48) kategori D'den başarısız
  olurken, **dize** tarafı sorunsuz geçmiştir; yani başarısızlık
  kütüphaneye değil, spesifik semantik boşluğa bağlıdır.
- **musl libc:** `memmem` (s98, 158 satır — Two-Way algoritması),
  `strsep` (s99), `strverscmp` (s100), `strcasestr` (s104), `memrchr`
  (s105). Bunların hepsi C'de ham işaretçi aritmetiğine dayanır ve model
  hepsini **hiç `unsafe` kullanmadan**, dilim/slice tabanlı güvenli Rust'a
  çevirmiştir.
- **OpenBSD/FreeBSD libc:** `strlcpy` (s101), `strnstr` (s102),
  `strtonum` (s106), `timingsafe_bcmp` (s107) — s107 özellikle ilginçtir:
  sabit zamanlı karşılaştırma, erken çıkış yapmayan bir döngü gerektirir
  ve model bu güvenlik sözleşmesini (kısa devre yapmama) korumuştur.
- **nginx** `ngx_atoi` (s108) ve **Apache HTTP Server** `getword` (s109).
- Tek başarısızlık: **s103_nginx_hextoi** (§2.18.F).

**3.10.c Çeşitlilik/çok dosyalı/eşzamanlılık katmanı (21/21 PASS — %100):**

- **Çok dosyalı modüller:** s110_queue_module, s111_linked_list_module —
  `manifest.json` ile birden fazla derleme birimi; ikisi de Rust'ın `mod`
  sistemine sorunsuz eşlendi.
- **Gerçek eşzamanlılık (3 örnek):** s112_producer_consumer_threads
  (pthread + koşul değişkeni), s113_rwlock_counter (okuma-yazma kilidi),
  s114_simple_threadpool (iş parçacığı havuzu). Üçü de ilk seferde doğru
  çevrildi — model `Arc<Mutex<>>`, `Condvar`, `RwLock` ve kanal
  desenlerini doğru seçti, hiçbirinde derleme hatası veya veri yarışı
  oluşmadı. s57 ile birlikte eşzamanlılık örnekleri artık 4/4 PASS'tır;
  bu, "Rust'ın tip sistemi paylaşılan-durum çevirisini zorlaştırır"
  hipotezini bu dört örnekte de doğrulamayan tutarlı bir olumsuz
  sonuçtur (negative result).
- **Dil özellikleri:** s115_bitvector_set, s116_tagged_union_variant
  (etiketli birleşim → Rust `enum` ile doğal eşleme),
  s117_goto_retry_loop, s118_variadic_sum (C değişken argümanlı
  fonksiyonu → makro/dilim tabanlı yeniden yapılandırma),
  s119_setjmp_error_handling (`setjmp`/`longjmp` → `Result` tabanlı hata
  yayılımına yeniden yapılandırıldı — kontrol akışı tamamen farklı, ama
  gözlemlenebilir davranış özdeş), s120_function_pointer_dispatch.
- **Klasik veri yapıları/algoritmalar:** s121_recursive_descent_calc2,
  s122_trie_insert_search, s123_avl_tree_insert (116 satır — döndürme
  mantığı dahil), s124_graph_bfs, s125_graph_dfs_cycle,
  s126_priority_queue_heap, s127_gauss_matrix_inverse,
  s128_custom_tokenizer, s129_command_dispatch_table,
  s130_qsort_callback_structs. Hepsi `Option<Box<T>>`/`Vec` tabanlı
  güvenli Rust'a çevrildi.

**Genel yorum:** 52 yeni PASS örneğinin hiçbirinde kod uzunluğu, dinamik
bellek, ham işaretçi kullanımı, çok dosyalılık veya eşzamanlılık tek
başına başarısızlık üretmemiştir. Bu, §3.9'daki gözlemi n=130 ölçeğinde
doğrular ve makalenin ana tezini güçlendirir: **risk, kodun ölçeğinde
değil, sayılabilir ve tanımlanabilir bir semantik boşluk kümesindedir.**

---

## 4. Dokuz Kök Neden — Özet Tablo

| # | Kategori | Örnekler (n=130) | Sayı | Tür | C sözleşmesi → LLM'in seçimi |
|---|---|---|---|---|---|
| A | Unsigned taşma | s09, s14, s58, s59 | 4 | RE | Kasıtlı mod-2ⁿ sarma → panik-eden varsayılan aritmetik |
| B | String modeli | s06, s13, s63 | 3 | FE | Bayt-düzeyi işleme → `.chars()` (Unicode) |
| C | char işaretliliği | s20, s49, s64, s65, s66 | 5 | FE | İşaretli char (127+ negatif) → `u8`/`i32` (hep pozitif) |
| D | Çıktı biçimlendirme | s15, s27, s48, s67, s68, s69 | 6 | FE | `%g` anlamlı-basamak/üstel mantığı → varsayılan `{}` |
| E | Güvensiz global durum | s19 | 1 | CE | Değiştirilebilir global → `static mut` (unsafe gerekir) |
| F | Platform tamsayı genişliği | s38, s51, s73, s74, s75, s103 | 6 | FE | Platformun gerçek `long` genişliği → sabit `i64` |
| G | usize taşması | s40, s52, s76, s77, s78 | 5 | RE | İşaretli int ile güvenli çıkarma → `usize` altında taşma |
| H | Switch fallthrough | s43, s53, s79, s80, s81 | 5 | FE | Kümülatif düşme → bağımsız `match` kolları |
| I | Makro çoklu-değerlendirme | s56, s82, s83 | 3 | FE | Metinsel ikame (çoklu değerlendirme) → fn (tek değerlendirme) |
| | | **Toplam** | **38** | | |

**Kategorilerin tetiklenme gücü eşit değildir.** Genişletmede her kategoriye
3'er yeni örnek eklendiğinde ortaya çıkan tetiklenme oranları, makalede
kategorilerin yan yana ve eşit ağırlıkta sunulmasının yanıltıcı olacağını
göstermektedir:

| Kategori | Yeni örneklerde tetiklenme | Yorum |
|---|---|---|
| C, D, F, G, H | 3/3 (%100) | Güçlü, sistematik ve tekrarlanabilir |
| A, I | 2/3 (%67) | Sistematik, ama koşula bağlı (maskeleme/yan etkisiz kullanım) |
| B | 1/3 (%33) | Yalnızca çıktı gerçekten değiştiğinde tetikleniyor |
| **E** | **0/3 (%0)** | **Sistematik bir boşluk değil; modelin tutarsızlığı** |

**Ortak örüntü:** Sekiz kategoride (E hariç) Rust kodu sözdizimsel olarak
geçerlidir ve "daha deyimsel/modern" görünen bir tercihi yansıtır — model,
kaynağın C'de gerçekten sahip olduğu bit genişliği/işaretlilik/sarma/kontrol
akışı sözleşmesini korumak yerine, hedef dilin idiyomatik varsayılanını
seçmiştir. Yalnızca kategori E'de aynı türden bir birebir aktarım Rust'ın tip
güvenliği tarafından derleme aşamasında yakalanmıştır — ve n=130'daki dört ek
örnek (s50, s70, s71, s72), modelin bu aktarımı **çoğunlukla doğru** yaptığını
(erişimi `unsafe` bloğuna sararak) göstermiştir; s19 kural değil istisnadır.

**n=130'un en önemli yapısal bulgusu:** 73 yeni örnek, bu dokuz kategoriye ek
olarak **onuncu bir kök neden ortaya çıkarmamıştır.** Veri seti
24→29→36→39→45→48→53→57 aşamalarında dört kez yeni kategori üretmişken
(F, G, H, I), 57→130 gibi çok daha büyük bir sıçramada hiç yeni kategori
üretmemesi, taksonominin bu kod sınıfı (tek dosyaya indirgenebilir, stdin/
stdout sözleşmeli C programları) için **doyuma ulaştığına** dair ilk somut
işarettir.

---

## 5. Kısıtlı Geri Bildirim Deneyi (Tablo 5'in arkasındaki tam veri)

Aynı başarısızlıklara üç farklı ayrıntı seviyesinde geri bildirim verildi.

**n=57 (kör protokolün gerçekten uygulandığı ölçüm):**

- **Seviye A (Oracle):** Tam derleyici hatası + panik metni + beklenen/alınan fark → **57/57 = %100**
- **Seviye B (Orta/CI-benzeri):** Derleyici/panik metni tam; FE için yalnızca girdi (fark yok) → **49/57 = %85.96**
- **Seviye C (Minimal):** Yalnızca gerçek başarısız test sayısı (CE hariç) → **41/57 = %71.93**

**n=130 (kör protokolün artık tamamı üzerinde gerçekten uygulandığı ölçüm):**

- **Seviye A (Oracle):** **130/130 = %100.00**
- **Seviye B (Orta/CI-benzeri):** **122/130 = %93.85**
- **Seviye C (Minimal):** **114/130 = %87.69**

> **✅ Düzeltme notu (2026-08-02) — Seviye B/C kör protokolü n=130'un
> tamamında tamamlandı.** Önceki bir turda bir denetim ajanı
> `translations_rust_levelB/` ve `translations_rust_levelC/` klasörlerini
> bayt düzeyinde denetleyip şunu bulmuştu: n=57 aşamasındaki **17 eski
> başarısızlık** için protokol doğru uygulanmıştı (Seviye B dosyaları hem
> Round 1 hem Round 2 sürümlerinden farklıydı, yani gerçek bağımsız "kör"
> düzeltme denemeleriydi), ancak **s58-s130 arasında eklenen 21 yeni
> başarısızlık** için Seviye B/C'de "düzeltilmiş" görünen her örneğin
> dosyası Round 2'nin oracle düzeltmesiyle bayt düzeyinde özdeşti (Seviye
> B'de 15, Seviye C'de 8 örnek) — yani bu 21 örnek için kısıtlı bilgiyle
> bağımsız bir düzeltme denemesi hiç yapılmamış, doğrudan oracle yanıtı
> kopyalanmıştı. Bu, o turdaki B (%89.23) ve C (%77.69) sayılarını yapay
> biçimde şişiriyordu.
>
> Sorun giderildi: bu 21 örneğin Seviye B ve Seviye C çevirileri sıfırdan,
> gerçekten kısıtlı bilgiyle (oracle dosyasına hiç bakılmadan) yeniden
> yazıldı ve harness her iki klasör üzerinde yeniden çalıştırıldı.
> **Sonuç: 21 yeni başarısızlığın tamamı (21/21) hem Seviye B'de hem
> Seviye C'de kısıtlı bilgiyle düzeltilebilmiştir.** B/C'deki düşüş
> tamamen n=57 aşamasından kalan 17 eski başarısızlıktan kaynaklanır ve
> oradaki sayılar (Seviye B'de 9/17, Seviye C'de 1/17 düzeltilmiştir)
> hiç değişmemiştir. Ayrıntı için bkz. `MODIFICATIONS.md` — "Seviye B/C
> Kör Protokol Düzeltmesi" girdisi.

Aşağıdaki gerekçe analizi, protokolün doğru uygulandığı **n=57 ölçümüne**
aittir:

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

**Yeni örneklerde örüntü tekrarlanmadı — 21 yeni başarısızlığın tamamı
düzeltildi:** n=130'da Seviye B'de düzeltilemeyen 8 örneğin tamamı n=57
aşamasından kalan eski başarısızlıklardır (s15/s27/s48, s38/s51, s43/s53,
s56 — n=57 ölçümüyle birebir aynı sekizli); **21 yeni başarısızlığın
(s58-s130 arası) tamamı hem Seviye B'de hem Seviye C'de gerçekten kısıtlı
bilgiyle düzeltilebilmiştir.** Bu, n=57'de gözlenen "belirli kategoriler
kısıtlı bilgiyle düzeltilemez" örüntüsünün yeni örneklerde aynı biçimde
tekrarlanmadığını gösterir — olası bir açıklama, yeni 21 örneğin çoğunun
(s85-s130 katmanı) gerçek üretim kodundan alınmış, göreli olarak daha
uzun/bağlamsal programlar olması ve bu bağlamın kısıtlı geri bildirimle
birlikte modele kategoriyi tahmin etmek için daha fazla ipucu sunmasıdır;
bu yorum doğrulanmamış bir varsayımdır, sistematik olarak test edilmemiştir.

---

## 6. Çoklu Platform Analizi (Windows LLP64 vs Linux/Docker LP64)

Windows: MSYS2/UCRT64 gcc 16.1.0 + rustc 1.97.1, `long`=32-bit.
Linux: Docker ubuntu:24.04, gcc 13.3.0 + rustc 1.97.1 (**birebir aynı rustc
sürümü** — gözlenen farkın Rust derleyicisinden değil C tarafındaki `long`
genişliğinden kaynaklandığını netleştirir).

| Koşul | Windows EA | Linux EA | Fark |
|---|---|---|---|
| Round 1 — debug | %70.77 (92/130) | %74.62 (97/130) | +3.85 puan |
| Round 1 — release | %74.62 (97/130) | %78.46 (102/130) | +3.84 puan |
| Round 2 — iyileştirilmiş | %100.00 (130/130) | **%94.62 (123/130)** | **−5.38 puan** |

(Veri setinin tamamı — 130 örnek — her iki platformda yeniden çalıştırıldı.
Genişletmeyle birlikte platforma duyarlı örnek sayısı 3'ten **7'ye**
çıkmıştır.)

**Platforma göre farklılaşan 7 örnek:**

| Örnek | Kategori | Round 1 (Win → Linux) | Round 2 (Win → Linux) |
|---|---|---|---|
| s38_bsd_strtol | F | FE → pass | pass → FE |
| s51_long_clamp | F | FE → pass | pass → FE |
| s73_bsd_atoi_overflow | F | FE → pass | pass → FE |
| s74_platform_loop_counter | F | FE → pass | pass → FE |
| s75_bsd_strtoul | F | FE → pass | pass → FE |
| s103_nginx_hextoi | F | FE → pass | pass → FE |
| s47_redis_sds | (CRLF/stdio) | pass → FE | pass → FE |

**Kategori F'nin altı örneği (Round 2, "düzeltilmiş"):** Windows için
yazılan `i32` düzeltmesi, Linux'ta `long` gerçekten 64-bit olduğundan artık
**yanlış** sonuç üretiyor — bu altı örneğin her birinde iki platform arasında
PASS/FAIL durumu tam olarak yer değiştiriyor. Bu, "iyileştirilmiş" bir
çevirinin evrensel değil platforma özgü olabileceğinin doğrudan kanıtıdır ve
**n=130 ile bu kanıt 2 örnekten 6 örneğe çıkarak anekdot olmaktan tamamen
çıkmıştır.** Özellikle s103_nginx_hextoi kayda değerdir: bu örnek veri
setinin hedefli bölümünden değil, hedeflenmemiş gerçek üretim kodu
katmanından (nginx) gelmiştir — yani platforma özgü düzeltme sorunu,
kasıtlı olarak kurgulanmış bir senaryo değil, gerçek kodda kendiliğinden
ortaya çıkan bir olgudur.

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

## 7. Çoklu Model Analizi (Gemini + Claude Haiku, n=130)

Veri seti n=130'a genişletildikten sonra üç model aynı programlar üzerinde,
aynı zero-shot istemle ölçülmüştür:

| Model | Kapsam | EA (örnek) | EA % | CE | RE | FE | NT |
|---|---|---|---|---|---|---|---|
| Claude Sonnet 5 (referans, Round 1) | 130/130 | 92/130 | %70.77 | 1 | 9 | 28 | 0 |
| Claude Haiku | 130/130 | 94/130 | %72.31 | 8 | 5 | 23 | 0 |
| Google Gemini (`gemini-flash-latest`) | 130/130 | 116/130 | %89.23 | 10 | 0 | 4 | 0 |

> **Gemini ölçümü 130 örneğin tamamı üzerinde tamamlanmıştır.** Google AI
> Studio ücretsiz katmanının günlük istek kotası nedeniyle çeviriler
> 2026-07-22 – 2026-08-03 arasında birkaç güne yayılarak üretilmiştir, ancak
> 130 örneğin **tamamı** gerçek API çağrısıyla çevrilmiş ve değerlendirilmiştir
> (her çağrının istemi/zaman damgası/parametreleri `results/manifest_gemini.json`
> içindedir). Dolayısıyla %89.23 tam kapsamlı bir ölçümdür ve diğer iki
> modelin sayılarıyla **doğrudan karşılaştırılabilir**. Release modunda da
> birebir aynı sonuç elde edilmiştir (116/130, %89.23 — hiçbir örnek debug ile
> release arasında yer değiştirmemiştir). Aşağıdaki §7.1-§7.5 vaka analizleri,
> veri seti n=57 aşamasındayken yapılan Gemini ölçümüne dayanır ve o örnekler
> için hâlâ geçerlidir; §7.6 ise n=130'a genişletmeyle gelen yeni gözlemleri
> özetler.

**Gemini'nin 14 başarısızlığının tam listesi (n=130):** 10'u derleme
hatasıdır (CE) — s26_rpn_calculator, s27_csv_stats, s46_musl_qsort,
s48_cjson_number, s67_stats_stddev_format, s68_currency_round_format,
s69_sqlite_snprintf_g, s75_bsd_strtoul, s94_freebsd_reallocarray,
s112_producer_consumer_threads; 4'ü fonksiyonel hatadır (FE) — s15_float_avg,
s47_redis_sds, s103_nginx_hextoi, s110_queue_module. Hiç çalışma zamanı
hatası (RE) veya sonlanmama (NT) gözlenmemiştir.

**Sessiz hata oranı modele göre çarpıcı biçimde değişiyor — bu, çalışmanın
en aktarılabilir bulgularından biridir:**

| Model | Başarısızlık | Derlemede yakalanan (CE) | Derlenip geçen (sessiz veya gürültülü) | Sessizlik oranı |
|---|---|---|---|---|
| Claude Sonnet 5 | 38 | 1 | 37 | **%97.4** |
| Claude Haiku | 36 | 8 | 28 | %77.8 |
| Gemini | 14 | 10 | 4 | **%28.6** |

Yani Gemini daha yüksek ham doğruluk gösterse de, **hatalarının çoğu
derleyici tarafından yakalanan türdendir**; Claude Sonnet 5'in hataları ise
neredeyse tamamen derleyiciden kaçan sessiz semantik hatalardır. Bir CI
hattında bu iki hata profilinin pratik riski taban tabana zıttır — yüksek EA,
düşük risk anlamına gelmez. Claude Haiku ise iki uç arasında yer alır.

### 7.1 Karşılaştırma Tablosu (n=57 aşamasındaki tam Gemini ölçümü)

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

### 7.5 Genel yorum (n=57 tam veriyle)

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

### 7.6 Genişletmenin (n=130) çoklu model açısından getirdiği yeni bulgular

**(a) Kategori D artık kesin biçimde model-bağımsız bir kör noktadır.**
Kategoriye eklenen üç yeni örneğin (s67, s68, s69) **üçünde de Gemini
başarısızdır** — üçü de CE, çünkü Gemini `%g` davranışını taklit etmeye
çalışırken geçersiz Rust biçim dizesi/söz dizimi üretmiştir (s27 ve s48'de
gözlenen örüntünün birebir tekrarı). Claude ise aynı üç örnekte FE
üretmiştir. Kategori D böylece **altı örnek, dört bağımsız kod tabanı ve iki
model boyunca** doğrulanmıştır: iki model de her seferinde başarısız olur,
yalnızca yüzeysel hata türü (Claude FE, Gemini CE) değişir. Bu, veri
setindeki tek gerçekten model-bağımsız C↔Rust boşluğudur.

**(b) Gemini'nin kategori F "üstünlüğü" bir tesadüfmüş.** Önceki turlarda
Gemini, `long` tipini `std::os::raw::c_long` ile çevirerek s38 ve s51'i
taşınabilir biçimde geçmiş, bu §7.2'de "daha doğru bir çözüm" olarak
raporlanmıştı. Yeni eklenen **s103_nginx_hextoi**'de ise Gemini de Claude
ile aynı hataya düşmüştür (FE) — yani `c_long` tercihi modelin tutarlı
biçimde uyguladığı bir kural değil, örneğe bağlı bir tercihtir. Bu, §7.2'nin
sonundaki "bu, Gemini'nin bu boşlukları anladığı anlamına gelmez" uyarısını
doğrudan doğrulayan yeni bir kanıttır.

**(c) Gemini eşzamanlılıkta ve çok dosyalı kodda zayıf.** Gemini'nin yeni
başarısızlıkları arasında **s112_producer_consumer_threads** (CE) ve
**s110_queue_module** (FE) bulunmaktadır — Claude her ikisini de ilk
seferde geçmiştir. Bu, §7.2b'de gözlenen "uzun/karmaşık gerçek kodda Claude
üstün" örüntüsünün çok dosyalı ve eşzamanlı koda da uzandığını gösterir.

**(c2) Tam kapsamla ortaya çıkan iki ek başarısızlık, aynı örüntüyü
pekiştirir.** Gemini'nin kalan başarısızlıkları olan **s75_bsd_strtoul** ve
**s94_freebsd_reallocarray** de gerçek BSD libc üretim kodudur ve ikisi de
CE'dir. s75'te Gemini, bayt okuyucuyu değiştirilebilir bir kapanışa (closure)
sarmış, sonra aynı indeksi kapanış dışından da okumaya çalışarak ödünç alma
(borrow) hatası almıştır (`E0503` — §7.3'te raporlanan `E0499` örüntüsünün
aynı ailesi). s94'te ise Gemini çeviriyi Rust'a taşımak yerine C'nin
`scanf`/`printf`/`realloc` çağrılarını `extern "C"` FFI ile olduğu gibi
bırakmış, üretilen ikili MSVC bağlayıcısında çözümlenememiştir — yani
"çeviri" yerine "sarmalama" tercihinin doğrudan bir maliyeti. Her iki örnek
de derleyicide yakalandığı için sessiz değildir.

**(d) Claude Haiku: katman-eşleştirilmiş üçüncü model.** Haiku 130/130 tam
kapsamda ölçülmüş ve EA = **%72.31 (94/130)** elde etmiştir — yani ham
doğrulukta Claude Sonnet 5 ile neredeyse aynıdır (%70.77). Ancak **hata
profili belirgin biçimde farklıdır:** Haiku 8 CE üretirken Sonnet 5 yalnızca
1 CE üretmiştir. Haiku'nun CE'leri ağırlıklı olarak veri setinin en
karmaşık/uzun örneklerindedir (s25_linked_list_ops, s37_bsd_getopt,
s71_global_errbuf, s75_bsd_strtoul, s87_sqlite_utf8_read,
s111_linked_list_module, s112_producer_consumer_threads,
s130_qsort_callback_structs). Ayrıca Haiku, Sonnet 5'in sorunsuz geçtiği
bazı **temel** örneklerde de başarısızdır (s01_sum, s07_bubble_sort,
s08_binary_search, s12_matrix_mult, s17_determinant, s21_matrix_transpose) —
yani Haiku'nun hataları kök-neden kategorilerine değil, genel kod-üretim
güvenilirliğine bağlıdır. Bu, aynı aileden iki model arasında bile hata
profilinin kategorik olarak farklılaşabildiğini gösterir ve "EA tek başına
model seçimi için yeterli bir ölçüt değildir" sonucunu güçlendirir.

**(e) McNemar testi (nihai, n=130 tam eşleştirilmiş karşılaştırma).**
`results/stats_report.md`'de raporlanan McNemar sonucu artık 130 örneğin
**tamamı** üzerinden hesaplanmıştır: ikisi de PASS 86, ikisi de FAIL 8,
yalnızca Claude FAIL 30, yalnızca Gemini FAIL 6; McNemar kesin (binom
tabanlı) iki-yönlü **p=0.0001**. İki modelin genel EA farkı bu nedenle
istatistiksel olarak anlamlıdır. Ancak anlamlı bir genel fark,
model×kategori etkileşiminin var olmadığı anlamına gelmez — §7.7'deki
kırılım, farkın yönünün kategoriye göre değiştiğini göstermektedir.

### 7.7 Model × kategori kırılımı (n=130, üç model)

Genel EA sıralaması (Gemini > Haiku ≈ Claude Sonnet 5) veri setinin
katmanları arasında sabit değildir; aşağıdaki kırılım, §7.5'te (d) olarak
tanımlanan **model × kod-karmaşıklığı etkileşiminin** n=130'daki tam
görünümüdür (her hücre: PASS/örnek sayısı):

| Veri seti katmanı | n | Claude Sonnet 5 | Gemini | Claude Haiku |
|---|---|---|---|---|
| Temel algoritmalar | 24 | 17/24 | 23/24 | 14/24 |
| Uzun özgün programlar | 5 | 4/5 | 3/5 | 2/5 |
| Rosetta Code | 7 | 7/7 | 7/7 | 7/7 |
| BSD libc | 3 | 2/3 | 3/3 | 0/3 |
| Hedeflenmemiş boşluk | 6 | 4/6 | 6/6 | 5/6 |
| musl/Redis/cJSON (en uzun üretim kodu) | 3 | 2/3 | **0/3** | 1/3 |
| Kök-neden 2. örnekleri | 5 | 1/5 | 5/5 | 4/5 |
| Çok dosyalı | 3 | 3/3 | 3/3 | 3/3 |
| Karmaşık makro | 1 | 0/1 | 1/1 | 0/1 |
| A-I derinleştirme (s58-s84) | 27 | 7/27 | 23/27 | 23/27 |
| Gerçek OSS üretim kodu | 25 | **24/25** | 23/25 | 20/25 |
| Çeşitlilik/çok-dosyalı/eşzamanlılık | 21 | **21/21** | 19/21 | 15/21 |
| **Toplam** | **130** | **92/130 (%70.77)** | **116/130 (%89.23)** | **94/130 (%72.31)** |

İki karşıt uç bu tabloda açıkça görülür: Gemini'nin üstünlüğü ezici biçimde
kısa/hedefli sızıntı katmanlarından gelir (A-I derinleştirme 23/27 vs 7/27;
kök-neden 2. örnekleri 5/5 vs 1/5), buna karşılık veri setinin en uzun ve
en karmaşık üretim kodu katmanında sıralama tersine döner (musl/Redis/cJSON:
Gemini 0/3, Claude 2/3) ve gerçek OSS üretim kodu ile eşzamanlılık
katmanlarında Claude Sonnet 5 öndedir (24/25 ve 21/21). Yani toplam EA
farkı tek başına "hangi model daha iyi" sorusuna cevap vermez.

---

## 8. Bellek Güvenliği ve `unsafe` Kullanımı

Round 1'in 130 çevirisinin tamamı `unsafe`/ham işaretçi kullanımı için
taranmıştır: yalnızca **7'si (%5.4)** gerçek `unsafe` kullanmıştır —
yedisi de C kodunun kendisinin yapısal olarak dayattığı bir gerekliliği
yansıtır:

- **s37 (getopt):** `optarg`/`optind`/`optopt`/`opterr` — çağıran kod
  tarafından okunması beklenen, dışa açık değiştirilebilir global durum.
- **s44 (fonksiyon-lokal static memoizasyon):** Çağrılar arasında kalıcı,
  değiştirilebilir durum gerektirir.
- **s46 (musl smoothsort):** Genel-amaçlı `void*` imzasına dayanan bit-düzeyi
  byte-pointer aritmetiği.
- **s50 (id_generator, Kategori E'nin 2. örneği):** s19 ile aynı
  `static mut` yapısı, ama bu kez erişim doğru biçimde `unsafe` bloğuna
  sarılmış (§3.7).
- **s70 (global LCG üreteç), s71 (global hata tamponu), s72 (global günlük
  seviyesi) — YENİ, kategori E'nin 3-5. örnekleri:** Üçünde de model yine
  `static mut` seçmiş ve erişimleri doğru biçimde `unsafe { ... }`
  bloklarına sarmıştır; üçü de ilk seferde PASS olmuştur.

**Bu, `unsafe` konusundaki yorumu tersine çeviren önemli bir bulgudur:**
Kategori E'nin beş örneğinden dördünde (s50, s70, s71, s72) model `unsafe`'i
**doğru kullandığı için başarılı** olmuş, yalnızca s19'da kullanmayı
unuttuğu için derleme hatası almıştır. Yani bu veri setinde `unsafe`
kullanımı bir risk göstergesi değil, C'nin global-durum sözleşmesinin doğru
tanınmasının göstergesidir; asıl anomali `unsafe`'in **kullanılmadığı** tek
vakadır (s19). Round 2'de s19 bu durumu `&mut i32` parametresine
dönüştüren, `unsafe` içermeyen güvenli bir tasarımla çözmüştür (dosyadaki
tek `unsafe` geçişi bir yorum satırıdır).

**Daha çarpıcı bulgu (n=130'da çok daha geniş kanıtla):** Ham işaretçi
aritmetiğine yapısal olarak bağımlı çok sayıda gerçek üretim kodu örneği
model tarafından **hiç `unsafe` kullanılmadan**, tamamen güvenli/deyimsel
Rust'a çevrildi ve ilk seferde PASS oldu:

- `heapsort()` (s39_bsd_heapsort, generic `void*`) ve Redis'in SDS string
  kütüphanesi (s47_redis_sds, pointer-öncesi gizli başlık düzeni).
- Yeni üretim kodu katmanının klasik pointer-yoğun libc fonksiyonları:
  musl `memmem` (s98, Two-Way algoritması), `strsep` (s99), `strcasestr`
  (s104), `memrchr` (s105), OpenBSD/FreeBSD `strlcpy` (s101), `strnstr`
  (s102), SQLite `strglob` (s86) — hepsi dilim/slice tabanlı güvenli
  indekslemeyle çevrildi.
- Dinamik veri yapıları: bağlı liste (s25, s111), BST (s28), hash tablosu
  (s29), trie (s122), AVL ağacı (s123), öncelik kuyruğu (s126) — hepsi
  tutarlı biçimde `Option<Box<T>>`/`Vec` deseniyle güvenli Rust'a çevrildi.
- Gerçek eşzamanlılık: s57, s112, s113, s114 — `Arc<Mutex<>>`, `Condvar`,
  `RwLock` desenleriyle, hiç `unsafe` kullanmadan.

**Sonuç:** `unsafe` kullanımı kodun karmaşıklığından değil, C kodunun dışa
açık davranış sözleşmesinin doğasından (harici mutable durum, fonksiyon ömrü
boyunca kalıcı durum, generic `void*` imzası) etkileniyor.

---

## 9. İstatistiksel Bulgular

> **⚠️ Bu bölümün ana bulgusu n=130'da TERSİNE DÖNMÜŞTÜR.** n=57'de hem
> Mann-Whitney (kod uzunluğu) hem Fisher (işaretçi kullanımı) testi anlamsız
> çıkıyordu; n=130'da her ikisi de anlamlıdır. Aşağıda hem yeni sayılar hem de
> bu tersine dönüşün neden **nedensel bir bulgu değil, veri seti tasarımının
> yarattığı bir karıştırıcı** olduğu açıklanmaktadır.

- **Kod uzunluğu ile başarı ilişkisi (n=130):** Mann-Whitney U testi,
  PASS (n=92) ve FAIL (n=38) gruplarının LoC dağılımları arasında
  istatistiksel olarak **anlamlı** bir fark bulmuştur: **U=924.0, p<0.0001**,
  rank-biserial etki büyüklüğü **r=0.471 (orta etki)**. Farkın yönü
  sezgiye ve literatüre zıttır: **başarısız örnekler daha kısadır**
  (aşağıdaki betimsel tabloya bakınız). n=57'de aynı test anlamsızdı
  (U=287.0, p=0.359, r=0.156).
- **⚠️ Bu anlamlılık nedensel değildir — karıştırıcı değişken (confounder):**
  İlişkinin kaynağı doğrudan veri setinin katman yapısıdır. Kök-neden
  kategorilerini derinleştirmek için eklenen **s58-s84 katmanı kasıtlı olarak
  kısa** programlardan oluşur (16-44 satır) ve büyük çoğunluğu başarısızdır
  (20/27 FAIL); buna karşılık **s85-s130 katmanı uzun** gerçek üretim/
  çeşitlilik kodundan oluşur ve neredeyse tamamı geçer (45/46 PASS).
  Dolayısıyla ölçülen ilişki "kısa kod çevirmek daha zordur" değil,
  **"hataya özel tasarlanmış örneklerimiz kısadır"** demektir. Makalede bu
  sonuç, karıştırıcı açıkça belirtilmeden raporlanmamalıdır; aksi hâlde
  literatürdeki "kod büyüdükçe hata artar" (FLUORINE) savını çürüttüğü
  biçiminde yanlış okunabilir. Doğru ifade: *bu veri setinde LoC ile başarı
  arasında gözlenen ilişki örnekleme tasarımının bir yan ürünüdür ve nedensel
  bir yorum taşımaz.*
- **İşaretçi kullanımı ile başarı ilişkisi (n=130):** Fisher'in kesin testi
  de artık **anlamlıdır**: tablo=[[69, 13], [23, 25]], **olasılık oranı=5.77,
  p<0.0001, %95 GA=[2.54, 13.09]** (aralık 1.0'ı içermiyor). İşaretçi
  kullanan kod belirgin biçimde daha çok geçmektedir (PASS'te %75.0, FAIL'de
  %34.2). **Bu da aynı karıştırıcının bir yansımasıdır:** işaretçi kullanan
  kod ağırlıklı olarak uzun gerçek üretim kodudur (libc fonksiyonları, veri
  yapıları), hedefli kısa örnekler ise nadiren işaretçi kullanır. n=57'de bu
  test anlamsızdı (odds=1.93, p=0.385).
  > **Not (`results/stats_report.md`'de düzeltilmesi gereken bir tutarsızlık):**
  > `stats_report.md`'deki Fisher bölümünde yer alan "Güven aralığının 1.0'i
  > içermesi, ilişkinin istatistiksel olarak anlamlı olmadığını doğrular"
  > cümlesi ile duyarlılık analizindeki "Gözlemlenen r=0.156" ifadesi,
  > n=57'den kalan **eskimiş şablon metinlerdir** ve güncel sayılarla
  > çelişmektedir (GA=[2.54, 13.09] 1.0'ı içermez; gözlenen r=0.471'dir).
  > Bu cümleler `harness/stats_report.py` içinde sabit metin olarak
  > gömülüdür ve düzeltilmelidir.
- **EA bootstrap %95 güven aralıkları (n=130, sabit seed=42, 5000 tekrar):**
  Round 1 debug %70.77 [%63.08, %78.46]; Round 1 release %74.62
  [%66.92, %82.31]; Round 2 %100.00 [%100.00, %100.00]. Genişletme,
  aralıkları n=57'ye göre belirgin biçimde daraltmıştır (n=57'de Round 1
  aralığı yaklaşık ±13 puandı, şimdi ±7.7 puan) — örneklem büyütmenin
  amaçlanan asıl kazanımı budur.
- **Güç ve duyarlılık analizi (n=130):** Bootstrap-tabanlı gerçekleşen güç
  artık **%98.9**'dur (n=57'de %15.0 idi). Ancak bu ölçüt p-değerinin
  tekdüze bir dönüşümüdür ve bağımsız bilgi taşımaz (Hoenig & Heisey 2001,
  "The Abuse of Power"). Bunun yerine hesaplanan duyarlılık analizi:
  n(FAIL)=38, n(PASS)=92, α=0.05 ile %80 güçte saptanabilecek en küçük etki
  büyüklüğü, anlamlılığı ölçmek için kullanılan AYNI Mann-Whitney U
  istatistiğinden ampirik rank-biserial formülüyle (r=1−2U/(n1·n2)) 
  hesaplandığında **|r|≈0.31**'dir (n=57'de |r|≈0.46 idi). Gözlemlenen
  r=0.471 bu eşiğin **üzerindedir** — yani veri seti artık bu büyüklükteki
  etkileri saptayacak güce sahiptir. Örneklem büyütmenin istatistiksel güç
  açısından hedefine ulaştığı nokta budur.
- **p-değerinin istikrarsızlığı (tarihsel kayıt):** Veri seti
  n=36→39→45→48→53→57'ye büyüdükçe p-değeri 0.076 → 0.187 → 0.169 → 0.273 →
  0.337 → 0.359 olarak dalgalanmış, ardından n=130'da <0.0001'e düşmüştür.
  Bu dizi, küçük örneklemlerde p-değerinin ne denli oynak olabileceğinin
  doğrudan, kendi verimizden bir kanıtıdır — ve son sıçrama, örneklem
  bileşimi değiştiğinde p-değerinin ne kadar sert biçimde değişebileceğini
  gösterir.

**Betimsel kod özellikleri (n=130, `harness/stats_report.py` ile yeniden
hesaplandı):**

| Özellik | PASS (n=92) | FAIL (n=38) |
|---|---|---|
| Ortalama LoC | 64.9 | 43.3 |
| Medyan LoC | 53.0 | 25.0 |
| İşaretçi kullanımı | %75.0 | %34.2 |
| malloc/calloc | %17.4 | %0.0 |
| String fonksiyonu | %51.1 | %42.1 |

FAIL grubunda **hiç** `malloc`/`calloc` kullanılmaması (%0.0 vs PASS'te
%17.4) n=57'deki gözlemin n=130'da da korunduğunu gösterir — dinamik
bellek yönetimi arıza riskini artırmıyor, aksine (yine karıştırıcı
etkisiyle) düşürüyor gibi görünüyor.

**Model karşılaştırması:**

- **McNemar testi (Claude vs Gemini) — NİHAİ:** `harness/stats_report.py`
  çıktısına göre ortak değerlendirilen **130 örneğin tamamı** üzerinden:
  ikisi de PASS 86, ikisi de FAIL 8, yalnızca Claude FAIL 30, yalnızca
  Gemini FAIL 6; McNemar kesin (binom tabanlı) iki-yönlü **p=0.0001**. İki
  modelin genel EA farkı istatistiksel olarak anlamlıdır. Her iki model de
  aynı 130 program üzerinde ölçüldüğü için eşleştirilmiş (paired) tasarıma
  uygun olan test budur; bağımsız iki örneklem testi (ör. ki-kare) burada
  uygun değildir. §7.7'deki kategori kırılımı ayrıca bu farkın yönünün
  kategoriye göre değiştiğini göstermektedir.

---

## 10. Ekstra Analiz: Zaman İçinde Bulguların Evrimi

Veri seti kademeli olarak genişletildikçe (24→29→36→39→45→48→53→57→130
örnek), kök neden kategorilerinin **4'ü (F, G, H, I) başlangıçta hiç
öngörülmemiş, sonradan ortaya çıkan bulgulardır**:

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
| s58-s84 | 27 kök-neden derinleştirme (kategori başına 3'er) | **Yeni kategori YOK.** Kategori E'nin aslında sistematik olmadığı ortaya çıktı (0/3 tetiklenme); C/D/F/G/H %100 tetiklendi |
| s85-s109 | 25 gerçek açık kaynak üretim kodu | **Yeni kategori YOK.** Yalnızca 1 başarısızlık (s103), o da mevcut kategori F'ye düştü |
| s110-s130 | 21 çeşitlilik/çok dosyalı/eşzamanlılık | **Yeni kategori YOK.** 21/21 PASS; eşzamanlılık örnekleri 4/4 PASS |

Bu tablo iki ayrı dönemi gösterir. **İlk dönemde (24→57)** veri seti
büyüdükçe önceden tahmin edilemeyen yeni hata modları ortaya çıkmaya devam
etti — 9 kategoriden 4'ü tasarım aşamasında yoktu. **İkinci dönemde
(57→130)** ise, veri seti iki katından fazlasına çıkmasına ve tamamen yeni
kod tabanları (SQLite, zlib, curl, OpenSSL, libsodium, nginx, Apache) ile
tamamen yeni program sınıfları (gerçek eşzamanlılık, `setjmp`/`longjmp`,
değişken argümanlı fonksiyonlar, AVL/trie/graf algoritmaları) eklenmesine
rağmen **onuncu bir kök neden ortaya çıkmadı.**

**Bu, çalışmanın en önemli yapısal bulgularından biridir:** taksonomi, bu kod
sınıfı (tek dosyaya indirgenebilir, stdin/stdout sözleşmeli C programları)
için **doyuma (saturation) ulaşmış görünmektedir.** Aynı dönemde ikinci bir
düzeltme de gerekmiştir: kategori E, üç yeni örneğinin de geçmesiyle
sistematik bir C↔Rust boşluğu olmaktan çıkıp **modelin tutarsızlığına bağlı
bir istisna** olarak yeniden sınıflandırılmalıdır (§2.18.E).

---

## 11. Genel Çıkarımlar

1. **Sessiz hata oranı çok yüksek:** 38 başarısızlığın 37'si (%97.4)
   derleyiciden hiçbir uyarı almadan geçti — yalnızca derleme başarısına
   dayanan bir kabul kriteri bu hataların hiçbirini yakalayamaz. İki
   katmanlı okuma: 28'i (%73.7) gerçekten sessizdir (FE), 9'u (%23.7)
   gürültülü RE'dir, yalnızca 1'i (%2.6) CE'dir.
2. **Tek bir örüntü, dokuz görünümü açıklıyor:** Sekiz kategoride model,
   C kaynağının gerçek bit-düzeyi/kontrol-akışı sözleşmesini korumak yerine
   Rust'ın "daha temiz" görünen varsayılanını seçiyor.
3. **Taksonomi doyuma ulaştı — ve bir kategori yeniden sınıflandırılmalı:**
   Veri setinin 57'den 130'a çıkarılması, tamamen yeni kod tabanları ve
   program sınıfları eklenmesine rağmen **onuncu bir kök neden ortaya
   çıkarmadı** (§10). Buna karşılık kategori E (güvensiz global durum),
   üç yeni örneğinin de geçmesiyle sistematik bir boşluk olmaktan çıkıp
   **modelin tutarsızlığına bağlı bir istisna** olarak yeniden
   yorumlanmalıdır (5 örnekte 4 PASS, §2.18.E).
4. **Düzeltmeler kırılgan olabilir — kanıt artık 6 örnekte:** Aynı düzeltme
   (`i32` kullanımı) bir platformda (Windows) doğru, başka bir platformda
   (Linux) yanlış olabilir; bu artık kategori F'nin **altı örneğinin
   tamamında** doğrulanmıştır ve bunlardan biri (s103_nginx_hextoi)
   hedeflenmemiş gerçek üretim kodundan kendiliğinden gelmiştir. Ayrıca bir
   düzeltmenin dar test girdileriyle doğrulanması (s15→s27 vakası) bile
   yetersiz kalabilir.
5. **Derleme modu hatayı sessizleştirebilir:** Release modunda kategori
   G'nin iki örneği (s76, s77) PASS'e değil **FE'ye** dönüşür — gürültülü
   bir çökme sessiz bir yanlış sonuca dönüşür. Yani release modu yalnızca
   "hatayı gizlemez", bazı durumlarda **riski artırır** (§2.18.G).
6. **Geri bildirim zenginliği belirleyici:** Hataların düzeltilme oranı,
   verilen bilginin ayrıntısına göre %100'den %71.9'a kadar değişebiliyor
   (n=57 kör protokolü) — gerçek bir CI/CD ortamındaki geri bildirim
   kalitesi, iyileştirme döngüsünün pratikteki etkinliğini doğrudan
   belirler. n=130'da da aynı yönlü düşüş gözlenir (%100→%93.85→%87.69);
   kör protokol artık n=130'un tamamında gerçekten uygulanmıştır, bu
   sayılar gerçek ölçümdür (§5, düzeltme notu).
7. **`unsafe` kullanımı isteğe bağlı değil, sözleşmeye bağlı — ve doğru
   kullanımı bir başarı göstergesidir:** Model, C kodunun kendisi
   gerektirmediği sürece hiç `unsafe` kullanmıyor (130 çeviriden yalnızca
   7'si); karmaşık bellek düzenlerini ve pointer-yoğun libc fonksiyonlarını
   bile güvenli soyutlamalarla yeniden yapılandırabiliyor. Kategori E'de ise
   asıl anomali `unsafe`'in **kullanılmadığı** tek vakadır (s19) — diğer
   dördünde model `unsafe`'i doğru kullandığı için başarılı olmuştur (§8).
8. **Bulgular tek modele özgü olabilir; "en iyi model" tek boyutlu bir soru
   değildir:** Üç modelli karşılaştırma, bazı hataların modeller arası ortak
   (Kategori D — altı örnek, dört kod tabanı, iki model boyunca doğrulanmış
   tek gerçekten model-bağımsız boşluk, §7.4/§7.6a), bazılarının modele özgü
   (Gemini'nin E0499 ödünç hatası — §7.3) olduğunu gösteriyor. Daha da
   önemlisi **hata profili EA'dan daha belirleyicidir:** Gemini daha yüksek
   ham doğruluk gösterse de hatalarının %69'u derlemede yakalanır, Claude
   Sonnet 5'in hatalarının ise yalnızca %2.6'sı — bir CI hattında bu iki
   profilin pratik riski taban tabana zıttır. Aynı aileden Claude Haiku
   bile (%72.31 EA, Sonnet 5'e çok yakın) tamamen farklı bir hata profili
   (8 CE, temel algoritmalarda hatalar) sergilemektedir.
9. **İstatistiksel sonuçlar örnekleme tasarımına duyarlıdır:** n=130'da hem
   Mann-Whitney (kod uzunluğu) hem Fisher (işaretçi kullanımı) testi anlamlı
   hale gelmiştir, ancak bu **nedensel bir bulgu değil, veri setinin katman
   yapısının yarattığı bir karıştırıcıdır** (kısa hedefli örnekler çoğunlukla
   başarısız, uzun gerçek kod örnekleri neredeyse tamamen başarılı). Bu tür
   kıyaslama veri setlerinde istatistiksel ilişkiler, örneklerin nasıl
   seçildiği açıkça belirtilmeden yorumlanmamalıdır (§9).

---

*Kaynaklar: `results/results_round1.json`, `results/results_round2*.json`,
`results/results_gemini.json`, `results/results_haiku.json`,
`results/results_round1_linux.json`, `results/platform_comparison.md`,
`results/model_comparison.md`, `results/stats_report.md`,
`results/VERISETI_VE_ALGORITMALAR.md` (örnek bazında tam katalog),
`translations_rust_refined/*.rs` (IYILESTIRME yorumları),
`translations_rust__gemini/*.rs`, `MODIFICATIONS.md`.*

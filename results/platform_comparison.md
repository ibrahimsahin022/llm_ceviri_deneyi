# Platform Karsilastirmasi: Windows (LLP64) vs Linux/Docker (LP64)

Windows: MSYS2/UCRT64 gcc 16.1.0 + rustup rustc 1.97.1 (stable), `long`=32-bit (LLP64).
Linux: Docker `ubuntu:24.04`, apt gcc 13.3.0 (Ubuntu 13.3.0-6ubuntu2~24.04.1) + rustup rustc 1.97.1 (stable, ayni surum/commit ile Windows), `long`=64-bit (LP64). rustc surumunun birebir ayni olmasi, gozlenen tum farkin C tarafindaki `long` genisligi ve stdio metin-modu davranisindan kaynaklandigini, Rust derleyici surumunden kaynaklanmadigini netlestirir.

## Ozet Tablo

| Kosul | Windows EA | Linux EA | Fark |
|---|---|---|---|
| Round 1 - dogrudan, debug | %70.91 (39/55) | %72.73 (40/55) | +1.82 puan |
| Round 1 - dogrudan, release | %74.55 (41/55) | %76.36 (42/55) | +1.81 puan |
| Round 2 - iyilestirilmis, debug | %100.00 (55/55) | %94.55 (52/55) | -5.45 puan |

## Platforma Gore Farklilik Gosteren Ornekler

### Round 1 - dogrudan, debug
| Ornek | Windows | Linux |
|---|---|---|
| s38_bsd_strtol | functional_error | pass |
| s47_redis_sds | pass | functional_error |
| s51_long_clamp | functional_error | pass |

### Round 1 - dogrudan, release
| Ornek | Windows | Linux |
|---|---|---|
| s38_bsd_strtol | functional_error | pass |
| s47_redis_sds | pass | functional_error |
| s51_long_clamp | functional_error | pass |

### Round 2 - iyilestirilmis, debug
| Ornek | Windows | Linux |
|---|---|---|
| s38_bsd_strtol | pass | functional_error |
| s47_redis_sds | pass | functional_error |
| s51_long_clamp | pass | functional_error |

## Yorum

**s38_bsd_strtol ve s51_long_clamp (Round 2, 'duzeltilmis'):** Bu iki orneğin Round 2 duzeltmesi (Faz oncesi Windows'ta %100 EA icin yapilan) `i32` kullanarak Windows'un 32-bit `long`'unu taklit ediyordu. Linux'ta C referansinin `long`'u gercekten 64-bit oldugundan, ayni 'duzeltilmis' Rust kodu artik yanlis sonuc uretiyor (32-bit sinirinda gereksiz yere kirpiyor). Bu, Round 2'nin '%100 basari' rakaminin platforma ozgu oldugunu, evrensel bir duzeltme olmadigini dogrudan kanitlar - bu calismanin en onemli yeni bulgularindan biridir.

**s47_redis_sds (Round 2, test 05 - beklenmedik):** Bu ornekte farkliligin nedeni `long` genisligi degildir. Kok neden: `tests/s47_redis_sds/05.txt` dosyasi CRLF (\r\n) satir sonlari icermektedir (muhtemelen dosyanin olusturuldugu ortamin bir artefakti). C referansindaki `main()`, `scanf("%d", &ncmd)` sonrasi yalnizca tek bir `getchar()` ile satir sonunu tuketiyor. Windows'un C calisma zamani (MSYS2/UCRT), stdin'i metin modunda acip \r\n dizisini otomatik olarak \n'e cevirir (klasik Windows CRT davranisi) - bu yuzden tek `getchar()` yeterli olur. Linux/glibc ise POSIX standardina uygun olarak boyle bir donusum yapmaz; \r karakteri stdin'de oldugu gibi kalir ve tek `getchar()` yalnizca \r'yi tuketir, ardindan ilk `fgets()` cagrisi kalan `\n`'i bos bir satir olarak okur - bu da komut sayacini bir eksik tuketip son komutun (CAT END) hic calismamasina yol acar. Rust cevirisi `BufRead::lines()` kullandigindan (hem \n hem \r\n'i sorunsuz isler) bu sorunu hic yasamaz - yani burada asil 'kirilan' taraf C referansidir, Rust cevirisi degil. Bu, kendi basina onemli bir bulgudur: **ayni C kaynak kodu ve ayni girdi, yalnizca stdio'nun metin-modu satir-sonu davranisi farkli oldugu icin iki platformda farkli sonuc uretebilir** - bu, calismanin C<->Rust semantik boslugu odaginin otesinde, C'nin kendi icinde de platformlar arasi tam taşınabilir olmadigini gosteren ayri, ilginc bir gozlemdir. Test dosyasi kasitli olarak duzeltilmemistir (CRLF oldugu gibi birakilmistir) cunku bu, gercek ve tekrarlanabilir bir bulgudur.

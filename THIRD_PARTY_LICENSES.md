# Üçüncü Taraf Kaynak Kod ve Lisanslar

Bu veri setindeki 13 örnek (s30-s39, s46-s48), tarafımızca yazılmamış,
bağımsız açık kaynak koddan alınmıştır (bkz. makale §III-A, Tablo I).
Her durumda çekirdek algoritma kaynaktan **değiştirilmeden** alınmış,
yalnızca `main()` bu deney ortamının stdin/stdout sözleşmesine uyacak
şekilde yeniden yazılmıştır. Orijinal telif/lisans başlıkları ilgili
`.c` dosyalarının başında olduğu gibi korunmuştur — bu dosya, o bilgilerin
tek bir yerde toplanmış bir dökümüdür.

## Rosetta Code (s30-s36) — GFDL 1.2 / CC-BY-SA

Kaynak: https://rosettacode.org — GNU Free Documentation License 1.2 /
Creative Commons Attribution-ShareAlike, atıfla yeniden kullanıma açıktır.
Ayna depo: https://github.com/acmeism/RosettaCodeData

| Örnek | Sayfa |
|---|---|
| `samples_c/s30_luhn_check.c` | rosettacode.org/wiki/Luhn_test_of_credit_card_numbers |
| `samples_c/s31_soundex.c` | rosettacode.org/wiki/Soundex |
| `samples_c/s32_levenshtein.c` | rosettacode.org/wiki/Levenshtein_distance |
| `samples_c/s33_knapsack.c` | rosettacode.org/wiki/Knapsack_problem/0-1 |
| `samples_c/s34_hanoi.c` | rosettacode.org/wiki/Towers_of_Hanoi |
| `samples_c/s35_lcs.c` | rosettacode.org/wiki/Longest_common_subsequence |
| `samples_c/s36_crc32.c` | rosettacode.org/wiki/CRC-32 |

## OpenBSD/FreeBSD libc (s37-s39) — BSD-3-Clause

Telif: The Regents of the University of California. Kaynak:
https://github.com/openbsd/src , https://github.com/freebsd/freebsd-src

| Örnek | Dosya | Telif satırı |
|---|---|---|
| `samples_c/s37_bsd_getopt.c` | `lib/libc/stdlib/getopt.c` | Copyright (c) 1987, 1993, 1994 The Regents of the University of California. |
| `samples_c/s38_bsd_strtol.c` | `lib/libc/stdlib/strtol.c` | Copyright (c) 1990 The Regents of the University of California. |
| `samples_c/s39_bsd_heapsort.c` | `lib/libc/stdlib/heapsort.c` | Copyright (c) 1991, 1993 The Regents of the University of California. |

BSD-3-Clause tam metni: https://opensource.org/license/bsd-3-clause

## musl libc (s46) — MIT

`samples_c/s46_musl_qsort.c` — kaynak: `src/stdlib/qsort.c` (smoothsort /
`__qsort_r`), https://github.com/kraj/musl — Copyright (C) 2011 by Lynn
Ochs; musl'ın bütünü MIT lisanslıdır (bkz.
https://git.musl-libc.org/cgit/musl/tree/COPYRIGHT).

## Redis SDS (s47) — BSD-3-Clause

`samples_c/s47_redis_sds.c` — kaynak: Redis 7.2.4, `src/sds.c` / `src/sds.h`
(SDSLib 2.0), https://github.com/redis/redis (tag `7.2.4`) — Copyright (c)
2006-2015 Salvatore Sanfilippo ve diğerleri. Not: Redis 8.0+ lisansını
değiştirmiştir (RSALv2/SSPL/AGPL); bu örnek özellikle BSD-3-Clause'un hâlâ
geçerli olduğu 7.2.4 etiketinden alınmıştır.

## cJSON (s48) — MIT

`samples_c/s48_cjson_number.c` — kaynak: `cJSON.c` içindeki `parse_number()`
ve `print_number()` fonksiyonları, https://github.com/DaveGamble/cJSON —
Copyright (c) 2009-2017 Dave Gamble and cJSON contributors.

## LLM çevirileri ve harness kodu

Yukarıdaki 13 örneğin **Rust çevirileri** (`translations_rust/`,
`translations_rust__gemini/` vb.) LLM'ler (Claude Sonnet 5, Google Gemini)
tarafından türetilmiştir; bu çeviriler ile projenin geri kalan tüm özgün
kodu (`harness/`, özgün `samples_c/` örnekleri, betikler) `LICENSE`
dosyasındaki MIT lisansı altındadır.

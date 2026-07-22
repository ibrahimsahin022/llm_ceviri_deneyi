#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Windows (LLP64, 32-bit long) sonuclariyla Linux/Docker (LP64, 64-bit long)
konteynerinde uretilen sonuclari karsilastirir. Faz 4 (cok platformlu
calistirma) icin.

Onkosul: her iki platformda da run_experiment.py calistirilmis olmali:
  Windows: python harness/run_experiment.py --label round1
           python harness/run_experiment.py --release --label round1_release
           python harness/run_experiment.py --rust-dir translations_rust_refined --label round2
  Linux (docker compose run --rm experiment-linux, bkz. docker-compose.yml):
           ayni uc komut ama --label'lar "_linux" son ekiyle
           (round1_linux, round1_release_linux, round2_linux)

Kullanim:
  python3 harness/compare_platforms.py
"""
import json
import sys
from pathlib import Path

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent

PAIRS = [
    ("round1", "round1_linux", "Round 1 - dogrudan, debug"),
    ("round1_release", "round1_release_linux", "Round 1 - dogrudan, release"),
    ("round2", "round2_linux", "Round 2 - iyilestirilmis, debug"),
]


def load(label):
    p = ROOT / "results" / f"results_{label}.json"
    if not p.exists():
        return None
    return json.loads(p.read_text(encoding="utf-8"))


def main():
    lines = ["# Platform Karsilastirmasi: Windows (LLP64) vs Linux/Docker (LP64)\n"]
    lines.append(
        "Windows: MSYS2/UCRT64 gcc 16.1.0 + rustup rustc 1.97.1 (stable), "
        "`long`=32-bit (LLP64).\n"
        "Linux: Docker `ubuntu:24.04`, apt gcc 13.3.0 (Ubuntu "
        "13.3.0-6ubuntu2~24.04.1) + rustup rustc 1.97.1 (stable, ayni "
        "surum/commit ile Windows), `long`=64-bit (LP64). rustc surumunun "
        "birebir ayni olmasi, gozlenen tum farkin C tarafindaki `long` "
        "genisligi ve stdio metin-modu davranisindan kaynaklandigini, "
        "Rust derleyici surumunden kaynaklanmadigini netlestirir.\n"
    )

    lines.append("## Ozet Tablo\n")
    lines.append("| Kosul | Windows EA | Linux EA | Fark |")
    lines.append("|---|---|---|---|")
    all_diffs = {}
    for win_label, linux_label, name in PAIRS:
        dw = load(win_label)
        dl = load(linux_label)
        if not dw or not dl:
            lines.append(f"| {name} | - | - | (eksik veri) |")
            continue
        ew, el = dw["execution_accuracy_pct"], dl["execution_accuracy_pct"]
        lines.append(f"| {name} | %{ew:.2f} ({dw['passed_samples']}/{dw['total_samples']}) "
                     f"| %{el:.2f} ({dl['passed_samples']}/{dl['total_samples']}) "
                     f"| {el - ew:+.2f} puan |")

        win_by_id = {r["id"]: r["category"] for r in dw["results"]}
        linux_by_id = {r["id"]: r["category"] for r in dl["results"]}
        for sid in win_by_id:
            wc, lc = win_by_id[sid], linux_by_id.get(sid)
            if wc != lc:
                all_diffs.setdefault(name, []).append((sid, wc, lc))

    lines.append("")
    lines.append("## Platforma Gore Farklilik Gosteren Ornekler\n")
    if not all_diffs:
        lines.append("(Hic farklilik bulunamadi.)\n")
    for name, diffs in all_diffs.items():
        lines.append(f"### {name}")
        lines.append("| Ornek | Windows | Linux |")
        lines.append("|---|---|---|")
        for sid, wc, lc in diffs:
            lines.append(f"| {sid} | {wc} | {lc} |")
        lines.append("")

    lines.append(
        "## Yorum\n\n"
        "**s38_bsd_strtol ve s51_long_clamp (Round 2, 'duzeltilmis'):** Bu iki "
        "orneğin Round 2 duzeltmesi (Faz oncesi Windows'ta %100 EA icin "
        "yapilan) `i32` kullanarak Windows'un 32-bit `long`'unu taklit "
        "ediyordu. Linux'ta C referansinin `long`'u gercekten 64-bit "
        "oldugundan, ayni 'duzeltilmis' Rust kodu artik yanlis sonuc "
        "uretiyor (32-bit sinirinda gereksiz yere kirpiyor). Bu, Round "
        "2'nin '%100 basari' rakaminin platforma ozgu oldugunu, evrensel "
        "bir duzeltme olmadigini dogrudan kanitlar - bu calismanin en onemli "
        "yeni bulgularindan biridir.\n\n"
        "**s47_redis_sds (Round 2, test 05 - beklenmedik):** Bu ornekte "
        "farkliligin nedeni `long` genisligi degildir. Kok neden: "
        "`tests/s47_redis_sds/05.txt` dosyasi CRLF (\\r\\n) satir sonlari "
        "icermektedir (muhtemelen dosyanin olusturuldugu ortamin bir "
        "artefakti). C referansindaki `main()`, `scanf(\"%d\", &ncmd)` "
        "sonrasi yalnizca tek bir `getchar()` ile satir sonunu tuketiyor. "
        "Windows'un C calisma zamani (MSYS2/UCRT), stdin'i metin modunda "
        "acip \\r\\n dizisini otomatik olarak \\n'e cevirir (klasik Windows "
        "CRT davranisi) - bu yuzden tek `getchar()` yeterli olur. Linux/"
        "glibc ise POSIX standardina uygun olarak boyle bir donusum "
        "yapmaz; \\r karakteri stdin'de oldugu gibi kalir ve tek "
        "`getchar()` yalnizca \\r'yi tuketir, ardindan ilk `fgets()` cagrisi "
        "kalan `\\n`'i bos bir satir olarak okur - bu da komut sayacini "
        "bir eksik tuketip son komutun (CAT END) hic calismamasina yol "
        "acar. Rust cevirisi `BufRead::lines()` kullandigindan (hem \\n hem "
        "\\r\\n'i sorunsuz isler) bu sorunu hic yasamaz - yani burada asil "
        "'kirilan' taraf C referansidir, Rust cevirisi degil. Bu, "
        "kendi basina onemli bir bulgudur: **ayni C kaynak kodu ve ayni "
        "girdi, yalnizca stdio'nun metin-modu satir-sonu davranisi "
        "farkli oldugu icin iki platformda farkli sonuc uretebilir** - "
        "bu, calismanin C<->Rust semantik boslugu odaginin otesinde, "
        "C'nin kendi icinde de platformlar arasi tam taşınabilir "
        "olmadigini gosteren ayri, ilginc bir gozlemdir. Test dosyasi "
        "kasitli olarak duzeltilmemistir (CRLF oldugu gibi birakilmistir) "
        "cunku bu, gercek ve tekrarlanabilir bir bulgudur.\n"
    )

    out_path = ROOT / "results" / "platform_comparison.md"
    out_path.write_text("\n".join(lines), encoding="utf-8")
    print("\n".join(lines))
    print("\nYazildi:", out_path.relative_to(ROOT))


if __name__ == "__main__":
    main()

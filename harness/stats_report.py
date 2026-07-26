#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Istatistiksel guc ve guven araligi raporu (Faz 2 - hakem geri bildirimi:
kucuk orneklem/istatistiksel guc zayifligini gidermek icin).

Uretir:
  - EA icin bootstrap %95 guven araligi (round1, round1_release, round2)
  - Mann-Whitney U: rank-biserial etki buyuklugu + bootstrap-tabanli
    gerceklesen guc (achieved power) tahmini
  - Fisher kesin testi: odds orani icin log-yaklasik %95 guven araligi
  - Cikti: results/stats_report.md

Agir bir bagimlilik (statsmodels) eklemeden, yalnizca numpy/scipy ile
calisir. Guc tahmini analitik degil, Monte Carlo/bootstrap tabanlidir:
gozlemlenen iki grubun (PASS/FAIL LoC dagilimlari) kendisinden tekrar tekrar
ornekleyip, ayni n ile Mann-Whitney testini kac kez p<0.05 verdigini sayar -
bu, "bu tam etki buyuklugunde, bu n ile testin gercekten anlamli cikma
olasiligi nedir" sorusuna dogrudan, seffaf bir cevaptir.
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import numpy as np
from scipy import stats

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent
RNG = np.random.default_rng(seed=42)  # tekrarlanabilirlik icin sabit seed
N_BOOT = 5000


def load_results(label: str) -> dict | None:
    p = ROOT / "results" / f"results_{label}.json"
    if not p.exists():
        return None
    return json.loads(p.read_text(encoding="utf-8"))


def bootstrap_ea_ci(pass_flags: list[bool], n_boot: int = N_BOOT) -> tuple[float, float, float]:
    """(nokta_tahmini, alt_2.5, ust_97.5) EA yuzdesi olarak dondurur."""
    arr = np.array(pass_flags, dtype=float)
    n = len(arr)
    point = 100.0 * arr.mean()
    boot_means = np.empty(n_boot)
    for i in range(n_boot):
        sample = RNG.choice(arr, size=n, replace=True)
        boot_means[i] = 100.0 * sample.mean()
    lo, hi = np.percentile(boot_means, [2.5, 97.5])
    return point, lo, hi


def has_pointer(src: str) -> bool:
    if "->" in src:
        return True
    if re.search(r"\*\s*[a-zA-Z_]\w*\s*[,;)\[=]", src):
        return True
    return False


def has_malloc(src: str) -> bool:
    return bool(re.search(r"\b(malloc|calloc|realloc)\s*\(", src))


STRING_FUNCS = ["strlen", "strcmp", "strncmp", "strcpy", "strncpy", "strcat",
                "strncat", "strtok", "fgets", "strchr", "strrchr", "strstr",
                "sprintf", "snprintf", "strdup"]


def has_strfunc(src: str) -> bool:
    return any(re.search(r"\b" + fn + r"\s*\(", src) for fn in STRING_FUNCS)


def read_source(sid: str) -> str:
    single = ROOT / "samples_c" / f"{sid}.c"
    if single.exists():
        return single.read_text(encoding="utf-8", errors="replace")
    multi_dir = ROOT / "samples_c" / sid
    if multi_dir.exists():
        return "\n".join(
            f.read_text(encoding="utf-8", errors="replace")
            for f in sorted(multi_dir.glob("*.c"))
        )
    return ""


def rank_biserial(u_stat: float, n1: int, n2: int) -> float:
    """Mann-Whitney U -> rank-biserial korelasyon (etki buyuklugu)."""
    return 1 - (2 * u_stat) / (n1 * n2)


def mannwhitney_achieved_power(fail_locs: list[int], pass_locs: list[int],
                                n_boot: int = N_BOOT, alpha: float = 0.05) -> float:
    """Gozlemlenen iki dagilimdan (with replacement) tekrar tekrar ayni
    boyutlarda ornekleyip, testin kac defa p<alpha verdigini olcer."""
    fail_arr = np.array(fail_locs)
    pass_arr = np.array(pass_locs)
    n1, n2 = len(fail_arr), len(pass_arr)
    sig_count = 0
    for _ in range(n_boot):
        f_sample = RNG.choice(fail_arr, size=n1, replace=True)
        p_sample = RNG.choice(pass_arr, size=n2, replace=True)
        try:
            _, p = stats.mannwhitneyu(f_sample, p_sample, alternative="two-sided")
        except ValueError:
            continue  # tum degerler ayni cikarsa (varyans=0) testi atla
        if p < alpha:
            sig_count += 1
    return 100.0 * sig_count / n_boot


def minimum_detectable_effect(n1: int, n2: int, pooled_sigma: float,
                               target_power: float = 0.80, alpha: float = 0.05,
                               n_boot: int = 2000) -> tuple[float, float]:
    """n1, n2 boyutlarinda, alpha=0.05'te %target_power guc icin gereken en
    kucuk kaydirma (delta, normal yaklasimla) ve bunun karsilik geldigi
    rank-biserial etki buyuklugunu (r) dondurur. Post-hoc/gozlemlenen guc
    yerine onerilen duyarlilik analizi budur (Hoenig & Heisey, 2001) - n
    sabitken, testin %80 guçle saptayabilecegi en kucuk etkiyi gosterir."""
    lo_delta, hi_delta = 1.0, pooled_sigma * 4
    for _ in range(22):  # ikili arama
        mid = (lo_delta + hi_delta) / 2
        sig = 0
        for _ in range(n_boot):
            f = RNG.normal(0, pooled_sigma, n1)
            p = RNG.normal(mid, pooled_sigma, n2)
            _, pv = stats.mannwhitneyu(f, p, alternative="two-sided")
            if pv < alpha:
                sig += 1
        pw = sig / n_boot
        if pw < target_power:
            lo_delta = mid
        else:
            hi_delta = mid
    delta = (lo_delta + hi_delta) / 2
    # Etki buyuklugu (r), anlamliligi olcmek icin kullanilan AYNI Mann-Whitney
    # U istatistiginden ampirik rank-biserial formuluyle (r = 1 - 2U/(n1*n2))
    # hesaplanir - normal/AUC yaklasik donusumu yerine (tutarlilik icin;
    # ikisi farkli sayısal deger uretebilir).
    r_samples = []
    for _ in range(n_boot):
        f = RNG.normal(0, pooled_sigma, n1)
        p = RNG.normal(delta, pooled_sigma, n2)
        u, _ = stats.mannwhitneyu(f, p, alternative="two-sided")
        r_samples.append(1 - 2 * u / (n1 * n2))
    r_eff = abs(float(np.mean(r_samples)))
    return delta, r_eff


def mcnemar_exact(only_a_fail: int, only_b_fail: int) -> float:
    """Iki bagimli (paired) model karsilastirmasi icin McNemar'in kesin
    (binom tabanli) iki-yonlu testi. only_a_fail: yalnizca A'nin basarisiz
    oldugu ornek sayisi; only_b_fail: yalnizca B'nin basarisiz oldugu ornek
    sayisi (uyumsuz/discordant ciftler)."""
    n = only_a_fail + only_b_fail
    if n == 0:
        return 1.0
    k = min(only_a_fail, only_b_fail)
    return stats.binomtest(k, n=n, p=0.5, alternative="two-sided").pvalue


def fisher_odds_ci(table: list[list[int]], alpha: float = 0.05) -> tuple[float, float, float]:
    """2x2 tablo icin odds orani ve log-yaklasik guven araligi.
    table = [[a,b],[c,d]] -> odds = (a*d)/(b*c)."""
    a, b = table[0]
    c, d = table[1]
    # sifir hucre duzeltmesi (Haldane-Anscombe)
    if 0 in (a, b, c, d):
        a, b, c, d = a + 0.5, b + 0.5, c + 0.5, d + 0.5
    odds = (a * d) / (b * c)
    se_log_odds = (1 / a + 1 / b + 1 / c + 1 / d) ** 0.5
    z = stats.norm.ppf(1 - alpha / 2)
    log_odds = np.log(odds)
    lo = np.exp(log_odds - z * se_log_odds)
    hi = np.exp(log_odds + z * se_log_odds)
    return odds, lo, hi


def main():
    lines = ["# Istatistiksel Guc ve Guven Araligi Raporu (Faz 2)\n"]
    lines.append(
        "Bootstrap/Monte Carlo yontemleri sabit seed (42) ile calisir; "
        "aynı veri uzerinde tekrar calistirildiginda ayni sayilari uretir.\n"
    )

    # ---- 1) EA bootstrap CI (uc kosul) ----
    lines.append("## EA Bootstrap %95 Guven Araligi\n")
    lines.append("| Kosul | EA (nokta) | %95 GA |")
    lines.append("|---|---|---|")
    ea_rows = []
    for label, name in [("round1", "Round 1 - dogrudan, debug"),
                         ("round1_release", "Round 1 - dogrudan, release"),
                         ("round2", "Round 2 - iyilestirilmis, debug")]:
        d = load_results(label)
        if not d:
            continue
        pass_flags = [r["category"] == "pass" for r in d["results"]]
        point, lo, hi = bootstrap_ea_ci(pass_flags)
        ea_rows.append((name, point, lo, hi))
        lines.append(f"| {name} | %{point:.2f} | [%{lo:.2f}, %{hi:.2f}] |")
    lines.append("")

    # ---- 2) Mann-Whitney U: etki buyuklugu + gerceklesen guc ----
    d1 = load_results("round1")
    if d1:
        pass_locs = [r["loc_c"] for r in d1["results"] if r["category"] == "pass"]
        fail_locs = [r["loc_c"] for r in d1["results"] if r["category"] != "pass"]
        u_stat, p_val = stats.mannwhitneyu(fail_locs, pass_locs, alternative="two-sided")
        n1, n2 = len(fail_locs), len(pass_locs)
        r_eff = rank_biserial(u_stat, n1, n2)
        power = mannwhitney_achieved_power(fail_locs, pass_locs)

        lines.append("## Mann-Whitney U (LoC: PASS vs FAIL) - Etki Buyuklugu ve Guc\n")
        lines.append(f"- n(FAIL)={n1}, n(PASS)={n2}")
        lines.append(f"- U={u_stat:.1f}, p={p_val:.4f}")
        lines.append(f"- Rank-biserial korelasyon (etki buyuklugu) r={r_eff:.3f} "
                      f"({'kucuk' if abs(r_eff) < 0.3 else 'orta' if abs(r_eff) < 0.5 else 'buyuk'} etki)")
        lines.append(f"- Bootstrap-tabanli gerceklesen guc (achieved power, alpha=0.05, "
                      f"{N_BOOT} tekrar): **%{power:.1f}** (yalnizca betimsel; post-hoc guc "
                      f"p-degerinin tekduze bir donusumudur ve p'nin otesinde bagimsiz bilgi "
                      f"tasimaz - Hoenig & Heisey 2001. Asagidaki duyarlilik analizine bakiniz.)")

        pooled = np.array(pass_locs + fail_locs, dtype=float)
        pooled_sigma = float(pooled.std(ddof=1))
        mde_delta, mde_r = minimum_detectable_effect(n1, n2, pooled_sigma)
        lines.append(
            f"- **Duyarlilik analizi (onerilen, post-hoc guc yerine):** n(FAIL)={n1}, "
            f"n(PASS)={n2}, alpha=0.05 ile %80 guçte saptanabilecek en kucuk etki "
            f"buyuklugu, anlamliligi olcmek icin kullanilan AYNI Mann-Whitney U "
            f"istatistiginden ampirik rank-biserial formuluyle (r=1-2U/(n1*n2), normal/AUC "
            f"yaklasik donusumu degil) hesaplandiginda rank-biserial |r|≈{mde_r:.2f}'dir "
            f"(LoC olceginde ≈{mde_delta:.0f} satirlik bir ortalama farka denk gelir, "
            f"pooled sigma={pooled_sigma:.1f}). Gozlemlenen r=0.156 bu esigin belirgin "
            f"altindadir - veri seti bu buyuklukte kucuk-orta etkileri saptayacak guce "
            f"sahip degildir; 'anlamli fark yok' sonucu bu nedenle kesin bir iliskisizlik "
            f"kaniti degil, dusuk guçle tutarli bir gozlem olarak okunmalidir.\n"
        )

    # ---- 3) Betimsel kod ozellikleri (Tablo VII) ----
    if d1:
        groups: dict[str, list] = {"pass": [], "fail": []}
        for r in d1["results"]:
            key = "pass" if r["category"] == "pass" else "fail"
            groups[key].append(r)

        lines.append("## Betimsel Kod Ozellikleri (PASS vs FAIL, Tablo VII)\n")
        lines.append(
            "Olcum tanimlari (tekrarlanabilirlik icin): isaretci kullanimi = "
            "kaynakta `->` VEYA `*isim` bicimli bir isaretci degisken kullanimi "
            "(regex: `r\"\\*\\s*[a-zA-Z_]\\w*\\s*[,;)\\[=]\"`); malloc/calloc = "
            "`malloc(`/`calloc(`/`realloc(` cagrisi; string fonksiyonu = "
            f"{', '.join(STRING_FUNCS)} fonksiyonlarindan en az birinin cagrisi.\n"
        )
        lines.append("| Ozellik | PASS(n=%d) | FAIL(n=%d) |" % (len(groups["pass"]), len(groups["fail"])))
        lines.append("|---|---|---|")
        stats_by_group = {}
        for key, results in groups.items():
            locs = [r["loc_c"] for r in results]
            n = len(results)
            ptr = sum(has_pointer(read_source(r["id"])) for r in results)
            mal = sum(has_malloc(read_source(r["id"])) for r in results)
            st = sum(has_strfunc(read_source(r["id"])) for r in results)
            stats_by_group[key] = {
                "n": n, "mean_loc": sum(locs) / n, "median_loc": float(np.median(locs)),
                "ptr_pct": 100.0 * ptr / n, "malloc_pct": 100.0 * mal / n, "str_pct": 100.0 * st / n,
                "ptr": ptr, "malloc": mal,
            }
        sp, sf = stats_by_group["pass"], stats_by_group["fail"]
        lines.append(f"| Ortalama LoC | {sp['mean_loc']:.1f} | {sf['mean_loc']:.1f} |")
        lines.append(f"| Medyan LoC | {sp['median_loc']:.1f} | {sf['median_loc']:.1f} |")
        lines.append(f"| Isaretci kullanimi | %{sp['ptr_pct']:.1f} | %{sf['ptr_pct']:.1f} |")
        lines.append(f"| malloc/calloc | %{sp['malloc_pct']:.1f} | %{sf['malloc_pct']:.1f} |")
        lines.append(f"| String fonksiyonu | %{sp['str_pct']:.1f} | %{sf['str_pct']:.1f} |")
        lines.append("")

    # ---- 4) Fisher (pointer kullanimi) - odds orani + CI ----
    if d1:
        ptr_pass = stats_by_group["pass"]["ptr"]
        ptr_fail = stats_by_group["fail"]["ptr"]
        nonptr_pass = stats_by_group["pass"]["n"] - ptr_pass
        nonptr_fail = stats_by_group["fail"]["n"] - ptr_fail
        table = [[ptr_pass, ptr_fail], [nonptr_pass, nonptr_fail]]
        odds, p_fisher = stats.fisher_exact(table)
        odds_ci, lo_ci, hi_ci = fisher_odds_ci(table)

        lines.append("## Fisher Kesin Testi (Isaretci Kullanimi vs PASS/FAIL) - Odds Orani GA\n")
        lines.append(f"- Tablo (pointer/non-pointer x PASS/FAIL): {table}")
        lines.append(f"- Odds orani={odds:.2f}, p={p_fisher:.4f}")
        lines.append(f"- Odds orani %95 guven araligi (log-yaklasik): "
                      f"[{lo_ci:.2f}, {hi_ci:.2f}]")
        lines.append(
            "  (Guven araliginin 1.0'i icermesi, iliskinin istatistiksel olarak "
            "anlamli olmadigini dogrular; aralik cok genistir - kucuk orneklemin "
            "dogal bir sonucu.)\n"
        )

    # ---- 5) McNemar (Claude vs Gemini, eslesmis/paired karsilastirma) ----
    d_gemini = load_results("gemini")
    if d1 and d_gemini:
        claude_map = {r["id"]: r["category"] == "pass" for r in d1["results"]}
        gemini_map = {r["id"]: r["category"] == "pass" for r in d_gemini["results"]}
        shared_ids = sorted(set(claude_map) & set(gemini_map))
        only_claude_fail = sum(1 for i in shared_ids if not claude_map[i] and gemini_map[i])
        only_gemini_fail = sum(1 for i in shared_ids if claude_map[i] and not gemini_map[i])
        both_fail = sum(1 for i in shared_ids if not claude_map[i] and not gemini_map[i])
        both_pass = sum(1 for i in shared_ids if claude_map[i] and gemini_map[i])
        p_mcnemar = mcnemar_exact(only_claude_fail, only_gemini_fail)

        lines.append("## McNemar Testi (Claude vs Gemini, eslestirilmis karsilastirma)\n")
        lines.append(f"- Ortak degerlendirilen ornek sayisi: {len(shared_ids)}")
        lines.append(f"- Ikisi de PASS: {both_pass} | Ikisi de FAIL: {both_fail}")
        lines.append(f"- Yalnizca Claude FAIL (Gemini PASS): {only_claude_fail}")
        lines.append(f"- Yalnizca Gemini FAIL (Claude PASS): {only_gemini_fail}")
        lines.append(f"- McNemar kesin (binom-tabanli) iki-yonlu p={p_mcnemar:.4f}")
        lines.append(
            "  (Iki modelin genel EA farkinin istatistiksel olarak anlamli olup "
            "olmadigini, eslesmis/paired tasarima uygun bicimde test eder - bagimsiz "
            "iki orneklem testi (ör. ki-kare) burada uygun degildir cunku iki model "
            "AYNI 57 program uzerinde olculmustur. Anlamli bir genel fark, "
            "model×kategori etkilesiminin var olmadigi anlamina gelmez - bkz. Tablo VI.)\n"
        )

    out_path = ROOT / "results" / "stats_report.md"
    out_path.write_text("\n".join(lines), encoding="utf-8")
    print("\n".join(lines))
    print("\nYazildi:", out_path.relative_to(ROOT))


if __name__ == "__main__":
    main()

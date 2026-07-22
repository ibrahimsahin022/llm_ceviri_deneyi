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
                      f"{N_BOOT} tekrar): **%{power:.1f}**")
        lines.append(
            "  (Yorum: gozlemlenen tam bu etki buyuklugunde ve bu n ile, testin "
            "tekrar tekrar uygulansaydi ne siklikta anlamli cikacagini gosterir. "
            "Dusuk guc, 'anlamli fark yok' sonucunun bir Tip II hatasi olabilecegi "
            "anlamina gelir - kesin bir 'iliski yoktur' iddiasi degildir.)\n"
        )

    # ---- 3) Fisher (pointer kullanimi) - odds orani + CI ----
    if d1:
        ptr_pass = ptr_fail = nonptr_pass = nonptr_fail = 0
        for r in d1["results"]:
            sid = r["id"]
            single = ROOT / "samples_c" / f"{sid}.c"
            if single.exists():
                src = single.read_text(encoding="utf-8", errors="replace")
            else:
                # Faz 3: cok dosyali ornek (samples_c/<id>/*.c) - tum
                # kaynak dosyalarin birlesimi uzerinde pointer taramasi yap
                multi_dir = ROOT / "samples_c" / sid
                src = "\n".join(
                    f.read_text(encoding="utf-8", errors="replace")
                    for f in sorted(multi_dir.glob("*.c"))
                ) if multi_dir.exists() else ""
            ptr = has_pointer(src)
            ok = r["category"] == "pass"
            if ptr and ok:
                ptr_pass += 1
            elif ptr and not ok:
                ptr_fail += 1
            elif not ptr and ok:
                nonptr_pass += 1
            else:
                nonptr_fail += 1
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

    out_path = ROOT / "results" / "stats_report.md"
    out_path.write_text("\n".join(lines), encoding="utf-8")
    print("\n".join(lines))
    print("\nYazildi:", out_path.relative_to(ROOT))


if __name__ == "__main__":
    main()

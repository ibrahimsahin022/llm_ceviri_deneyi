#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Sonuc JSON'larindan makale icin temiz figurleri uretir (results/figures/*.png)."""
import json
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Patch

ROOT = Path(__file__).resolve().parent.parent
RES = ROOT / "results"
FIG = RES / "figures"
FIG.mkdir(exist_ok=True)

plt.rcParams.update({
    "font.family": "DejaVu Sans",
    "font.size": 11,
    "axes.spines.top": False,
    "axes.spines.right": False,
    "axes.grid": True,
    "grid.alpha": 0.18,
    "figure.dpi": 150,
    "axes.titleweight": "bold",
})

# Tutarli, erisilebilir palet
COL = {
    "pass": "#2a9d8f",
    "runtime_error": "#e76f51",
    "functional_error": "#e9c46a",
    "non_termination": "#8d99ae",
    "compilation_error": "#5b3758",
    "bar": "#4c72b0",
}
TR = {
    "pass": "Başarılı (PASS)",
    "runtime_error": "Çalışma Zamanı (RE)",
    "functional_error": "Fonksiyonel (FE)",
    "non_termination": "Sonlanmama (NT)",
    "compilation_error": "Derleme (CE)",
}


def load(label):
    p = RES / f"results_{label}.json"
    return json.load(open(p, encoding="utf-8")) if p.exists() else None


COND = [("round1", "Round 1\ndoğrudan · debug"),
        ("round1_release", "Round 1\ndoğrudan · release"),
        ("round2", "Round 2\niyileştirilmiş · debug")]

# ============ FIGUR 2: Yurutme Dogrulugu (EA) ============
labels, vals = [], []
for lab, name in COND:
    d = load(lab)
    if d:
        labels.append(name)
        vals.append(d["execution_accuracy_pct"])

fig, ax = plt.subplots(figsize=(6.6, 4.2))
bars = ax.bar(labels, vals, color=[COL["bar"], COL["non_termination"], COL["pass"]], width=0.58)
ax.set_ylabel("Yürütme Doğruluğu (EA) %")
ax.set_ylim(0, 112)
ax.set_title("Şekil 2. Koşullara göre yürütme doğruluğu", pad=12)
for b, v in zip(bars, vals):
    ax.text(b.get_x() + b.get_width() / 2, v + 2, f"%{v:.1f}", ha="center",
            fontsize=12, fontweight="bold")
ax.grid(axis="x", alpha=0)
fig.tight_layout()
fig.savefig(FIG / "fig2_execution_accuracy.png", bbox_inches="tight")
plt.close(fig)

# ============ FIGUR 3: Kosullara gore YIGILMIS ornek dagilimi ============
cats = ["pass", "functional_error", "runtime_error", "compilation_error", "non_termination"]
fig, ax = plt.subplots(figsize=(7.4, 4.6))
xpos = range(len([c for c in COND if load(c[0])]))
condnames = [name for lab, name in COND if load(lab)]
bottoms = [0] * len(condnames)
for cat in cats:
    heights = []
    for lab, _ in COND:
        d = load(lab)
        if d:
            heights.append(d["sample_category_counts"][cat])
    ax.bar(condnames, heights, bottom=bottoms, color=COL[cat], label=TR[cat],
           width=0.55, edgecolor="white", linewidth=0.6)
    # etiketler (0 olmayanlar)
    for i, h in enumerate(heights):
        if h > 0:
            ax.text(i, bottoms[i] + h / 2, str(h), ha="center", va="center",
                    fontsize=10, fontweight="bold",
                    color="white" if cat in ("pass", "runtime_error", "compilation_error") else "#333")
    bottoms = [b + h for b, h in zip(bottoms, heights)]
total_samples = load(COND[0][0])["total_samples"]
ax.set_ylabel(f"Örnek sayısı (toplam {total_samples})")
ax.set_ylim(0, total_samples + 2)
ax.set_title("Şekil 3. Koşullara göre sonuç dağılımı", pad=12)
ax.legend(loc="upper center", bbox_to_anchor=(0.5, -0.12), ncol=3, frameon=False, fontsize=9.5)
ax.grid(axis="x", alpha=0)
fig.tight_layout()
fig.savefig(FIG / "fig3_error_distribution.png", bbox_inches="tight")
plt.close(fig)

# ============ FIGUR 4: Kod uzunlugu (LoC) vs sonuc ============
import random
d = load("round1")
xs, ys, cs, names2 = [], [], [], []
for r in d["results"]:
    xs.append(r["loc_c"])
    ok = 1 if r["category"] == "pass" else 0
    ys.append(ok)
    cs.append(COL["pass"] if ok else COL["runtime_error"])
    names2.append(r["id"])

max_loc = max(xs)
rng = random.Random(42)
# hafif dusey jitter: ust uste binen noktalari bir "swarm" gibi ayirir
yj = [y + (0.10 if y == 0 else -0.10) * (i % 5) / 4 + rng.uniform(-0.02, 0.02)
      for i, y in enumerate(ys)]

fig, ax = plt.subplots(figsize=(8.6, 4.8))
# arka plan bantlari: basarili/basarisiz alanlarini gorsel olarak ayirir
ax.axhspan(-0.42, 0.42, color=COL["runtime_error"], alpha=0.06, zorder=0)
ax.axhspan(0.58, 1.42, color=COL["pass"], alpha=0.06, zorder=0)
ax.axhline(0, color="#999", lw=0.6, ls=":", zorder=1)
ax.axhline(1, color="#999", lw=0.6, ls=":", zorder=1)

ax.scatter(xs, yj, c=cs, s=95, zorder=3, edgecolors="white", linewidths=1.3, alpha=0.92)

ax.set_yticks([0, 1])
ax.set_yticklabels(["Başarısız", "Başarılı"], fontsize=11, fontweight="bold")
ax.set_ylim(-0.55, 1.55)
ax.set_xlabel("Kaynak kod uzunluğu (C, satır sayısı)")
ax.set_title("Şekil 4. Kod uzunluğu ile başarı ilişkisi (Round 1)", pad=12)

# yalnizca metinde tartisilan, bilgilendirici noktalar etiketlenir (tumu degil)
highlight = {
    "s47_redis_sds": "en uzun (522 satır) — PASS",
    "s48_cjson_number": "389 satır — FAIL (%g)",
    "s38_bsd_strtol": "154 satır — FAIL (tamsayı genişliği)",
}
for x, y, n in zip(xs, yj, names2):
    if n in highlight:
        ax.annotate(highlight[n], (x, y), textcoords="offset points",
                    xytext=(0, 14 if y >= 0.5 else -18), fontsize=8.5, ha="center",
                    color="#333", fontweight="bold")

ax.set_xlim(-10, max_loc + 20)
ax.grid(axis="y", alpha=0)
ax.grid(axis="x", alpha=0.15)
legend_handles = [
    Patch(facecolor=COL["pass"], label="Başarılı (PASS)"),
    Patch(facecolor=COL["runtime_error"], label="Başarısız (CE/RE/FE)"),
]
ax.legend(handles=legend_handles, loc="upper center", bbox_to_anchor=(0.5, -0.16),
          ncol=2, frameon=False, fontsize=9.5)
fig.tight_layout()
fig.savefig(FIG / "fig4_loc_vs_success.png", bbox_inches="tight")
plt.close(fig)

# ============ FIGUR 5: Basarisizliklarin kok neden dagilimi (Round 1) ============
# Elle tanimlanan kok-neden gruplari (analize dayali)
rootcauses = [
    ("Unsigned tamsayı taşması", 2, "runtime_error"),      # s09, s14
    ("String modeli (karakter/bayt)", 2, "functional_error"),  # s06, s13
    ("char işaret (signedness)", 2, "functional_error"),   # s20, s49
    ("Çıktı biçimlendirme (%g)", 3, "functional_error"),   # s15, s27, s48
    ("Global durum → static mut", 1, "compilation_error"), # s19
    ("Tamsayı genişliği (platform)", 2, "functional_error"),  # s38, s51
    ("usize taşması (yeni)", 2, "runtime_error"),  # s40, s52
    ("Switch fallthrough (yeni)", 2, "functional_error"),  # s43, s53
    ("Makro çoklu-değerlendirme (yeni)", 1, "functional_error"),  # s56
]
rootcauses.sort(key=lambda t: t[1])
names = [t[0] for t in rootcauses]
counts = [t[1] for t in rootcauses]
colors = [COL[t[2]] for t in rootcauses]

fig, ax = plt.subplots(figsize=(7.8, 4.0))
ax.barh(names, counts, color=colors, edgecolor="white", height=0.62)
for i, v in enumerate(counts):
    ax.text(v + 0.05, i, str(v), va="center", fontsize=11, fontweight="bold")
ax.set_xlabel("Başarısız örnek sayısı")
ax.set_xlim(0, max(counts) + 0.8)
ax.set_title("Şekil 5. Başarısızlıkların kök-neden dağılımı (Round 1)", pad=12)
legend_el = [
    Patch(facecolor=COL["runtime_error"], label="Çalışma Zamanı (RE)"),
    Patch(facecolor=COL["functional_error"], label="Fonksiyonel (FE)"),
    Patch(facecolor=COL["compilation_error"], label="Derleme (CE)"),
]
ax.legend(handles=legend_el, loc="lower right", frameon=False, fontsize=9.5)
ax.grid(axis="y", alpha=0)
fig.tight_layout()
fig.savefig(FIG / "fig5_rootcause.png", bbox_inches="tight")
plt.close(fig)

# ============ FIGUR 4b: Bootstrap %95 guven arali ile EA (Faz 2) ============
import numpy as np
rng = np.random.default_rng(seed=42)  # stats_report.py ile ayni seed

def bootstrap_ci(pass_flags, n_boot=5000):
    arr = np.array(pass_flags, dtype=float)
    point = 100.0 * arr.mean()
    boots = np.array([100.0 * rng.choice(arr, size=len(arr), replace=True).mean()
                       for _ in range(n_boot)])
    lo, hi = np.percentile(boots, [2.5, 97.5])
    return point, lo, hi

labels4b, points4b, los4b, his4b = [], [], [], []
for lab, name in COND:
    d = load(lab)
    if d:
        flags = [r["category"] == "pass" for r in d["results"]]
        point, lo, hi = bootstrap_ci(flags)
        labels4b.append(name)
        points4b.append(point)
        los4b.append(point - lo)
        his4b.append(hi - point)

fig, ax = plt.subplots(figsize=(6.6, 4.2))
bars = ax.bar(labels4b, points4b, color=[COL["bar"], COL["non_termination"], COL["pass"]],
              width=0.5, yerr=[los4b, his4b], capsize=6,
              error_kw=dict(elinewidth=1.3, ecolor="#333"))
ax.set_ylabel("Yürütme Doğruluğu (EA) % (bootstrap %95 GA ile)")
ax.set_ylim(0, 112)
ax.set_title("Şekil 4b. EA ve bootstrap %95 güven aralığı (n=5000 tekrar)", pad=12)
for b, v in zip(bars, points4b):
    ax.text(b.get_x() + b.get_width() / 2, v + 8, f"%{v:.1f}", ha="center",
            fontsize=11, fontweight="bold")
ax.grid(axis="x", alpha=0)
fig.tight_layout()
fig.savefig(FIG / "fig4b_bootstrap_ci.png", bbox_inches="tight")
plt.close(fig)

print("Figurler uretildi:")
for f in sorted(FIG.glob("*.png")):
    print("  ", f.relative_to(ROOT))

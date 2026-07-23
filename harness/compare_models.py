#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Uretilmis her translations_rust__<model>/ klasoru icin run_experiment.py'yi
calistirir (mevcut degerlendirme mantigina hic dokunmadan) ve model basina
EA + hata dagilimini karsilastiran bir tablo uretir.

Kullanim:
  python3 harness/compare_models.py
  python3 harness/compare_models.py --models gemini,gpt4o
"""
import argparse
import json
import subprocess
import sys
from pathlib import Path

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

ROOT = Path(__file__).resolve().parent.parent

ALL_MODELS = ["gemini", "gpt4o", "deepseek"]


def run_experiment_for(model: str, release: bool = False) -> dict | None:
    rust_dir = f"translations_rust__{model}"
    if not (ROOT / rust_dir).exists():
        print(f"[atlandi] {rust_dir} bulunamadi (henuz ceviri uretilmemis).")
        return None
    label = f"{model}_release" if release else model
    cmd = [sys.executable, str(ROOT / "harness" / "run_experiment.py"),
           "--rust-dir", rust_dir, "--label", label, "--skip-missing"]
    if release:
        cmd.append("--release")
    r = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)
    print(r.stdout)
    if r.returncode != 0:
        print(r.stderr, file=sys.stderr)
    result_path = ROOT / "results" / f"results_{label}.json"
    if not result_path.exists():
        return None
    return json.loads(result_path.read_text(encoding="utf-8"))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--models", default=",".join(ALL_MODELS))
    args = ap.parse_args()
    models = args.models.split(",")

    rows = []
    # Claude Sonnet 5 referans satiri (mevcut, halihazirda olcumlu sonuc)
    claude_path = ROOT / "results" / "results_round1.json"
    if claude_path.exists():
        d = json.loads(claude_path.read_text(encoding="utf-8"))
        rows.append(("claude-sonnet-5 (referans, round1)", d))

    for model in models:
        d = run_experiment_for(model)
        if d:
            rows.append((model, d))

    if len(rows) <= 1:
        print("\nKarsilastirilacak model sonucu yok (Claude disinda). "
              "Once harness/generate_translations.py ile ceviri uretin.")
        return

    dataset_total = len(list((ROOT / "samples_c").glob("*.c"))) + \
        len(list((ROOT / "samples_c").glob("*/manifest.json")))

    lines = [f"| Model | Kapsam (degerlendirilen/toplam {dataset_total}) | EA (örnek) | EA % | CE | RE | FE | NT |",
             "|---|---|---|---|---|---|---|---|"]
    for name, d in rows:
        c = d["sample_category_counts"]
        coverage = f"{d['total_samples']}/{dataset_total}"
        note = "" if d["total_samples"] == dataset_total else " [KISMI - kota/hata nedeniyle eksik]"
        lines.append(
            f"| {name} | {coverage}{note} | {d['passed_samples']}/{d['total_samples']} | "
            f"%{d['execution_accuracy_pct']:.2f} | {c['compilation_error']} | "
            f"{c['runtime_error']} | {c['functional_error']} | {c['non_termination']} |"
        )
    table_md = "\n".join(lines)
    print("\n" + table_md)

    out_md = ROOT / "results" / "model_comparison.md"
    out_md.write_text(
        "# Model Karşılaştırması (gerçek ölçüm)\n\n" + table_md + "\n", encoding="utf-8"
    )
    print("\nYazildi:", out_md.relative_to(ROOT))


if __name__ == "__main__":
    main()

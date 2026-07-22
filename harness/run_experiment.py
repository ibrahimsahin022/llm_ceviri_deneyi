#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
LLM Kod Cevirisi Deney Ortami — Degerlendirme Motoru (Harness)
==============================================================

Amac: Bir kaynak dildeki (C) programlarin, bir LLM tarafindan hedef dile (Rust)
cevrilmis hallerini DIFERANSIYEL TEST ile degerlendirmek.

Yontem (her ornek icin):
  1) C kaynagini derle  -> referans (ground truth) ikili dosya
  2) Rust cevirisini derle
  3) Her test girdisi icin:
        - C'yi calistir  -> beklenen cikti
        - Rust'i calistir -> aday cikti
        - Karsilastir ve siniflandir
  4) Ornek verdicti + toplu istatistikler

Hata Taksonomisi (literaturdeki 4 tur):
  - CE  : Compilation Error   (Rust derlenmedi)
  - RE  : Runtime Error       (calisirken panic/hata, sifir olmayan cikis)
  - NT  : Non-termination     (zaman asimi / sonsuz dongu)
  - FE  : Functional Error    (calisti ama cikti referanstan farkli)
  - PASS: cikti referansla ayni

Ana metrik: Execution Accuracy (EA) = tum test girdilerini gecen ornek sayisi / toplam ornek.

Kullanim:
  python3 run_experiment.py                         # varsayilan (debug, overflow-checks ON)
  python3 run_experiment.py --rust-dir translations_rust_refined   # iyilestirme turu
  python3 run_experiment.py --release               # release derleme (overflow kontrolu KAPALI)
  python3 run_experiment.py --timeout 5             # ornek basina zaman asimi (sn)
"""

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path

# Proje kok dizini (bu betigin bir ust klasoru)
ROOT = Path(__file__).resolve().parent.parent

# Windows'ta calistirilabilir dosyalar .exe uzantisi ister
EXE = ".exe" if os.name == "nt" else ""

CAT_ORDER = ["compilation_error", "non_termination", "runtime_error", "functional_error", "pass"]
CAT_TR = {
    "compilation_error": "Derleme Hatasi (CE)",
    "runtime_error": "Calisma Zamani Hatasi (RE)",
    "non_termination": "Sonlanmama (NT)",
    "functional_error": "Fonksiyonel Hata (FE)",
    "pass": "Basarili (PASS)",
}


def sh(cmd, **kw):
    return subprocess.run(cmd, capture_output=True, **kw)


def normalize(b: bytes) -> bytes:
    """Bastaki/sondaki bosluklari ve satir sonu farklarini normalize et."""
    if b is None:
        return b""
    b = b.replace(b"\r\n", b"\n").replace(b"\r", b"\n")
    return b.strip()


def discover_samples(rust_dir: Path):
    samples = []
    for c_file in sorted((ROOT / "samples_c").glob("*.c")):
        sid = c_file.stem
        rs_file = rust_dir / f"{sid}.rs"
        tests_dir = ROOT / "tests" / sid
        samples.append({
            "id": sid,
            "c": c_file,
            "rs": rs_file,
            "tests": tests_dir,
            "loc_c": sum(1 for _ in c_file.open(encoding="utf-8", errors="replace")),
        })
    return samples


def compile_c(sample, build_dir: Path):
    out = build_dir / f"{sample['id']}_c{EXE}"
    r = sh(["gcc", "-O2", "-o", str(out), str(sample["c"]), "-lm"])
    return (r.returncode == 0, out, r.stderr.decode(errors="replace"))


def compile_rust(sample, build_dir: Path, release: bool):
    out = build_dir / f"{sample['id']}_rust{EXE}"
    if release:
        flags = ["-O", "-C", "overflow-checks=off"]
    else:
        # Gelistiricinin varsayilani (cargo run / debug): tasma kontrolu ACIK
        flags = ["-C", "opt-level=0", "-C", "debug-assertions=on", "-C", "overflow-checks=on"]
    cmd = ["rustc", *flags, "-o", str(out), str(sample["rs"])]
    r = sh(cmd)
    return (r.returncode == 0, out, r.stderr.decode(errors="replace"))


def run_binary(binary: Path, input_bytes: bytes, timeout: float):
    """(status, stdout_bytes, stderr_text) dondurur.
    status: 'ok' | 'runtime' | 'timeout'"""
    try:
        r = subprocess.run([str(binary)], input=input_bytes,
                           capture_output=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return "timeout", b"", ""
    if r.returncode != 0:
        return "runtime", r.stdout, r.stderr.decode(errors="replace")
    return "ok", r.stdout, r.stderr.decode(errors="replace")


def evaluate_sample(sample, build_dir, timeout, release):
    result = {
        "id": sample["id"],
        "loc_c": sample["loc_c"],
        "category": None,
        "cases": [],
        "compile_error": "",
        "notes": "",
    }

    # 1) C referansini derle
    c_ok, c_bin, c_err = compile_c(sample, build_dir)
    if not c_ok:
        result["category"] = "compilation_error"
        result["notes"] = "REFERANS C DERLENMEDI: " + c_err[:400]
        return result

    # 2) Rust cevirisini derle
    if not sample["rs"].exists():
        result["category"] = "compilation_error"
        result["compile_error"] = "Rust cevirisi bulunamadi: " + str(sample["rs"])
        return result
    rs_ok, rs_bin, rs_err = compile_rust(sample, build_dir, release)
    if not rs_ok:
        result["category"] = "compilation_error"
        result["compile_error"] = rs_err[:800]
        return result

    # 3) Test girdileri
    test_files = sorted(sample["tests"].glob("*.txt")) if sample["tests"].exists() else []
    if not test_files:
        result["notes"] = "Test girdisi yok"
    worst = "pass"
    for tf in test_files:
        inp = tf.read_bytes()
        c_status, c_out, _ = run_binary(c_bin, inp, timeout)
        r_status, r_out, r_stderr = run_binary(rs_bin, inp, timeout)

        if r_status == "timeout":
            cat = "non_termination"
        elif r_status == "runtime":
            cat = "runtime_error"
        elif normalize(r_out) != normalize(c_out):
            cat = "functional_error"
        else:
            cat = "pass"

        case = {
            "input": tf.name,
            "category": cat,
            "expected": normalize(c_out).decode(errors="replace")[:200],
            "got": normalize(r_out).decode(errors="replace")[:200],
        }
        if cat == "runtime_error":
            lines = [ln for ln in r_stderr.strip().splitlines() if ln.strip()]
            panic_line = next((ln for ln in lines if "panic" in ln.lower()), lines[-1] if lines else "")
            case["stderr"] = panic_line[:250]
        result["cases"].append(case)

        # en kotu kategoriyi tut (CE zaten yukarida ele alindi)
        if CAT_ORDER.index(cat) < CAT_ORDER.index(worst):
            worst = cat
    result["category"] = worst
    return result


def main():
    ap = argparse.ArgumentParser(description="LLM kod cevirisi deney degerlendirme motoru")
    ap.add_argument("--rust-dir", default="translations_rust",
                    help="Rust cevirilerinin bulundugu klasor (ROOT'a gore)")
    ap.add_argument("--timeout", type=float, default=5.0, help="Ornek basina zaman asimi (sn)")
    ap.add_argument("--release", action="store_true",
                    help="Release derleme (tasma kontrolu KAPALI)")
    ap.add_argument("--label", default=None, help="Sonuc dosyalari icin etiket")
    ap.add_argument("--skip-missing", action="store_true",
                    help="Rust cevirisi dosyasi olmayan ornekleri CE saymak yerine "
                         "tamamen atla (kismi/kota-sinirli coklu-model kosumlari icin)")
    args = ap.parse_args()

    rust_dir = (ROOT / args.rust_dir).resolve()
    build_dir = ROOT / "build"
    build_dir.mkdir(exist_ok=True)
    results_dir = ROOT / "results"
    results_dir.mkdir(exist_ok=True)

    label = args.label or (Path(args.rust_dir).name + ("_release" if args.release else "_debug"))

    print("=" * 70)
    print("LLM KOD CEVIRISI DENEYI — C -> Rust")
    print("Rust klasoru :", rust_dir.name)
    print("Derleme modu :", "RELEASE (overflow-checks OFF)" if args.release else "DEBUG (overflow-checks ON)")
    print("Zaman asimi  :", args.timeout, "sn")
    print("=" * 70)

    samples = discover_samples(rust_dir)
    if args.skip_missing:
        skipped = [s["id"] for s in samples if not s["rs"].exists()]
        samples = [s for s in samples if s["rs"].exists()]
        if skipped:
            print(f"[--skip-missing] {len(skipped)} ornek atlandi (ceviri yok): {', '.join(skipped)}")
    results = []
    for s in samples:
        res = evaluate_sample(s, build_dir, args.timeout, args.release)
        results.append(res)
        n_cases = len(res["cases"])
        n_pass = sum(1 for c in res["cases"] if c["category"] == "pass")
        flag = "OK " if res["category"] == "pass" else "XX "
        print(f"{flag}{res['id']:<22} LoC(C)={res['loc_c']:<3} "
              f"kategori={CAT_TR[res['category']]:<28} testler={n_pass}/{n_cases}")

    # ---- Ozet istatistikler ----
    total = len(results)
    passed = sum(1 for r in results if r["category"] == "pass")
    ea = 100.0 * passed / total if total else 0.0

    cat_counts = {c: 0 for c in CAT_ORDER}
    for r in results:
        cat_counts[r["category"]] += 1

    # test-girdisi bazinda dagilim
    case_counts = {c: 0 for c in CAT_ORDER}
    total_cases = 0
    for r in results:
        for c in r["cases"]:
            case_counts[c["category"]] += 1
            total_cases += 1

    print("\n" + "-" * 70)
    print(f"YURUTME DOGRULUGU (EA): {passed}/{total} = %{ea:.2f}")
    print("-" * 70)
    print("Ornek bazinda kategori dagilimi:")
    for c in CAT_ORDER:
        print(f"  {CAT_TR[c]:<30}: {cat_counts[c]:>3} ornek")
    print("\nTest-girdisi bazinda dagilim (toplam {}):".format(total_cases))
    for c in CAT_ORDER:
        pct = 100.0 * case_counts[c] / total_cases if total_cases else 0
        print(f"  {CAT_TR[c]:<30}: {case_counts[c]:>3} (%{pct:.1f})")

    # ---- Dosyalara yaz ----
    summary = {
        "label": label,
        "rust_dir": rust_dir.name,
        "release": args.release,
        "total_samples": total,
        "passed_samples": passed,
        "execution_accuracy_pct": round(ea, 2),
        "sample_category_counts": cat_counts,
        "case_category_counts": case_counts,
        "total_cases": total_cases,
        "results": results,
    }
    (results_dir / f"results_{label}.json").write_text(
        json.dumps(summary, ensure_ascii=False, indent=2), encoding="utf-8")

    # CSV (ornek bazinda)
    csv_lines = ["sample_id,loc_c,category,n_cases,n_pass"]
    for r in results:
        n_cases = len(r["cases"])
        n_pass = sum(1 for c in r["cases"] if c["category"] == "pass")
        csv_lines.append(f"{r['id']},{r['loc_c']},{r['category']},{n_cases},{n_pass}")
    (results_dir / f"results_{label}.csv").write_text("\n".join(csv_lines), encoding="utf-8")

    print("\nSonuclar yazildi:")
    print("  ", results_dir / f"results_{label}.json")
    print("  ", results_dir / f"results_{label}.csv")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Coklu-model ceviri ureticisi.

samples_c/*.c (tek dosya) ve samples_c/*/manifest.json (cok dosya, Faz 3)
uzerinde doner, secilen modelle (Gemini/GPT-4o/DeepSeek) her programi
Rust'a cevirir, sonucu translations_rust__<model>/<id>.rs (tek dosya) veya
translations_rust__<model>/<id>/main.rs (cok dosya) olarak yazar ve her
cagrinin tam kaydini (istem, model kimligi, zaman damgasi, temperature,
top_p) results/manifest_<model>.json'a ekler/gunceller.

Kullanim:
  python3 harness/generate_translations.py --model gemini
  python3 harness/generate_translations.py --model gpt4o --dry-run
  python3 harness/generate_translations.py --model deepseek --only s01_sum,s02_gcd
"""
import argparse
import json
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "harness"))

try:
    from dotenv import load_dotenv
    load_dotenv(ROOT / ".env")
except ImportError:
    pass

from translators import PROMPT_TEMPLATE, DEFAULT_TEMPERATURE, DEFAULT_TOP_P  # noqa: E402

MODEL_DIR_NAMES = {
    "gemini": "translations_rust__gemini",
    "gpt4o": "translations_rust__gpt4o",
    "deepseek": "translations_rust__deepseek",
}


def get_translator(model: str):
    if model == "gemini":
        from translators.gemini_translator import GeminiTranslator
        return GeminiTranslator()
    if model == "gpt4o":
        from translators.openai_translator import OpenAITranslator
        return OpenAITranslator()
    if model == "deepseek":
        from translators.deepseek_translator import DeepSeekTranslator
        return DeepSeekTranslator()
    raise ValueError(f"Bilinmeyen model: {model}")


def discover_samples():
    """(sid, c_source_text, is_multi) uclusu listesi dondurur. Tek-dosya
    ornekler samples_c/*.c'den, cok-dosya ornekler samples_c/*/manifest.json
    ile isaretli dizinlerden (Faz 3) okunur; cok-dosyali kaynak, dosya
    sinirlarini belirten yorum basliklariyla tek bir istem metnine
    birlestirilir (model tek bir main.rs uretecek sekilde yonlendirilir)."""
    samples = []
    samples_c_dir = ROOT / "samples_c"

    for c_file in sorted(samples_c_dir.glob("*.c")):
        sid = c_file.stem
        src = c_file.read_text(encoding="utf-8", errors="replace")
        samples.append((sid, src, False))

    for manifest_path in sorted(samples_c_dir.glob("*/manifest.json")):
        sid = manifest_path.parent.name
        m = json.loads(manifest_path.read_text(encoding="utf-8"))
        parts = []
        for fname in m["c_files"]:
            fpath = manifest_path.parent / fname
            parts.append(f"/* ---- dosya: {fname} ---- */\n" +
                         fpath.read_text(encoding="utf-8", errors="replace"))
        src = "\n\n".join(parts)
        src += ("\n\n/* NOT: Bu, birden fazla C dosyasindan (yukarida belirtilen dosya "
                "adlariyla) olusan cok-dosyali bir programdir. Lutfen TEK BIR Rust "
                "dosyasi (tek bir main.rs) olarak, ayni davranissal sozlesmeyi "
                "koruyacak sekilde cevirin. */")
        samples.append((sid, src, True))

    samples.sort(key=lambda t: t[0])
    return samples


def output_path_for(out_dir: Path, sid: str, is_multi: bool) -> Path:
    if is_multi:
        d = out_dir / sid
        d.mkdir(parents=True, exist_ok=True)
        return d / "main.rs"
    return out_dir / f"{sid}.rs"


def main():
    ap = argparse.ArgumentParser(description="Coklu-model C->Rust ceviri ureticisi")
    ap.add_argument("--model", required=True, choices=["gemini", "gpt4o", "deepseek"])
    ap.add_argument("--dry-run", action="store_true",
                     help="API'yi cagirmadan istemi olusturup yazdirir (anahtar gerekmez)")
    ap.add_argument("--only", default=None,
                     help="Virgulle ayrilmis ornek id listesi (ör. s01_sum,s02_gcd); "
                          "verilmezse tum ornekler islenir")
    ap.add_argument("--limit", type=int, default=None,
                     help="Ilk N ornekle sinirla (maliyet/hiz testi icin)")
    ap.add_argument("--sleep", type=float, default=4.0,
                     help="Her API cagrisi arasinda bekleme (sn) - ucretsiz katman rate-limit'ini asmamak icin")
    args = ap.parse_args()

    samples = discover_samples()
    if args.only:
        wanted = set(args.only.split(","))
        samples = [s for s in samples if s[0] in wanted]
    if args.limit:
        samples = samples[: args.limit]

    if not samples:
        print("Islenecek ornek bulunamadi.")
        return

    out_dir = ROOT / MODEL_DIR_NAMES[args.model]
    out_dir.mkdir(exist_ok=True)
    manifest_path = ROOT / "results" / f"manifest_{args.model}.json"
    manifest_path.parent.mkdir(exist_ok=True)

    # Mevcut manifesti sample_id -> kayit sozlugu olarak yukle; bu calistirmada
    # islenen id'ler icin guncellenir, digerleri (ör. onceki kismi kosumdan
    # kalan gercek kayitlar) OLDUGU GIBI korunur.
    manifest_by_id = {}
    if manifest_path.exists():
        for entry in json.loads(manifest_path.read_text(encoding="utf-8")):
            manifest_by_id[entry["sample_id"]] = entry

    translator = None
    if not args.dry_run:
        translator = get_translator(args.model)

    print(f"Model: {args.model} | dry-run: {args.dry_run} | ornek sayisi: {len(samples)}")
    print("-" * 70)

    for i, (sid, c_source, is_multi) in enumerate(samples):
        if args.dry_run:
            prompt = PROMPT_TEMPLATE.format(c_source=c_source)
            print(f"[DRY-RUN] {sid}: istem hazirlandi ({len(prompt)} karakter), API cagrilmadi.")
            manifest_by_id[sid] = {
                "sample_id": sid,
                "model_id": f"{args.model} (DRY-RUN, cagrilmadi)",
                "prompt_chars": len(prompt),
                "temperature": DEFAULT_TEMPERATURE,
                "top_p": DEFAULT_TOP_P,
                "dry_run": True,
            }
            continue

        try:
            result = translator.translate(c_source, sid)
        except Exception as exc:  # gercek API hatasi: uydurma sonuc uretme, oldugu gibi bildir
            print(f"[HATA] {sid}: {exc}")
            manifest_by_id[sid] = {
                "sample_id": sid,
                "model_id": getattr(translator, "model_id", args.model),
                "error": str(exc),
            }
            continue

        rs_path = output_path_for(out_dir, sid, is_multi)
        rs_path.write_text(result.rust_code, encoding="utf-8")
        manifest_by_id[sid] = {
            "sample_id": sid,
            "model_id": result.model_id,
            "timestamp_utc": result.timestamp_utc,
            "temperature": result.temperature,
            "top_p": result.top_p,
            "prompt_text": result.prompt_text,
            "raw_response_meta": result.raw_response_meta,
        }
        print(f"[OK] {sid}: {rs_path.relative_to(ROOT)} yazildi ({result.model_id})")

        if args.sleep > 0 and i < len(samples) - 1:
            time.sleep(args.sleep)

    manifest = [manifest_by_id[k] for k in sorted(manifest_by_id)]
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2), encoding="utf-8")
    print("-" * 70)
    print("Manifest yazildi:", manifest_path.relative_to(ROOT))


if __name__ == "__main__":
    main()

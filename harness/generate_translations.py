#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Coklu-model ceviri ureticisi.

samples_c/*.c uzerinde doner, secilen modelle (Gemini/GPT-4o/DeepSeek) her
programi Rust'a cevirir, sonucu translations_rust__<model>/<id>.rs olarak
yazar ve her cagrinin tam kaydini (istem, model kimligi, zaman damgasi,
temperature, top_p) results/manifest_<model>.json'a ekler.

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


def main():
    ap = argparse.ArgumentParser(description="Coklu-model C->Rust ceviri ureticisi")
    ap.add_argument("--model", required=True, choices=["gemini", "gpt4o", "deepseek"])
    ap.add_argument("--dry-run", action="store_true",
                     help="API'yi cagirmadan istemi olusturup yazdirir (anahtar gerekmez)")
    ap.add_argument("--only", default=None,
                     help="Virgulle ayrilmis ornek id listesi (ör. s01_sum,s02_gcd); "
                          "verilmezse tum samples_c/*.c islenir")
    ap.add_argument("--limit", type=int, default=None,
                     help="Ilk N ornekle sinirla (maliyet/hiz testi icin)")
    ap.add_argument("--sleep", type=float, default=4.0,
                     help="Her API cagrisi arasinda bekleme (sn) - ucretsiz katman rate-limit'ini asmamak icin")
    args = ap.parse_args()

    c_files = sorted((ROOT / "samples_c").glob("*.c"))
    if args.only:
        wanted = set(args.only.split(","))
        c_files = [f for f in c_files if f.stem in wanted]
    if args.limit:
        c_files = c_files[: args.limit]

    if not c_files:
        print("Islenecek ornek bulunamadi.")
        return

    out_dir = ROOT / MODEL_DIR_NAMES[args.model]
    out_dir.mkdir(exist_ok=True)
    manifest_path = ROOT / "results" / f"manifest_{args.model}.json"
    manifest_path.parent.mkdir(exist_ok=True)
    # Her calistirma, o modelin GUNCEL/TAM uretim turunu temsil eder - eski
    # (orn. dry-run) girdilerle karismasin diye manifest sifirdan baslar.
    manifest = []

    translator = None
    if not args.dry_run:
        translator = get_translator(args.model)

    print(f"Model: {args.model} | dry-run: {args.dry_run} | ornek sayisi: {len(c_files)}")
    print("-" * 70)

    for c_file in c_files:
        sid = c_file.stem
        c_source = c_file.read_text(encoding="utf-8", errors="replace")

        if args.dry_run:
            prompt = PROMPT_TEMPLATE.format(c_source=c_source)
            print(f"[DRY-RUN] {sid}: istem hazirlandi ({len(prompt)} karakter), API cagrilmadi.")
            manifest.append({
                "sample_id": sid,
                "model_id": f"{args.model} (DRY-RUN, cagrilmadi)",
                "prompt_chars": len(prompt),
                "temperature": DEFAULT_TEMPERATURE,
                "top_p": DEFAULT_TOP_P,
                "dry_run": True,
            })
            continue

        try:
            result = translator.translate(c_source, sid)
        except Exception as exc:  # gercek API hatasi: uydurma sonuc uretme, oldugu gibi bildir
            print(f"[HATA] {sid}: {exc}")
            manifest.append({
                "sample_id": sid,
                "model_id": getattr(translator, "model_id", args.model),
                "error": str(exc),
            })
            continue

        rs_path = out_dir / f"{sid}.rs"
        rs_path.write_text(result.rust_code, encoding="utf-8")
        manifest.append({
            "sample_id": sid,
            "model_id": result.model_id,
            "timestamp_utc": result.timestamp_utc,
            "temperature": result.temperature,
            "top_p": result.top_p,
            "prompt_text": result.prompt_text,
            "raw_response_meta": result.raw_response_meta,
        })
        print(f"[OK] {sid}: {rs_path.relative_to(ROOT)} yazildi ({result.model_id})")

        if args.sleep > 0 and c_file != c_files[-1]:
            time.sleep(args.sleep)

    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2), encoding="utf-8")
    print("-" * 70)
    print("Manifest yazildi:", manifest_path.relative_to(ROOT))


if __name__ == "__main__":
    main()

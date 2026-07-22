"""Google Gemini adaptoru (google-genai SDK, aistudio.google.com anahtari)."""
from __future__ import annotations

import os

from . import DEFAULT_TEMPERATURE, DEFAULT_TOP_P, Translator


class GeminiTranslator(Translator):
    def __init__(self, model_id: str | None = None):
        from google import genai  # gec import: paket kurulu degilse sadece bu adaptor patlar

        api_key = os.environ.get("GOOGLE_API_KEY")
        if not api_key:
            raise RuntimeError(
                "GOOGLE_API_KEY ortam degiskeni bulunamadi. .env dosyasina ekleyin "
                "(bkz. .env.example) veya `export GOOGLE_API_KEY=...` calistirin."
            )
        self.model_id = model_id or os.environ.get("GEMINI_MODEL", "gemini-flash-latest")
        self._client = genai.Client(api_key=api_key)

    def _call_api(self, prompt: str) -> tuple[str, dict]:
        from google.genai import types

        resp = self._client.models.generate_content(
            model=self.model_id,
            contents=prompt,
            config=types.GenerateContentConfig(
                temperature=DEFAULT_TEMPERATURE,
                top_p=DEFAULT_TOP_P,
            ),
        )
        text = resp.text or ""
        meta = {
            "finish_reason": getattr(
                resp.candidates[0], "finish_reason", None
            ) if resp.candidates else None,
            "usage": resp.usage_metadata.model_dump() if getattr(resp, "usage_metadata", None) else None,
        }
        return text, meta

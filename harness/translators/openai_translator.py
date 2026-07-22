"""OpenAI adaptoru (openai SDK, platform.openai.com anahtari)."""
from __future__ import annotations

import os

from . import DEFAULT_TEMPERATURE, DEFAULT_TOP_P, Translator


class OpenAITranslator(Translator):
    def __init__(self, model_id: str | None = None):
        from openai import OpenAI

        api_key = os.environ.get("OPENAI_API_KEY")
        if not api_key:
            raise RuntimeError(
                "OPENAI_API_KEY ortam degiskeni bulunamadi. .env dosyasina ekleyin "
                "(bkz. .env.example) veya `export OPENAI_API_KEY=...` calistirin."
            )
        self.model_id = model_id or os.environ.get("OPENAI_MODEL", "gpt-4o")
        self._client = OpenAI(api_key=api_key)

    def _call_api(self, prompt: str) -> tuple[str, dict]:
        resp = self._client.chat.completions.create(
            model=self.model_id,
            messages=[{"role": "user", "content": prompt}],
            temperature=DEFAULT_TEMPERATURE,
            top_p=DEFAULT_TOP_P,
        )
        text = resp.choices[0].message.content or ""
        meta = {
            "finish_reason": resp.choices[0].finish_reason,
            "usage": resp.usage.model_dump() if resp.usage else None,
        }
        return text, meta

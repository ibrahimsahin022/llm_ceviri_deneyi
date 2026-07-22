"""
Model-bagimsiz LLM ceviri araligi (Translator arayuzu).

Her adaptor (gemini_translator.py, openai_translator.py, deepseek_translator.py)
bu modüldeki Translator soyut sinifini uygular. Amac: run_experiment.py'nin
degerlendirme mantigina hic dokunmadan, farkli saglayicilarin C->Rust cevirisini
ayni sabit istemle (prompt), ayni sabit sampling parametreleriyle uretip
tam manifest (istem, model kimligi, zaman damgasi, temperature, top_p) ile
kaydetmek — boylece tekrarlanabilirlik ve model-karsilastirmasi mumkun olsun.
"""
from __future__ import annotations

import re
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from datetime import datetime, timezone


PROMPT_TEMPLATE = """Asagidaki C programini, davranissal olarak TAM ESDEGER bir Rust programina cevir.

Kurallar:
- Program stdin'den okuyup stdout'a yazmalidir; giris/cikis sozlesmesi (format) C ile birebir ayni kalmalidir.
- Yalnizca Rust kaynak kodunu dondur; aciklama, markdown code fence (```), veya baska hicbir metin ekleme.
- Kodun derlenebilir ve calisir olmasi gerekir (rustc ile dogrudan derlenecek, Cargo projesi degil).
- C kodunun semantigini (tasma davranisi, string/bayt modeli, bicimlendirme vb.) olabildigince sadik yansit;
  ancak idiyomatik/guvenli Rust yazmaktan cekinme (orn. unsafe yalnizca gercekten gerekliyse).

C kaynak kodu:
```c
{c_source}
```

Rust kodu:"""

# Tum adaptorler icin SABIT sampling parametreleri (tekrarlanabilirlik icin
# manifestte de ayrica loglanir). Degistirmek isteyen bir adaptor bunu
# TranslationResult icinde farkli bir deger olarak raporlayabilir, ancak
# varsayilan budur.
DEFAULT_TEMPERATURE = 0.2
DEFAULT_TOP_P = 1.0


@dataclass
class TranslationResult:
    rust_code: str
    model_id: str
    prompt_text: str
    temperature: float
    top_p: float
    timestamp_utc: str = field(default_factory=lambda: datetime.now(timezone.utc).isoformat())
    raw_response_meta: dict = field(default_factory=dict)


class Translator(ABC):
    """Her saglayici adaptoru bu sinifi uygular."""

    #: alt siniflar tarafindan doldurulur (orn. "gemini-2.5-flash")
    model_id: str = "unknown"

    @abstractmethod
    def _call_api(self, prompt: str) -> tuple[str, dict]:
        """(ham_metin, raw_response_meta) dondurur. Alt sinif uygular."""
        raise NotImplementedError

    def translate(self, c_source: str, sample_id: str) -> TranslationResult:
        prompt = PROMPT_TEMPLATE.format(c_source=c_source)
        raw_text, meta = self._call_api(prompt)
        rust_code = extract_rust_code(raw_text)
        return TranslationResult(
            rust_code=rust_code,
            model_id=self.model_id,
            prompt_text=prompt,
            temperature=DEFAULT_TEMPERATURE,
            top_p=DEFAULT_TOP_P,
            raw_response_meta=meta,
        )


_CODE_FENCE_RE = re.compile(r"```(?:rust)?\s*\n(.*?)```", re.DOTALL)


def extract_rust_code(text: str) -> str:
    """LLM yaniti markdown code fence icinde gelirse temizler; degilse
    yaniti oldugu gibi dondurur."""
    m = _CODE_FENCE_RE.search(text)
    if m:
        return m.group(1).strip() + "\n"
    return text.strip() + "\n"

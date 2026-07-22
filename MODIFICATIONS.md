# Hakem Zayıflıklarını Giderme — Değişiklik Günlüğü

Bu dosya, `C:\Users\ibrah\.claude\plans\fluttering-wiggling-corbato.md` planındaki
5 fazın her birinin sonunda güncellenir. Her girdi üç kısımdan oluşur:
(a) değişen/eklenen dosyalar, (b) o fazda gerçekten ölçülen sayılar, (c) makalenin
ilgili bölümüne eklenmesi önerilen taslak metin.

---

## Faz 1 — Çoklu Model Desteği

### (a) Değişen/eklenen dosyalar
- **Yeni:** `harness/translators/__init__.py` — `Translator` soyut arayüzü,
  `TranslationResult` veri sınıfı, sabit `PROMPT_TEMPLATE`, sabit sampling
  parametreleri (`temperature=0.2`, `top_p=1.0`).
- **Yeni:** `harness/translators/gemini_translator.py`,
  `harness/translators/openai_translator.py`,
  `harness/translators/deepseek_translator.py`.
- **Yeni:** `harness/generate_translations.py` (CLI, `--model`, `--dry-run`,
  `--only`, `--limit`, `--sleep`), `harness/compare_models.py`.
- **Yeni:** `.env.example`, `.gitignore`.
- **Değişti:** `harness/run_experiment.py` — yalnızca `--skip-missing` bayrağı
  eklendi (rust dosyası olmayan örnekleri CE saymak yerine tamamen atlar;
  varsayılan davranış değişmedi, geriye dönük tam uyumlu).
- **Yeni:** `results/manifest_gemini.json` (gerçek, 19 kayıt),
  `results/manifest_gpt4o.json`, `results/manifest_deepseek.json` (48'er kayıt,
  hepsi `dry_run: true` — API çağrılmadı), `results/results_gemini.json/csv`,
  `results/model_comparison.md`.

### (b) Gerçekten ölçülen sayılar
- **Gemini (`gemini-flash-latest`, gerçek API çağrısı):** Google AI Studio
  ücretsiz katmanının **günlük kota sınırı** (`GenerateRequestsPerDayPerProjectPerModel-FreeTier`,
  limit=20 istek/gün/model) nedeniyle yalnızca **19/48** örnek çevrildi;
  20. istekten itibaren tüm çağrılar `429 RESOURCE_EXHAUSTED` ile reddedildi
  (ham hata mesajları `results/manifest_gemini.json`'da saklı). Kullanıcı
  talimatıyla mevcut 19 örnek üzerinden kısmi ama gerçek bir ölçüm raporlanıyor;
  hiçbir sayı uydurulmadı.
  - **Gemini EA (n=19, kısmi kapsam): 18/19 = %94.74** (1 FE: `s15_float_avg` —
    Claude'da da aynı kök nedenden [Kategori D, %g biçimlendirme] başarısız
    olan örnekle aynı; ilginç biçimde bağımsız bir model de aynı boşluğa
    düşmüş).
  - Claude Sonnet 5 (referans, aynı 19 örnek alt kümesinde tekrar hesaplanmadı,
    tam veri seti üzerinden mevcut sonuç): **36/48 = %75.00**.
  - **Not:** Bu iki sayı doğrudan karşılaştırılabilir DEĞİLDİR (farklı örneklem
    büyüklüğü/alt kümesi); doğru karşılaştırma için Gemini'nin kalan 29
    örneğinin tamamlanması (kota sıfırlandıktan sonra, yarından itibaren günde
    ~20 istekle kademeli olarak) gerekir.
  - OpenAI (GPT-4o) ve DeepSeek: **API anahtarı yok, hiç çağrılmadı.**
    Yalnızca `--dry-run` ile 48 örneğin tamamı için istem (prompt) inşası
    doğrulandı (`results/manifest_gpt4o.json`, `results/manifest_deepseek.json`
    — `dry_run: true` olarak işaretli, gerçek çeviri veya sonuç İÇERMEZ).

### (c) Makaleye önerilen taslak metin

**§3.2 Çeviri Süreci'ne eklenecek paragraf:**
> Çalışmanın ilk sürümünde yalnızca Claude Sonnet 5 kullanılmıştı. Hakem
> geri bildirimi doğrultusunda, model-bağımsız bir çeviri altyapısı
> (`harness/translators/`) kuruldu ve Google Gemini (`gemini-flash-latest`)
> için gerçek, otomatik API çağrılarıyla kısmi bir tekrar ölçüm yapıldı
> (19/48 örnek, Google AI Studio ücretsiz katmanının günlük kota sınırı
> nedeniyle). Bu kısmi örneklemde Gemini EA = %94.74 (18/19) ölçülmüştür;
> tek başarısızlık, Claude'da da aynı kök nedenden (çıktı biçimlendirme
> semantiği, §4.4.D) kaynaklanmıştır — bu, en azından bu tek örnekte, ilgili
> semantik boşluğun modele özgü olmayabileceğine dair ön bir işarettir. OpenAI
> ve DeepSeek adaptörleri de aynı altyapıyla yazılmış ve istem inşası
> doğrulanmıştır, ancak yazarların bu modeller için API erişimi olmadığından
> gerçek sonuç ölçülememiştir.

**§6 Geçerlilik Tehditleri → "Dış Geçerlilik" alt bölümüne eklenecek not:**
> Çoklu-model karşılaştırması bu sürümde tamamlanmamıştır: Gemini için
> yalnızca 19/48 örnek (günlük API kotası nedeniyle), GPT-4o ve DeepSeek için
> hiç gerçek sonuç yoktur. Dolayısıyla "bulgular yalnızca Claude Sonnet 5'e
> özgüdür" sınırlaması büyük ölçüde geçerliliğini korumaktadır; kısmi Gemini
> verisi yalnızca bir ön işarettir, kapsamlı bir karşılaştırma değildir.

### Sonraki adım
Google AI Studio kotası sıfırlandıkça (`python harness/generate_translations.py
--model gemini --only <eksik-id'ler> --sleep 4`) kalan 29 örnek tamamlanabilir;
tamamlandığında `python harness/compare_models.py` yeniden çalıştırılıp bu
bölüm tam n=48 sayılarıyla güncellenecektir.

---

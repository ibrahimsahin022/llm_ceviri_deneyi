# Model Karşılaştırması (gerçek ölçüm)

| Model | Kapsam (degerlendirilen/toplam 57) | EA (örnek) | EA % | CE | RE | FE | NT |
|---|---|---|---|---|---|---|---|
| claude-sonnet-5 (referans, round1, k=1) | 57/57 | 40/57 | %70.18 | 1 | 4 | 12 | 0 |
| gemini-flash-latest | 57/57 | 51/57 | %89.47 | 4 | 0 | 2 | 0 |
| claude-sonnet-5 (tekrar, taze/izole oturum, k=2) | 57/57 | 55/57 | %96.49 | 0 | 0 | 2 | 0 |
| claude-haiku-4-5 (katman-eşleştirilmiş, taze/izole oturum) | 57/57 | 36/57 | %63.16 | 2 | 2 | 17 | 0 |

## Notlar

- **k=2 (Sonnet, tekrar):** k=1 ile SIFIR ortak başarısızlık — k=1'in 17 hatasının tamamı düzelmiş, 2 yeni hata (s47_redis_sds, s54_stack_module) ortaya çıkmıştır. Ayrıntılı kök neden analizi ve yöntem (bulaşma/contamination denetimi dahil) için bkz. makale Bölüm IV-G ve `MODIFICATIONS.md`.
- **Haiku (katman-eşleştirilmiş):** Gemini Flash ile aynı "hafif/hızlı" katmanda ama ondan çok daha düşük doğrulukta (%63.16 vs %89.47); 2 derleme hatası tamamen yeni bir tür — Rust ödünç-denetleyici (borrow checker) ihlalleri (E0716, E0502) — Sonnet'in 17 hatasının hiçbirinde görülmemişti. Ayrıntı için makale Bölüm IV-F.
- Her iki yeni ölçüm de bu depodaki mevcut sonuçlardan/analiz dosyalarından habersiz, izole bir ajan oturumuyla üretilmiş ve yalnızca `samples_c/` okuyup kendi çıktı dizinine yazdığı işlem kaydı denetlenerek kontaminasyona karşı doğrulanmıştır.

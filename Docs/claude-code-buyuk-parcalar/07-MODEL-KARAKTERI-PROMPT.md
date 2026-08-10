## 7. Claude Spesifik Model Karakteri ve Prompt Mühendisliği

### 7.1 Claude'un "Kişiliği" ve Çalışma Tarzı

Modelleri sadece teknik kapasiteleriyle değil **davranışsal karakterleriyle** sınıflandırmak gerekir:

| Özellik | Claude (Fable/Opus/Sonnet) | Claude Haiku | GPT-5/Codex | DeepSeek/Gemini |
|:---|:---|:---|:---|:---|
| **Metafor** | Deneyimli meslektaş | Hızlı stajyer | Disiplinli robot | Ucuz iş gücü |
| **Bağlam anlayışı** | Çok yüksek (satır arası okur) | Yüksek | Yüksek (ama mekanik) | Orta |
| **Talimat esnekliği** | Yaratıcı, öneri sunar | Hızlı uyum | Tam itaat | Mekanik |
| **Aşırı düşünme** | Opus/Fable eğilimli; Sonnet orta; Haiku az | Çok az | Az | Çok az |
| **Hata paterni** | Karmaşık yerlerde ufak detay atlar | Büyük resim kaçırır | Yanlış anladıysa ısrar eder | Kod tekrarı, sığ cevap |
| **En iyi olduğu iş** | Mimari, yaratıcılık, zor bug | Özet, sınıflandırma, boilerplate | Doğrulama, sıfır-context inceleme | Yüksek hacim, tekrar |
| **En kötü olduğu iş** | Basit tekrarlayan işler | Yaratıcı mimari | Belirsiz problem (sorgulamaz) | Yeni konsept öğrenme |
| **Fiyat (Ağustos 2026)** | Fable $10/$50, Opus 5 $5/$25, Sonnet 5 $2-3/$10-15 | $1/$5 | modele göre | modele göre, genelde ucuz |

### 7.2 Claude Konuşma Tarzı Entegrasyonu

Claude ile çalışırken en iyi sonuç veren iletişim tarzı:

**✓ İYİ:** Net, yapılandırılmış, kabul kriterleri belirgin
```
Şu görevi yap: [Açık tanım]
Girdiler: [Dosya yolları, ilgili bağlam]
Kısıtlar: [Neler yapılamaz]
Kabul kriterleri: [Bittiğini nasıl anlarız]
Önce test yaz, sonra implementasyon. Herhangi bir varsayım yapmadan önce sor.
```

**✗ KÖTÜ:** Belirsiz, geniş, duygusal
```
Şu özelliği eklesene nasıl yaparsın iyi olur
```
*(Sonuç: Yanlış yönde koca bir dosya yığını)*

### 7.3 Aşırı Düşünmeyi (Overthinking) Engelleme

Claude'u basit görevler için fazla "düşünmeye" zorlamak kaliteyi düşürür ve token yakar:

**Basit görevlerde şu prompt ile başlayın:**
```
Bu basit bir görev. Gereksiz açıklama ve detay verme, sadece minimal düzeltmeyi yap:
[değişiklik]
```

**Karmaşık görevlerde ise tam tersi, düşünmeye teşvik edin:**
```
Bu karmaşık bir sorun. Acele etme. Adım adım düşün:
1. Önce nedenini analiz et
2. Olası çözümleri listele ve trade-off'larını açıkla
3. En iyi çözümü seç ve uygula
4. Sonucu test et ve doğrula
```

### 7.4 Kritik Karar Anları

Claude'u şu anlarda **mutlaka** durdurup onay alın:
- Yeni bir npm/pip paketi ekleneceği zaman
- Veritabanı migration'ı oluşturulacağı zaman
- Production environment variable'ları değiştirileceği zaman
- Auth/security ile ilgili kod değiştirildiğinde
- 200+ satırlık büyük bir değişiklikten önce
- Üçüncü parti API entegrasyonu öncesi

Bu anları CLAUDE.md içinde açıkça belirtin:

```markdown
## Onay Gerektiren Durumlar
Aşağıdaki durumlarda KOD YAZMA, önce insana sor:
- Yeni bağımlılık ekleme
- Migration oluşturma
- Güvenlik ile ilgili değişiklik
- 100 satırdan fazla tek seferde değişiklik
- Environment variable değişiklikleri
- Üçüncü parti servis entegrasyonu
```

---


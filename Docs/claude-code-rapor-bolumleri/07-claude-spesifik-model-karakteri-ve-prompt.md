## 7. Claude Spesifik Model Karakteri ve Prompt Mühendisliği

### 7.1 Claude'un "Kişiliği" ve Çalışma Tarzı

Modelleri sadece teknik kapasiteleriyle değil **davranışsal karakterleriyle** sınıflandırmak gerekir:

| Özellik | Claude Opus/Sonnet | GPT-4o/5 | DeepSeek-Coder |
|:---|:---|:---|:---|
| **Metafor** | Deneyimli meslektaş | Disiplinli robot | Hızlı stajyer |
| **Bağlam anlayışı** | Çok yüksek (satır arası okur) | Yüksek (ama mekanik) | Orta |
| **Talimat esnekliği** | Yaratıcı, öneri sunar | Tam itaat | Mekanik |
| **Aşırı düşünme** | Eğilimli (basit işi uzatabilir) | Az | Çok az |
| **Hata yapma paterni** | Karmaşık yerlerde ufak tefek detayları atlar | Talimatı yanlış anlıyorsa o yolda ısrar eder | Benzer kod tekrarları |
| **En iyi olduğu iş** | Mimari, yaratıcılık, karmaşık hata ayıklama | Doğrulama, sıfır-context inceleme | Boilerplate, yüksek hacim |
| **En kötü olduğu iş** | Basit tekrarlayan işler (bunalır) | Belirsiz problem tanımları (sorgulamaz) | Yeni konsept öğrenme |

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

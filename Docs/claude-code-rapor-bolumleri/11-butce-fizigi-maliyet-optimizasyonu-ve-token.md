## 11. Bütçe Fiziği: Maliyet Optimizasyonu ve Token Ekonomisi

### 11.1 "Pricing Physics" Gerçekleri

Maliyet sadece token başına fiyat değildir. Göz önünde bulundurmanız gereken 4 bileşen:

1. **Doğrudan token maliyeti:** Input + output token'ların toplamı
2. **Calibration Cost (Kalibrasyon Maliyeti):** Model değiştirdiğinizde o modelin kör noktalarını öğrenmek için harcanan zaman ve token. Sürekli model değiştirmek, küçük fiyat farklarından çok daha pahalıya mal olur.
3. **Hata düzeltme maliyeti:** Yanlış çıktının düzeltilmesi için harcanan zaman ve tekrar üretim.
4. **Fırsat maliyeti:** Ajandan beklerken kaybettiğiniz zaman (özellikle 24/7 fleet senaryosunda).

**Ana Kural: Model sadakati model yarışından daha değerlidir.** 3-6 ay aynı modeli kullanarak onun karakterini öğrenmek ve sistemleri o modele göre evrimleştirmek, her yeni çıkan "en iyi" modele atlamaktan daha yüksek verim sağlar.

### 11.2 Güncel Fiyatlandırma (Ağustos 2026)

#### Claude API Fiyatları ($/1M token)

Kaynak: Anthropic resmi fiyatlandırma ve third-party doğrulamaları (benchlm.ai, coursiv.io, devtk.ai, Ağustos 2026)

| Model | Base Input | Base Output | Cache Hit (5dk+ sonra) | Batch Input | Batch Output |
|:---|---:|---:|---:|---:|---:|
| Claude Fable 5 (yeni amiral gemisi) | **$10** | **$50** | $1.00 | $5.00 | $25.00 |
| Claude Opus 5 (en güçlü agentik) | **$5** | **$25** | $0.50 | $2.50 | $12.50 |
| Claude Sonnet 5 (promo) | **$2** | **$10** | $0.20 | $1.00 | $5.00 |
| Claude Sonnet 5 (normal, 1 Eylül 2026 sonrası) | $3 | $15 | $0.30 | $1.50 | $7.50 |
| Claude Sonnet 4.6 (eski) | $3 | $15 | $0.30 | $1.50 | $7.50 |
| Claude Haiku 4.5 (hızlı/hafif) | **$1** | **$5** | $0.10 | $0.50 | $2.50 |

**Not:** Tüm Claude modelleri **200K context penceresine** standart fiyattan sahiptir, uzun context için ekstra ücret alınmaz. Opus 5 için isteğe bağlı "fast mode" 6x fiyatla ($30/$150) çalışır.

#### Prompt Caching (%90'a Varan İndirim)
Claude Code, aynı system prompt'u ve büyük dosyaları tekrar tekrar göndermek yerine **otomatik olarak cache'ler.** Cache hit durumunda input token fiyatı %10'a düşer (örneğin Sonnet 5'te $2 yerine $0.20/MTok).

Cache kırılmaları pahalı olduğu için:
- CLAUDE.md'yi ve büyük statik dosyaları görev **sırasında** sık sık değiştirmeyin
- Büyük bir dosyayı bir kere okutup sonra referans verin
- Aynı oturumu `/compact` ile özetleyerek sürdürün, tamamen `/clear` ile sıfırlamayın

#### Batch API (%50 İndirim)
Acil olmayan, toplu işler (toplu test yazma, kod dönüşümü, dokümantasyon) için Batch API kullanın. Girdiyi toplu yollayın, 5 dakika-24 saat içinde sonuçları alın; hem input hem output **%50 ucuz**.

#### Abonelik Planları (Claude App + CLI)

| Plan | Aylık Ücret | Kullanım Alanı |
|:---|---:|:---|
| Free | $0 | Sınırlı Sonnet, deneme amaçlı |
| **Pro** | **$20/ay** | Bireysel geliştirici, haftada ~10-20 saat aktif kullanım (çoğu kişi için yeterli) |
| Max 5x | $100/ay | Tam zamanlı mühendis, günlük 4-8 saat Claude kullanımı |
| Max 20x | $200/ay | Yoğun agentik işler, Opus ağırlıklı kullanım. ~70M token/ay API eşdeğeri. |
| Team | $30/kullanıcı/ay | Ekip özellikleri, merkezi fatura |
| Team Premium | $100/koltuk/ay (5 kişilik min) | Kurumsal SSO, SLA, gelişmiş yönetim |
| Enterprise | Özel | Özel dağıtım, zero data retention, VPC |

#### Rakip Modeller Fiyat Karşılaştırması (referans)

| Model | Input/MTok | Output/MTok | Kullanım Alanı |
|:---|---:|---:|:---|
| GPT-5 (OpenAI) | $1.25 | $10 | Zero-context review, doğrulama |
| DeepSeek-V4 Flash | $0.14 | $0.28 | Çok ucuz, boilerplate/toplu iş |
| Gemini 2.5 Pro | $1.25 | $10 | Multimodal görevler |

### 11.3 Haftalık Örnek Maliyet Hesabı

**Orta boy bir feature, 50 saat aktif Claude kullanımı:**

| Yaklaşım | Maliyet | Not |
|:---|---:|:---|
| Tamamen Opus 5 ile | ~$225/hafta | Gereksiz pahalı |
| Tamamen Sonnet 5 ile (promo) | ~$30/hafta | İyi F/P |
| Tamamen Sonnet 5 (normal) | ~$45/hafta | Standart |
| **Karışık model (ÖNERİLEN)** | **~$35-40/hafta** | Daha iyi kalite + yüksek hız |

**Karışık model detayı:**
- Mimari/özel sorunlar (%10): Opus 5 → ~$22
- Ana geliştirme (%70): Sonnet 5 → ~$13
- Boilerplate/test/doküman (%20): Haiku 4.5 + caching → ~$2
- Cache hit'ler ile toplam %30 ek indirim → **~$26-30/hafta**

### 11.4 Maliyet Düşürme Taktikleri (Kanıtlanmış)

1. **Prompt Caching'i Maksimize Edin:**
   - CLAUDE.md'yi stabil tutun (değişirse cache bozulur)
   - Göreve başlamadan gerekli tüm dosyaları bir kere okutun
   - `/compact` kullanın (oturumu sıfırlamadan özetler, cache korunur)

2. **Doğru Modeli Doğru İşe Kullanın (Tiered Routing):**
   - Mimari/karar/zor hata → Opus 5
   - Normal kod yazma, özellik geliştirme → Sonnet 5 (en iyi F/P)
   - Test, dokümantasyon, basit refactor, özet → Haiku 4.5
   - Toplu dönüşümler, acil olmayan işler → Batch API
   - Çapraz doğrulama → GPT-5 / DeepSeek (farklı perspektif + ucuz)

   `model`i oturum içinde `/model` komutu ile anında değiştirebilirsiniz.

3. **Görevleri Uygun Boyuta Bölün:**
   10 saatlik tek bir görev yerine 2 saatlik 5 görev (her biri temiz context ile) hem daha ucuzdur hem de "dumb zone" riskini azaltır. Her ara görev sonunda checkpoint commit'i ile de güvenli olur.

4. **/effort Ayarını Kullanın:**
   ```
   /effort low     → Basit, hızlı işler (daha az token)
   /effort medium  → Normal işler (varsayılan)
   /effort high    → Zor problemler
   /effort xhigh   → Çok zor
   /effort max     → En zor, pahalı
   ```
   Basit bir işi `max` eforla yapmak overthinking'e ve gereksiz token tüketimine yol açar. Her görevin zorluğuna göre ayarlayın.

5. **Bütçe Limiti Koyun:**
   Script veya arka plan çalıştırmalarında mutlaka limit koyun:
   ```bash
   claude -p --max-budget-usd 2 "büyük test yazımı"
   claude -p --max-turns 6 "hata düzelt"
   ```
   Bu limitler aşıldığında Claude otomatik olarak durur.

6. **`.claudeignore` Kullanın:**
   Claude'un okuması gerekmeyen dosyaları (`node_modules`, büyük log dosyaları, binary'ler, build çıktıları) taramasını engelleyin. Gereksiz dosya tarama hem yavaşlatır hem de input token tüketir.

7. **Büyük Çıktıları Dosyaya Yazdırma:**
   "Bu fonksiyonu `src/auth/login.ts` içine yaz" demek, tüm kodu ekrana basmasından daha verimlidir. Dosyaya yazma komutu gereksiz konuşma token'larını ortadan kaldırır.

8. **Pro (Abonelik) + API Hibrit Kullanım:**
   Günlük interaktif kullanım Pro abonelikle (ayda $20 limitsiz sayılır), arka plan otomasyon/script/CI işleri için API kullanmak en maliyet etkin yaklaşımdır.

### 11.5 Tüketimi Takip Etme

#### Oturum İçi Takip
```
/usage          # Mevcut oturumun token ve maliyet özeti
```

Claude Code artık `/cost` yerine `/usage` kullanıyor, ancak `/cost` da halen çalışmaktadır.

#### Proje Bazlı Takip
Anthropic Console → Usage sekmesinden günlük/haftalık/aylık tüketimi grafiklerle görebilirsiniz. API için proje bazlı API key oluşturarak tüketimi ayırın.

#### Hard Limit
Console'dan aylık harcama limiti belirleyin (örn: $100). Limit aşıldığında otomatik olarak çağrılar durur. Bu özellikle production/CI otomasyonunda ani fatura sürprizlerini önler.

### 11.6 Pro ve API Karşılaştırması (Ne Zaman Hangisi?)

| Kullanım Senaryosu | Tavsiye | Neden |
|:---|:---|:---|
| Haftada <10 saat kullanım | Pro ($20/ay) | Ucuz, basit |
| Tam zamanlı geliştirici, Claude'u ana editör gibi kullanma | Max 5x ($100/ay) | Rate limit sizi kesmez |
| Haftada 70M+ token, yoğun Opus/agent kullanımı | Max 20x ($200/ay) | API'den ucuza gelir |
| CI/CD, cron job, arka plan ajanı | API | Esnek, otomasyona uygun |
| Takım halinde kullanım, SSO ihtiyacı | Team/Enterprise | Yönetim kolaylığı |
| Hem günlük kullanım + otomasyon | Pro ($20) + API (ayrı) | Hibrit en optimal |

**Tavsiyemiz:**
- **Başlangıç (L0-L2):** Sadece Claude Pro ($20) — her şey için yeterli.
- **L2-L3:** Pro + düşük bütçeli API (ayda ~$20-50).
- **L3-L4:** Max 20x + API (filo ve otomasyon için). Console'da mutlaka hard limit belirleyin.

### 11.7 Gerçek Dünya Karşılaştırması: Aylık Maliyet Beklentisi

| Olgunluk Seviyesi | Kurulum Tipi | Aylık Toplam Maliyet |
|:---|:---|---:|
| L0-L1 (Bireysel başlangıç) | Sadece Pro | $20 |
| L2 (Bireysel ustalık) | Pro + Biraz API | $30-50 |
| L3 (Takım seviyesi) | Max + API + Team | $150-300/kişi |
| L4 (Otonom fabrika, Fleet) | Max 20x + API + VPS | $300-800 (tek operatör) |
| L4 (3-5 kişilik AI-native takım) | Team Premium + API | $600-2000 (karşılığı ~15-30 kişilik çıktı) |

**ROI Perspektifi:** $500/ay bir mühendis için çok küçük bir maliyet iken, doğru kurulmuş bir agentic workflow ile tek bir kıdemli mühendisin çıktısı 3-5x artıyorsa geri dönüş 10-20 kattır.

---

### Hızlı Maliyet Kontrol Listesi

- [ ] `/.claudeignore` doğru yapılandırılmış
- [ ] Görevler uygun boyutta bölünmüş
- [ ] Basit işler için `/effort low` kullanılıyor
- [ ] Büyük işler için `--max-budget-usd` limiti koyuluyor
- [ ] Uygun model seçimi yapılıyor (Opus'ta ısrar edilmiyor)
- [ ] CLI'da `/usage` ile düzenli kontrol
- [ ] Anthropic Console'da hard limit belirlenmiş
- [ ] Cache bozacak sık CLAUDE.md değişikliklerinden kaçınılıyor

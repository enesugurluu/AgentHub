## 11. Bütçe Fiziği: Maliyet Optimizasyonu ve Token Ekonomisi

> **Fiyatlandırma son güncelleme:** 2026-08-09 (benchlm.ai 7 Ağu 2026, coursi.io 25 Tem 2026, Anthropic resmi fiyatlandırma sayfası).
> Sonnet 5 giriş fiyatı **31 Ağustos 2026'ya kadar $2/MTok** (promo), sonrası **$3/MTok**.

### 11.1 "Pricing Physics" Gerçekleri

Maliyet sadece token başına fiyat değildir. Göz önünde bulundurmanız gereken 4 bileşen:

1. **Doğrudan token maliyeti:** Input + output token'ların toplamı
2. **Kalibrasyon maliyeti:** Model değiştirdiğinizde o modelin kör noktalarını öğrenmek için harcanan zaman ve token
3. **Hata düzeltme maliyeti:** Yanlış çıktının sebep olduğu hataları düzeltmek için harcanan zaman
4. **Fırsat maliyeti:** Ajandan beklerken kaybettiğiniz zaman (özellikle 24/7 fleet senaryosunda)

**Kural: Model sadakati, model yarışından daha değerlidir.** Sürekli "en yeni en iyi model"e atlamak yerine 3-6 ay aynı modeli kullanarak onun karakterini öğrenmek ve sistemleri o modele göre ayarlamak daha yüksek verim sağlar.

### 11.2 Token Fiyat Matematiği (Ağustos 2026)

| Model | Input/MTok | 5 dk cache yazma | 1 sa cache yazma | Cache okuma | Output/MTok | En iyi kullanım |
|:---|---:|---:|---:|---:|---:|:---|
| **Claude Fable 5** | $10.00 | $12.50 | $20.00 | $1.00 | $50.00 | En zor mimari, yenilikçi ARGE, akıl yürütme sınırında |
| **Claude Opus 5** | $5.00 | $6.25 | $10.00 | $0.50 | $25.00 | Karmaşık ajan görevleri, production kod, ana inceleme (Opus 4.6/4.7/4.8 ile aynı fiyat) |
| **Claude Sonnet 5 (promo)** | $2.00 | $2.50 | $4.00 | $0.20 | $10.00 | **31 Ağu 2026'ya kadar** — ana geliştirme (en iyi F/P) |
| **Claude Sonnet 5 (normal)** | $3.00 | $3.75 | $6.00 | $0.30 | $15.00 | Ana geliştirme, 1 Eylül 2026+ |
| **Claude Haiku 4.5** | $1.00 | $1.25 | $2.00 | $0.10 | $5.00 | Sınıflandırma, extract, özet, boilerplate, routing |
| Claude Opus 5 Fast Mode | $10.00 | — | — | — | $50.00 | Düşük gecikmeli Opus (2x fiyat) |
| inference_geo=us (tüm katmanlar) | × 1.1 | × 1.1 | × 1.1 | × 1.1 | × 1.1 | Veri ABD'de kalsın zorunluluğu |

**Batch API (tüm modellerde %50 indirim, 24 saat içinde döner):**

| Model | Batch input | Batch output |
|:---|---:|---:|
| Fable 5 | $5.00 | $25.00 |
| Opus 5 | $2.50 | $12.50 |
| Sonnet 5 | $1.50 | $7.50 |
| Haiku 4.5 | $0.50 | $2.50 |

**Abonelik Planları:**

| Plan | Aylık | Not |
|:---|---:|:---|
| Free | $0 | Sınırlı Sonnet |
| Pro | $20 | Orta düzey kullanım, web+CLI |
| Max 5x | $100 | 5x Pro limiti |
| Max 20x | $200 | 20x Pro — ayda ~70M token API'ye denk kullanım sağlar |
| Team | $100/kullanıcı/ay (en az 5) | Admin paneli, SSO, merkezi fatura |

#### Örnek Hesap (1 haftalık geliştirme)

```
Senaryo: Orta boy bir feature, haftada ~50 saat aktif Claude kullanımı.
Cache isabet oranı: %60 (sağlam CLAUDE.md + tekrar eden sistem promptu).

Karışık model + cache (ideal):
  Mimari/özel sorun: %10 Opus 5 → 0.5*5 + 0.2*25 = $7.5
  Ana geliştirme:    %70 Sonnet 5 (promo $2/$10) → 3.5*2 + 1.4*10 = $21
  Boilerplate/test:  %20 Haiku 4.5 → 1*1 + 0.4*5 = $3.0
  Cache hit üzerinden tasarruf: ~%35-40
  TOPLAM: ~$20-25/hafta

Aynı işi saf Opus 5 ile, cache kullanmadan:
  5*5 + 2*25 = $75/hafta — gereksiz pahalı.
```

### 11.3 Maliyet Düşürme Manivelaları (Etki Sırasıyla)

1. **Cache isabet oranını maksimize edin (en büyük kaldıraç).**
   - Claude Code otomatik prompt caching kullanır, ama siz sistemi düzenlerseniz isabet yükselir:
   - CLAUDE.md'yi sık değiştirmeyin (değişirse prefix cache bozulur).
   - Büyük dosyaları oturum başında **bir kere** okutup sonra referans verin.
   - 5 dakikalık kısa TTL cache canlı oturumlar için, 1 saatlik TTL uzun görevler için daha ucuz.
   - Cache okuma maliyeti input'un sadece %10'u (Haiku'da $0.10, Opus 5'te $0.50).

2. **Toplu/asinchron işleri Batch API'ye yönlendirin.**
   - Dokümantasyon toplu çeviri, eval seti cevaplama, geniş repo tarama, günlük özet: %50 indirim.
   - Yanıt 24 saat içinde, ama 1 dakikada da dönebilir.

3. **Fast Mode'u sadece gerçek zamanlı etkileşimde kullanın.**
   - Tüm fiyatları 2x yapar. Elle interaktif çalışırken gerekiyorsa aç, arka planda kapat.

4. **Modeli göre seçin (routing):**
   - `/effort low` → Haiku/Sonnet kullan; boilerplate ve arama için idealdir.
   - `/effort high/xhigh` → Opus 5; mimari, karmaşık bug.
   - `/effort max` → Fable 5; sınır durumları, en zor problemler.

5. **`/compact` kullanım alışkanlığı.**
   - Uzun oturumlarda eski konuşma şişer; `/compact` bağlamı özetleyerek gelecekteki input token'ları azaltır.
   - Oturum içinde `/usage` ile anlık tüketimi takip edin (eski `/cost` yerine).

6. **`.claudeignore` ile tarama kapsamını daraltın:**
   ```
   node_modules/  .next/  dist/  *.log  *.lock  .env*
   coverage/  __pycache__/  build/  target/  .venv/
   ```

7. **Büyük çıktıları dosyaya yazdırın** (ekrana dökmeyin — formatlama token'ı yer).

8. **Uzun görevleri bölün.** 10 saatlik tek görev yerine 2 saatlik 5 görev (her temiz context) hem daha ucuz hem daha az hata.

9. **`--max-budget-usd` ve `--max-turns` ile üst sınır koyun** (özellikle `--bg` arka plan ajanları için):
   ```bash
   claude --bg --max-budget-usd 5 --max-turns 50 "Günlük güvenlik denetimi yap"
   ```

10. **US-only inference coğrafi çarpanını gereksiz açmayın.** Mevzuat zorunlu değilse `inference_geo` ayarını varsayanda bırakın (1.1x maliyet farkı).

### 11.4 Tüketimi Takip Etme

**Oturum içi:**
```
> /usage
Oturum: 45 dakika
Input tokens: 245.000 (cache hit %58)
Output tokens: 68.000
Toplam maliyet: ~$2.34
```

**Anthropic Console → Usage** sekmesinden günlük/haftalık tüketimi takip edin ve **hard budget limiti** belirleyin (ayda $X'i geçerse API çağrıları dursun).

**Proje bazlı takip (script):**
```bash
#!/bin/bash
# ~/bin/cost-log.sh — Claude oturumları için basit CSV log
LOG="$HOME/costs/cost-$(date +%Y-%m).csv"
mkdir -p "$HOME/costs"
[ -f "$LOG" ] || echo "tarih,proje,input_cache_hit,input,output,model,usd" > "$LOG"
echo "$(date +%Y-%m-%d),$(basename $(pwd)),$@" >> "$LOG"
```

### 11.5 Plan Karşılaştırması (Güncel)

| Seçenek | Aylık | Avantaj | Dezavantaj |
|:---|---:|:---|:---|
| Free | $0 | Başlangıç, deneme | Çok sınırlı |
| Pro | $20 | Limitsiz (rate limitli), web+CLI | Rate limit sıkı, API cotası yok |
| Max 5x | $100 | Yüksek limit | API'ye göre pahalı olabilir |
| Max 20x | $200 | 70M token/ay denk kullanım | Pahalı |
| Team | $500+/5 kişi | SSO, admin, merkezi politika | Sadece takım |
| API (kullanım bazlı) | $20-$200+ | Esnek, otomasyon, Batch/Cache, CI/CD | Ani yükseliş riski (budget ile korunur) |
| **Hibrit (Pro + API)** | $40-$100 | Günlük etkileşim Pro; otomasyon/fleet API | İki ayrı sistem |

**Tavsiye:**
- **L0-L2:** Pro ($20) yeterli.
- **L3-L4:** Hibrit — gündüz Pro; fleet/otomasyon/CI için API (Console'da hard limit ile).
- **5+ kişilik takım:** Team planı + plugin/merkezi `.claude/settings.json` dağıtımı.

### 11.6 Cache Matematiği (Gerçek Dünya Örneği)

Opus 5'te uzun bir ajan oturumu (120.000 token'lık sabit prefix, 40 tur):

| Kalem | Token | Birim | Tutar |
|:---|---:|:---|---:|
| Cache write (5dk TTL, bir kere) | 120.000 | $6.25/MTok | $0.75 |
| Cache reads (39 tur) | 4.680.000 | $0.50/MTok | $2.34 |
| Yeni input (turn başı 1.500 × 40) | 60.000 | $5.00/MTok | $0.30 |
| Output (2.500 × 40) | 100.000 | $25.00/MTok | $2.50 |
| **Toplam** | | | **$5.89** |
| Aynı iş cache'siz (input'u her tur tekrar): | 4.860.000 | $5.00/MTok | **$26.80** |
| **Cache tasarrufu:** | | | **~%78** |

Bu tek başına neden CLAUDE.md'yi stabil tutup büyük dosyaları bir kere okutmanız gerektiğini açıklıyor.

---

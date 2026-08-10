## 13. Ölçümleme, Metrikler ve Sürekli İyileştirme

### 13.1 Ölçülecek Metrikler

Sistemin geliştiğini kanıtlamak için metrik gerekir. İşte tutulması faydalı metrikler:

#### Verimlilik Metrikleri
- **AI destekli teslim hızı:** Haftada kapanan story point / takım üyesi
- **PR büyüklüğü:** Ortalama PR satır sayısı (küçük PR = daha hızlı review, ideal <300 satır)
- **İlk geçen test oranı:** İlk denemede geçen test yüzdesi
- **AI tarafından yazılan kod oranı:** Sonradan elle değiştirilmeyen satır oranı (yaklaşık)
- **Otonom görev oranı:** Hiç insan müdahalesi olmadan tamamlanan görev yüzdesi

#### Kalite Metrikleri
- **Post-merge bug oranı:** Merge sonrası bulunan bug / KLOC (bin satır kod)
- **Review dönüş sayısı:** PR'ın merge olmadan önce kaç revizyon geçirdiği
- **Test coverage'daki değişim:** AI kullanımıyla coverage artış/azalışı
- **Sentry hata sayısı:** Zaman içinde production hata oranındaki değişim
- **Build başarısı oranı:** CI/CD ilk denemede geçme yüzdesi

#### Operasyonel Metrikler
- **Token maliyeti / story point:** Başarıya göre normalize edilmiş maliyet
- **Ortalama görev süresi:** Ajan başlangıcından PR'a kadar geçen zaman
- **İnsan müdahale sayısı:** Ajan görevi başına insan müdahale adedi
- **Cache hit oranı:** Prompt caching verimliliği (maliyetle doğrudan ilgili)
- **Ajan uptime:** Fleet'in çalışma süresi ve kesinti oranı

### 13.2 Yerleşik Ölçüm Araçları

**`/usage` Komutu:** Oturum bazlı token tüketimi ve maliyet.
```
> /usage
Total cost: $2.34
Duration: 45 dakika
Input tokens: 245,000
Output tokens: 68,000
Cache read tokens: 120,000 (ücret %10'dan)
```

**`claude agents` Komutu:** Tüm çalışan, bloke olan ve tamamlanan arka plan ajanlarının durumunu gösterir.

**Anthropic Console:**
- Günlük/haftalık/aylık tüketim grafikleri
- API key bazında tüketim ayırma
- Hard limit belirleme (örn: ayda $200'yi geçerse dur)
- Cache hit oranı ve hata oranları

**Team/Enterprise Plan:**
- Kullanıcı başına tüketim
- Takım metrikleri ve yönetim paneli
- Denetim kayıtları (audit log)

### 13.3 Basit Ölçüm Dashboard
Pahalı araçlara gerek yok; Notion, Google Sheets veya basit bir markdown dosyası ile başlayın:

**`metrics/weekly-report.md`:**
```markdown
# Haftalık AI Metrik Raporu — Hafta 32, 2026

## Verimlilik
- Kapatılan story point: 42 (önceki hafta: 35, AI öncesi ortalama: 12)
- Ortalama PR büyüklüğü: 180 satır (ideal <300)
- AI'nın ürettiği ve elle değişmeyen kod: ~%75
- Otonom görev oranı: %60

## Kalite
- Post-merge bug: 2 (önceki 5)
- Ortalama review döngüsü: 1.2 revizyon (önceki 2.8)
- Test coverage: %78 → %83
- Sentry haftalık hata: 8 (önceki 22)
- Build ilk geçme oranı: %75 (önceki %50)

## Operasyon
- Token maliyeti: $67/hafta
- Cache hit oranı: %55 (hedef: %70+)
- İnsan müdahale/görev: 0.8 (daha iyi)
- Fleet uptime: %99.5
```

### 13.4 A/B Test ve Model Karşılaştırması

Hangi prompt/ayar/modelin daha iyi olduğunu görmek için kontrollü deney yapın:

**Basit A/B metodu:**
1. Bir görev seçin ve iki özdeş worktree oluşturun
2. Ayar A (örn: Sonnet 5 + TDD skill) ile bir worktree'de çalıştırın
3. Ayar B (örn: Haiku + basit prompt) ile diğer worktree'de çalıştırın
4. Şunları karşılaştırın:
   - Testleri ilk denemede geçme oranı
   - Toplam tur sayısı
   - Toplam token maliyeti (/usage)
   - Nihai kod kalitesi (insan review)
   - Hata sayısı
5. Kazanan ayarı sistem varsayılanı yapın.

### 13.5 Eval Seti ve Kalite Ölçüm Sistemi

Sistemin iyileşip iyileşmediğini ölçmek için sabit bir eval seti kullanın:

**`evals/eval-set.md`:**
```markdown
# Eval Soru Seti
# Ajan cevapları bu sorularda doğru/yarı/yanlış olarak skorlanır

## Kategori: Mimari Bilgi
1. Redis kullanımından neden vazgeçtik? (Beklenen: INCIDENT-142, ADR-007, BullMQ)
2. Auth0 ne zaman kullanılmalı?

## Kategori: Kod Standartları
3. Yeni bir API endpoint için hangi adımlar zorunlu?
4. Migration nasıl oluşturulur? Elle düzenlemek neden yasak?

## Kategori: Sorun Çözme
5. P2002 Prisma hatası alınırsa ilk kontrol edilecekler?
6. Next.js'te "use client" hangi durumlarda gerekir?
```

**Periyodik ölçüm:**
- Haftada bir temiz bir Claude oturumunda bu soruları sorun
- Cevapları doğru/yarı/yanlış skorlayın
- Hafıza sistemi (graph, skills, rules) geliştikçe skor yükselmelidir
- Hedef: %90+ doğru cevap oranı
- Skor düştüğünde hangi kural/bilginin kaybolduğunu analiz edin ve eksik dokümantasyonu ekleyin.

### 13.6 Haftalık Retrospektif

Her hafta 30 dakikalık bir "AI workflow retro" yapın:
1. Bu hafta hangi AI görevleri iyi gitti? Neden? → Skill/rule olarak kalıcılaştırın
2. Hangi görevler başarısız oldu veya çok müdahale gerektirdi? Neden? → Hook/kural eksiğini tespit edin
3. Hangi prompt'lar tekrar kullanılmaya değer? → Skill dosyasına dönüştürün
4. CLAUDE.md veya rules dosyalarına eklenmesi gereken yeni kurallar var mı?
5. Graf hafızasına eklenecek yeni entity/edge var mı?
6. Maliyet normal mi? Anormal artış var mı?
7. Önümüzdeki hafta hangi otomasyonu ekleyebilirsiniz?

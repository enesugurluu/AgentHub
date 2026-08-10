## 13. Ölçümleme, Metrikler ve Sürekli İyileştirme

### 13.1 Ölçülecek Metrikler

Sistemin geliştiğini kanıtlamak için metrik gerekir. İşte tutulması faydalı metrikler:

#### Verimlilik Metrikleri
- **AI destekli teslim hızı:** Haftada kapanan story point / takım üyesi
- **PR büyüklüğü:** Ortalama PR satır sayısı (küçük PR = daha hızlı review)
- **İlk geçen test oranı:** İlk denemede geçen test yüzdesi
- **AI tarafından yazılan kod oranı:** Sonradan elle değiştirilmeyen satır oranı (yaklaşık)

#### Kalite Metrikleri
- **Post-merge bug oranı:** Merge sonrası bulunan bug sayısı / KLOC
- **Review dönüş sayısı:** PR'ın merge olmadan önce kaç revizyon geçirdiği
- **Test coverage'daki değişim:** AI kullanımıyla coverage artış/azalış hızı
- **Sentry hata sayısındaki değişim:** Zaman içinde production hata oranı

#### Operasyonel Metrikler
- **Token maliyeti / story point:** Başarıya göre normalize edilmiş maliyet
- **Ajan başarı oranı:** Verilen görevlerin insan müdahalesi olmadan tamamlanma yüzdesi
- **Ortalama görev tamamlama süresi:** Ajan başlangıcından PR'a kadar geçen zaman
- **Müdahale sayısı:** Ajan görevi başına insan müdahale adedi

### 13.2 Basit Ölçüm Dashboard (Spreadsheet/Notion)

Pahalı araçlara gerek yok; Notion, Google Sheets veya basit bir markdown dosyası ile başlayın:

**`metrics/weekly-report.md`:**
```markdown
# Haftalık AI Metrik Raporu — Hafta 32, 2026

## Verimlilik
- Kapatılan story point: 42 (önceki hafta: 35, AI öncesi ortalama: 12)
- Ortalama PR büyüklüğü: 180 satır (ideal <300)
- AI'nın ürettiği ve elle değişmeyen kod: ~75%

## Kalite
- Post-merge bug: 2 (önceki 5)
- Ortalama review döngüsü: 1.2 revizyon (önceki 2.8)
- Test coverage: %78 → %83

## Operasyon
- Token maliyeti: $67/hafta
- Ajan başarı oranı: %60'ı tam otonom, %30'u hafif müdahale, %10'u elle al
- Toplam 7 otonom PR açıldı, 5'i doğrudan merge oldu
```

### 13.3 A/B Test ve Model Karşılaştırması

Bazen hangi modelin/prompt'un daha iyi olduğunu görmek için kontrollü deney yapın:

**Basit A/B metodu:**
1. Bir görev seçin ve bir özdeş kopya oluşturun (aynı TASK.md)
2. Ayar A ile (örn: Claude Sonnet) bir worktree'de çalıştırın
3. Ayar B ile (örn: DeepSeek) başka bir worktree'de çalıştırın
4. Şunları karşılaştırın:
   - Testleri ilk denemede geçme oranı
   - Toplam tur sayısı
   - Toplam token maliyeti
   - Nihai kod kalitesi (insan review)
   - Hata sayısı

### 13.4 Yerleşik Ölçüm Araçları

Claude Code'un kendi ölçüm komutları ile sıfır entegrasyonla başlayabilirsiniz:

| Komut/araç | Ne ölçer? |
|:---|:---|
| `/usage` | Oturum içi input/output token ve yaklaşık USD maliyet (eski `/cost`). Cache isabet oranını da gösterir. |
| `claude agents` | Çalışan/tamamlanan/engellenmiş ajanlar; her ajan için süre, tur, tahmini maliyet. |
| `/status` | O anki oturum durumu (model, effort, kullanım). |
| `claude doctor` | Kurulum ve ayar sağlık raporu. |
| Anthropic Console → Usage | Hesap bazlı günlük/haftalık kullanım; hard budget limiti buradan belirlenir. |

Otomasyon ve filo ölçümü için `claude -p -o json` ile çıktıyı programatik olarak alıp CSV/veritabanına yazabilirsiniz.

### 13.5 Retrospektif ve Sürekli İyileştirme

Haftalık 30 dakikalık bir "AI workflow retro" yapın:
1. Bu hafta hangi AI görevleri iyi gitti? Neden?
2. Hangi görevler başarısız oldu veya çok müdahale gerektirdi? Neden?
3. Hangi prompt'lar/kalıplar tekrar kullanılmaya değer? → skills/ dizinine ekle
4. CLAUDE.md'ye eklenmesi gereken yeni kurallar/tecrübeler var mı?
5. Graf hafızasına eklenecek yeni entity/edge var mı?

---


# CLAUDE CODE WORKFLOW — SENIOR LEAD KURULUM RAPORU (ANA PARÇALAR)

Bu klasör, ana raporun **16 ana bölüm + 2 ek** olarak büyük parçalara ayrılmış,
**Ağustos 2026** itibarıyla 4 paralel web araştırma ajanı ile doğrulanmış ve
kalite puanı 9 altında kalan bölümleri güncellenmiş halidir.

## Kalite Döngüsü Metodolojisi

Her bölüm için:
1. Mevcut içerik okundu.
2. Paralel web araştırması ile doğruluk kontrolü (maksimum 4 paralel ajan).
3. Dört kritere göre 10 üzerinden puan verildi:
   - Doğruluk (4 puan) — komut/fiyat/özellik güncelliği
   - Kapsam (3 puan) — 2026 ana özellikleri (MCP, Skills, Hooks, Subagents, Agent Teams, Plugins, `--bg`, `--worktree`) içeriyor mu?
   - Pratiklik (2 puan) — kod/komutlar çalıştırılabilir mi?
   - Stratejik değer (1 puan) — Senior lead seviyesinde derinlik var mı?
4. **Puan < 9 olan bölümler güncellendi**; ≥9 olanlara dokunulmadı.

## Bölüm Listesi (Güncel Puanlar)

| # | Dosya | Başlık | Puan | Durum |
|---|---|---|---:|---|
| 0 | [KALITE_DOGRULUK_RAPORU.md](./KALITE_DOGRULUK_RAPORU.md) | Kalite Değerlendirme ve Doğruluk Raporu | — | 🆕 |
| 1 | [00-KAPAK-ve-ICINDEKILER.md](./00-KAPAK-ve-ICINDEKILER.md) | Kapak, Belge Bilgisi, İçindekiler | 9.5 | ✅ |
| 2 | [01-YONETICI-OZETI.md](./01-YONETICI-OZETI.md) | Yönetici Özeti | 9.0 | ✅ |
| 3 | [02-OLGUNLUK-MODELI-L0-L4.md](./02-OLGUNLUK-MODELI-L0-L4.md) | Olgunluk Modeli: L0→L4 | **9.1** | 🆕 güncellendi |
| 4 | [03-FAZ0-TEMEL-KURULUM.md](./03-FAZ0-TEMEL-KURULUM.md) | Faz 0: Temel Kurulum ve Çevre | **9.2** | 🆕 yeniden yazıldı |
| 5 | [04-FAZ1-BIREYSEL-USTALIK.md](./04-FAZ1-BIREYSEL-USTALIK.md) | Faz 1: Bireysel Ustalık (Hafta 1-4) | **9.0** | 🆕 güncellendi |
| 6 | [05-FAZ2-TAKIM-BENIMSEME.md](./05-FAZ2-TAKIM-BENIMSEME.md) | Faz 2: Takım Benimseme (Ay 2-3) | **9.0** | 🆕 güncellendi |
| 7 | [06-FAZ3-OTONOM-YAZILIM-FABRIKASI.md](./06-FAZ3-OTONOM-YAZILIM-FABRIKASI.md) | Faz 3: Otonom Yazılım Fabrikası (Ay 3-6) | **9.0** | 🆕 güncellendi |
| 8 | [07-MODEL-KARAKTERI-PROMPT.md](./07-MODEL-KARAKTERI-PROMPT.md) | Model Karakteri ve Prompt Mühendisliği | **9.0** | 🆕 güncellendi |
| 9 | [08-CONTEXT-VE-HAFIZA-GRAF.md](./08-CONTEXT-VE-HAFIZA-GRAF.md) | Context ve Graf Hafıza Mimarisi | 9.0 | ✅ |
| 10 | [09-DETERMINISTIK-KONTROL-QUALITY-GATES.md](./09-DETERMINISTIK-KONTROL-QUALITY-GATES.md) | Deterministik Kontrol ve Quality Gate'ler | 9.0 | ✅ |
| 11 | [10-PARALEL-AGENT-ORKESTRASYONU.md](./10-PARALEL-AGENT-ORKESTRASYONU.md) | Paralel Ajan Orkestrasyonu ve Worktree | **9.1** | 🆕 güncellendi |
| 12 | [11-BUTCE-FIZIGI-MALIYET.md](./11-BUTCE-FIZIGI-MALIYET.md) | Bütçe Fiziği: Maliyet Optimizasyonu | **9.3** | 🆕 yeniden yazıldı |
| 13 | [12-GUVENLIK-RISK-UYUMLULUK.md](./12-GUVENLIK-RISK-UYUMLULUK.md) | Güvenlik, Risk ve Uyumluluk | **9.2** | 🆕 yeniden yazıldı |
| 14 | [13-OLCUMLEME-METRIKLER.md](./13-OLCUMLEME-METRIKLER.md) | Ölçümleme, Metrikler ve Sürekli İyileştirme | **9.0** | 🆕 güncellendi |
| 15 | [14-YOL-HARITASI-30-60-90.md](./14-YOL-HARITASI-30-60-90.md) | 30/60/90 Günlük Yol Haritası | **9.1** | 🆕 yeniden yazıldı |
| 16 | [15-ACIL-DURUM-PLAYBOOKLARI.md](./15-ACIL-DURUM-PLAYBOOKLARI.md) | Acil Durum Playbook'ları | **9.0** | 🆕 güncellendi |
| 17 | [16-SONUC-VE-GELECEK.md](./16-SONUC-VE-GELECEK.md) | Sonuç ve Gelecek Vizyonu | 9.5 | ✅ |
| 18 | [97-EK-A-HIZLI-BASLANGIC-KOMUTLAR.md](./97-EK-A-HIZLI-BASLANGIC-KOMUTLAR.md) | Ek A: Hızlı Başlangıç Komut Özeti | **9.4** | 🆕 yeniden yazıldı |
| 19 | [98-EK-B-DIZIN-YAPISI.md](./98-EK-B-DIZIN-YAPISI.md) | Ek B: Tavsiye Edilen Dizin Yapısı | **9.3** | 🆕 yeniden yazıldı |

## Doğrulanan Kaynaklar (Ağustos 2026)

- Anthropic resmi dokümantasyonu (code.claude.com/docs)
- Claude Code CLI referans rehberleri (dev.to, eesel.ai, benchtlm.ai, coursi.io)
- Claude Code uzantı katmanları: CLAUDE.md / Skills / MCP / Hooks / Subagents / Agent Teams / Plugins kararlaştırma matrisi
- Fiyatlandırma: Fable 5, Opus 5, Sonnet 5 (promo/normal), Haiku 4.5, Cache katmanları, Batch API %50, Fast Mode 2x, inference_geo 1.1x
- Güvenlik: v2.1.90 ile düzeltilen 50-subcommand deny bypass (ADVISORY-CC-2026-002) ve SOCKS5 sandbox bypass; PreToolUse hook referansı; destructive_command_guard (dcg)
- Orkestrasyon araçları: Claude Squad, Vibe Kanban, Composio AO, Conductor, Parallel Code, Bernstein, Emdash, Baton, Nimbalyst, OMC
- CLI komutları: native installer, `claude doctor`, `/usage`, `/effort`, `/skills`, `--worktree`, `--bg`, `claude agents`, `/schedule`, `/model`, `--max-budget-usd`, `--max-turns`

Tüm bölümler artık **9.0 veya üstü** puanla Ağustos 2026 için doğrulanmış durumdadır.

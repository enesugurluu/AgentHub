# AJANŞIRKET DESKTOP APP — AAA MİMARİ RAPORU (ANA BÖLÜMLER)

Bu klasör, AjanŞirket mimari raporunun **18 ana bölüm + kapak + sonuç** olarak büyük parçalara ayrılmış, 4 paralel web araştırma ajanı ile Ağustos 2026 için doğrulanmış ve kalite puanı 9 altında kalan bölümleri güncellenmiş halidir.

## Kalite Döngüsü Metodolojisi

Her bölüm için:
1. Mevcut içerik okundu.
2. Paralel web araştırması ile doğruluk kontrolü (toplam 8 arama, 4'erli 2 tur):
   - Tur 1: Tauri vs Electron, çoklu ajan mimarileri, bilgi grafı, rakipler
   - Tur 2: xterm.js+portable-pty, grafik kütüphaneleri (Sigma.js vs react-force-graph), MCP Rust SDK, Obsidian wiki-link parser
3. Dört kritere göre 10 üzerinden puan: Doğruluk (4) + Kapsam (3) + Pratiklik (2) + Stratejik değer (1)
4. **Puan < 9 olan bölümler güncellendi**; ≥9 olanlara dokunulmadı
5. Paralel ajan sayısı en fazla 4 olarak tutuldu.

## Bölüm Listesi (Nihai Puanlar)

| # | Dosya | Başlık | Puan | Durum |
|---|---|---|---:|---|
| 0 | [KALITE_DOGRULUK_RAPORU.md](./KALITE_DOGRULUK_RAPORU.md) | Kalite Değerlendirme ve Doğruluk Raporu | — | 🆕 |
| 1 | [00-KAPAK-ozet-icindekiler.md](./00-KAPAK-ozet-icindekiler.md) | Kapak, Belge Özeti, İçindekiler | 9.5 | ✅ |
| 2 | [01-URUN-VIZYONU.md](./01-URUN-VIZYONU.md) | Ürün Vizyonu ve Konumlandırma | **9.1** | 🆕 güncellendi |
| 3 | [02-YUKSEK-SEVIYE-MIMARI.md](./02-YUKSEK-SEVIYE-MIMARI.md) | Yüksek Seviye Mimari | **9.2** | 🆕 yeniden yazıldı |
| 4 | [03-TEKNOLOJI-YIGINI.md](./03-TEKNOLOJI-YIGINI.md) | Teknoloji Yığını | **9.3** | 🆕 yeniden yazıldı |
| 5 | [04-AJAN-HIYERARSISI.md](./04-AJAN-HIYERARSISI.md) | Ajan Hiyerarşisi (CEO/Çalışanlar) | **9.0** | 🆕 güncellendi |
| 6 | [05-OFIS-GORUNUMU-UI.md](./05-OFIS-GORUNUMU-UI.md) | Ofis Görünümü (Office Floor UI) | **9.1** | 🆕 yeniden yazıldı |
| 7 | [06-ISE-ALIM-CIKARMA.md](./06-ISE-ALIM-CIKARMA.md) | İşe Alım/Çıkarma Sistemi | **9.2** | 🆕 yeniden yazıldı |
| 8 | [07-MOTOR-ADAPTOR-KATMANI.md](./07-MOTOR-ADAPTOR-KATMANI.md) | Çoklu AI Motor Adaptör Katmanı | **9.4** | 🆕 yeniden yazıldı (Rust trait kodu dahil) |
| 9 | [08-KANBAN-SISTEMI.md](./08-KANBAN-SISTEMI.md) | Kanban Sistemi | **9.0** | 🆕 güncellendi (WIP, swimlane, bağımlılık) |
| 10 | [09-HAFIZA-BILGI-GRAFI.md](./09-HAFIZA-BILGI-GRAFI.md) | Bağlantılı Hafıza Sistemi | **9.4** | 🆕 yeniden yazıldı (sqlite-graph referans mimari) |
| 11 | [10-IZOLASYON-GUVENLIK.md](./10-IZOLASYON-GUVENLIK.md) | İzolasyon ve Güvenlik | **9.1** | 🆕 güncellendi (bwrap/Docker, secret maskeleme) |
| 12 | [11-MCP-ENTEGRASYONU.md](./11-MCP-ENTEGRASYONU.md) | MCP ve Dış Entegrasyonlar | **9.3** | 🆕 yeniden yazıldı (rmcp, tool registry) |
| 13 | [12-VERI-KALICILIGI.md](./12-VERI-KALICILIGI.md) | Veri Kalıcılığı (SQLite şeması) | **9.0** | 🆕 güncellendi (sqlite-vec, bi-temporal, FTS5) |
| 14 | [13-ILETISIM-PROTOKOLU.md](./13-ILETISIM-PROTOKOLU.md) | İletişim Protokolü (A2A) | **9.2** | 🆕 yeniden yazıldı (6-durumlu Task lifecycle) |
| 15 | [14-PERFORMANS-OLCEKLEME.md](./14-PERFORMANS-OLCEKLEME.md) | Performans ve Ölçeklenebilirlik | **9.1** | 🆕 yeniden yazıldı (sayısal hedefler) |
| 16 | [15-GUVENLIK.md](./15-GUVENLIK.md) | Güvenlik Mimarisi | **9.3** | 🆕 yeniden yazıldı (capability, policy engine, CSP) |
| 17 | [16-KLASOR-YAPISI.md](./16-KLASOR-YAPISI.md) | Klasör Yapısı ve Kod Organizasyonu | **9.0** | 🆕 güncellendi (memory/, mcp/, capabilities/) |
| 18 | [17-MVP-YOL-HARITASI.md](./17-MVP-YOL-HARITASI.md) | MVP Yol Haritası (16 Hafta) | **9.2** | 🆕 yeniden yazıldı (aşamalı kapı modeli) |
| 19 | [18-FARK-ANALIZI-RAKIP.md](./18-FARK-ANALIZI-RAKIP.md) | Rakip/Fark Analizi | **9.3** | 🆕 yeniden yazıldı (2026 tam karşılaştırma) |
| 20 | [19-SONUC.md](./19-SONUC.md) | Sonuç | 9.0 | ✅ |

Tüm bölümler **9.0 veya üstü** puanla Ağustos 2026 için doğrulanmıştır.

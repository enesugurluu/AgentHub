# AJANSIRKET — AI AGENT ŞİRKETİ MİMARİSİ
## AAA Seviye Profesyonel Desktop App Sistem Raporu

**Proje:** Agent Company (Çalışma Adı: *"AjanŞirket"*)
**Seviye:** AAA / Production-Grade Mimari
**Versiyon:** 1.0
**Tarih:** 2026-08-09
**Durum:** Kapsamlı Mimari Tasarım

---

## BELGE ÖZETİ

Bu rapor, kullanıcının istediği özellikleri tam olarak kapsayan, CEO merkezli, çoklu ajanlı bir desktop uygulamanın mimarisini sunar:

1. ✅ **CEO Ajan:** Hiyerarşik orkestrasyon, strateji ve koordinasyon
2. ✅ **Uzman Çalışan Ajanlar:** Rol bazlı özelleşmiş ajanlar (CTO, Backend Dev, Frontend Dev, QA, DevOps, PM vb.)
3. ✅ **Ofis Görünümü:** Tüm ajanların ve durumlarının görsel olarak izlenebildiği interaktif ofis panosu
4. ✅ **İşe Alım / Çıkış (Hire/Fire):** Ajan ekleme/çıkarma, motor atama ve özelleştirme sistemi
5. ✅ **Çoklu AI Motor Desteği:** Claude Code, Codex CLI, Gemini CLI, OpenCode, Aider, Cursor CLI, Copilot CLI, Qwen Code vb.
6. ✅ **Kanban Sistemi:** CEO ve çalışanların görevlerinin takibi, sütunlar ve kartlar
7. ✅ **Bağlantılı Hafıza (Obsidian Tarzı):** Bidirectional linkli bilgi grafı, düğümler arası ilişkiler, görsel gezinti
8. ✅ **Desktop Uygulama:** Tauri 2.x + React tabanlı, çapraz platform, hafif ve güvenli

Mimari endüstri standardı çoklu ajan paternlerini (Hierarchical Orchestrator-Worker), production desktop teknolojilerini ve açık standartları (MCP, A2A, Git Worktree) temel alır.

---

## İÇİNDEKİLER

1. [Ürün Vizyonu ve Konumlandırma](#1-ürün-vizyonu)
2. [Sistem Mimarisi — Yüksek Seviye Bakış](#2-yüksek-seviye-mimari)
3. [Teknoloji Yığını Gerekçelendirme](#3-teknoloji-yığını)
4. [CEO ve Çalışan Ajan Hiyerarşisi](#4-ajan-hiyerarşisi)
5. [Ofis Görünümü (Office Floor UI)](#5-ofis-görünümü)
6. [İşe Alım ve Çıkış Yönetimi](#6-i̇şe-alım-i̇şten-çıkarma)
7. [Çoklu AI Motor Adaptör Katmanı](#7-motor-adaptörü)
8. [Kanban ve Görev Yönetimi](#8-kanban-sistemi)
9. [Bağlantılı Hafıza Sistemi (Knowledge Graph)](#9-hafıza-sistemi)
10. [İzolasyon ve Güvenlik (Git Worktree + Sandbox)](#10-i̇zolasyon-ve-güvenlik)
11. [MCP ve Dış Entegrasyonlar](#11-mcp-entegrasyonu)
12. [Veritabanı ve Kalıcılık Katmanı](#12-veri-kalilicilik)
13. [İletişim ve Mesajlaşma Sistemi](#13-i̇letişim-protokolü)
14. [Performans ve Ölçeklenebilirlik](#14-performans)
15. [Güvenlik Mimarişi](#15-güvenlik)
16. [Klasör Yapısı ve Kod Organizasyonu](#16-klasör-yapısı)
17. [MVP Aşamaları (0-3 Ay)](#17-mvp-yol-haritası)
18. [Rakip/Fark Analizi](#18-fark-analizi)

---


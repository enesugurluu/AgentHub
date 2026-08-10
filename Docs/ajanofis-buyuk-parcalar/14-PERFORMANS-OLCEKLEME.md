## 14. Performans ve Ölçeklenebilirlik

### 14.1 Sayısal Hedefler

| Metrik | Hedef | Gerekçe |
|:---|:---|:---|
| Soğuk başlangıç | <1.5 saniye (M2/modern Intel) | Tauri 2.9 benchmark 190 ms; UI yükleme ile 1.5 sn güvenli hedef |
| Boşta RAM | 80-120 MB (sadece uygulama) | Tauri taban çizgisi 45-80 MB; React + Sigma + xterm.js ile 120 MB |
| UI FPS (ofis katı) | 60 FPS (20 ajan + dekor) | SVG + requestAnimationFrame |
| Bilgi grafı FPS | 60 FPS 10K düğüme kadar; 30 FPS 50K | Sigma.js v3 WebGL |
| Paralel ajan kapasitesi | 12 eşzamanlı (kullanıcı makinesine göre ayarlanabilir) | Her CLI ajanı 200-500 MB; 12 × 300 MB ≈ 3.6 GB, 8 GB RAM makinede rahat |
| PTY latency | <16 ms (tuştan ekrana) | xterm.js + portable-pty kanıtlanmış (Tempest/Terax/Terminon) |
| Veritabanı sorgu (multi-hop) | <10 ms (10K entity grafı) | SQLite + FTS5 + recursive CTE |
| Crash/auto-kurtarma | Oturum yeniden başlatıldığında ajanlar kaldığı yerden | Process checkpoint + JSONL log replay |

### 14.2 Paralellik Stratejisi

- Her ajan kendi OS süreci; PTY stdio non-blocking (tokio::process)
- stdout okuma akışları tokio tasks ile paralel; her ajan için ayrı task
- Büyük JSONL loglar arka plan thread'inde yazılır (ana thread bloke olmaz)
- React tarafı:
  - Terminal çıktısı ring buffer ile (son 10.000 satır tutulur, üstü sanal scroll ile truncate)
  - Kanban kartları react-virtual ile sanallaştırılır
  - Sigma.js graf hesaplaması Web Worker içinde, ana thread bloke olmaz
  - Ofis katı SVG; yalnızca görünen ajanlar render (viewport culling)

### 14.3 Bellek Yönetimi

- Rust tarafı: tüm süreçler Arc/Mutex ile paylaşılır; ajan kapatılınca PTY ve worktree temizlenir
- React tarafı: uzun oturum terminal çıktısı `@tanstack/react-virtual` ile scroll sanallaştırma
- SQLite WAL modu; uzun yazmalar batch transaction ile
- JSONL loglar günlük rotasyona tabi; 30 günden eski arşivlenir
- Embedding vektörleri (384-dim f32) lazy yüklenir; aktif sorgu yoksa bellekte tutulmaz

### 14.4 Token ve Bütçe Takibi

- Her CLI çıktısı stream parser tarafından izlenir; maliyet ve turn bilgisi anlık olarak event bus'a akar
- Ajan bazlı, proje bazlı, global (gün/hafta/ay) bütçe limitleri
- Limit aşıldığında ajan otomatik duraklatılır; UI'da bildirim çıkar
- Sol üst köşede anlık "harcanan miktar" rozeti; tıklayınca detay grafik (Chart.js veya Recharts)

### 14.5 Ağ ve Dış Bağlantılar

- MCP sunucuları ile iletişim connection-pool üzerinden
- Opsiyonel bulut senkronizonu (sonraki sürüm) delta-based, arka planda
- Remote control (mobil) için WebSocket + uçtan uca şifreli kanal; varsayılan olarak kapalı

### 14.6 Karşılaştırma (Bench Kaynakları)

- Tauri 2.9 vs Electron 33: karşılaştırma (johal.in Nisan 2026, buildmvpfast Haziran 2026, fyrosoft Mart 2026)
  - 3 MB vs 96 MB hello world
  - 172 MB vs 409 MB RAM (6 pencere)
  - 190 ms vs 640 ms cold start (M2 Air)
- Sigma.js 10K-100K node render yeteneği (resmi sigma docs, graphology ekosistemi)
- xterm.js: VS Code, Hyper, Warp, Tempest, Terminon, Terax tarafından kullanılıyor; WebGL renderer ile 60 FPS

---

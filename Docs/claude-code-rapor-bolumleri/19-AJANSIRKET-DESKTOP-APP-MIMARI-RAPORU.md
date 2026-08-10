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

## 1. Ürün Vizyonu

### 1.1 Vizyon Cümlesi
> "Geliştiricinin kendi yönettiği, uzman AI çalışanlardan oluşan bir yazılım şirketi gibi çalışan; her çalışanın bir rolü, masası, uzmanlığı olan; görevleri görsel bir kanban üzerinde takip eden; öğrenilen bilgileri bağlantılı bir hafızada tutan; birden fazla AI motorunu (Claude Code, Codex, Gemini vb.) aynı anda koşturup orkestra edebilen desktop uygulama."

### 1.2 Farklılaşma
Mevcut orkestrasyon araçları (Claude Squad, Vibe Kanban, Kangentic, Nimbalyst) tek düzlemde görev çalıştırır. **AjanŞirket** ise:

| Mevcut araçlar | AjanŞirket |
|:---|:---|
| Düz görev listesi | Şirket metaforu + rol hiyerarşisi |
| Sadece CLI çıktısı | Ofis katı görselleştirmesi (ajanlar masasında) |
| Bellek yok/lineer not | Wiki-linkli bilgi grafı (Obsidian tarzı) |
| Sınırlı ajan özelleştirme | İşe alım menüsünden rol tanımı, motor seçimi, skill yükleme |
| Genelde Electron (ağır) | Tauri 2 + Rust (~15-20 MB kurulum) |
| Tek yönlü görev atama | CEO ↔ çalışan çift yönlü raporlama, toplantı (debat) özelliği |

### 1.3 Kullanıcı Personası
- Senior/Lead Developer ve Tech Lead
- AI-native üretim biçimini benimsemiş mühendis
- 2+ CLI kodlama ajanı kullanıyor ve aralarında koordinasyon ihtiyacı duyuyor
- Günde 4-8 saat kodlama ajanı ile çalışıyor
- Kendi "ajan ekibini" kurmak, yönetmek ve büyütmek istiyor

---

## 2. Yüksek Seviye Mimari

```
┌─────────────────────────────────────────────────────────────────┐
│           AJANŞIRKET DESKTOP APP (Tauri 2 Pencere)              │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │           REACT + TypeScript FRONTEND                    │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐ │ │
│  │  │  Office  │ │ Kanban   │ │ Memory   │ │ Settings /   │ │ │
│  │  │  Floor   │ │ Board    │ │ Graph    │ │ Hire/Fire    │ │ │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └──────┬───────┘ │ │
│  │       │             │             │              │         │ │
│  └───────┴─────────────┴─────────────┴──────────────┴─────────┘ │
│         │               IPC (Tauri invoke + events)             │
│  ┌──────┴───────────────────────────────────────────────────┐   │
│  │               RUST BACKEND (Tauri Core)                │   │
│  │  ┌────────────┐ ┌──────────────┐ ┌───────────────────┐  │   │
│  │  │ Orkestras- │ │ Process      │ │ Motor Adaptörleri │  │   │
│  │  │ yon Motoru │ │ Yöneticisi   │ (Claude/Codex/...) │  │   │
│  │  │ (CEO Core) │ │ (pty/stdio)  │                   │  │   │
│  │  └─────┬──────┘ └──────┬───────┘ └─────────┬─────────┘  │   │
│  │        │               │                   │            │   │
│  │  ┌─────┴──────┐ ┌──────┴───────┐ ┌────────┴─────────┐  │   │
│  │  │ Görev/Kan- │ │ Git Worktree │ │ Knowledge Graph  │  │   │
│  │  │ ban Yöneti-│ │ Yöneticisi   │ │ (SQLite + FTS5)  │  │   │
│  │  │ cisi       │ │ (İzolasyon)  │ │                  │  │   │
│  │  └────────────┘ └──────────────┘ └──────────────────┘  │   │
│  │                                                         │  │
│  └─────────────────────────────────────────────────────────┘   │
│                      Spawn edilmiş süreçler                    │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌────────┐  │
│  │ Claude  │ │ Codex   │ │ Gemini  │ │ OpenCode│ │ Aider  │  │
│  │ (work-  │ │ (work-  │ │ (work-  │ │ (work-  │ │ (work- │  │
│  │ tree1)  │ │ tree2)  │ │ tree3)  │ │ tree4)  │ │ tree5) │  │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Mimari Özeti
- **Katman 1 (UI):** React + TypeScript + Tailwind + shadcn/ui. Ofis görünümü, kanban, graf, ayarlar.
- **Katman 2 (Çekirdek):** Tauri 2 Rust backend. Orkestrasyon, süreç yönetimi, veritabanı, adaptörler, güvenlik.
- **Katman 3 (Ajanlar):** Harici CLI'lar (Claude Code, Codex vb.) kendi işleminde, kendi git worktree'sinde izole çalışır; stdio/PTY üzerinden kontrol edilir.
- **Katman 4 (Veri):** Yerel SQLite (kanban, ajanlar, ayarlar) + JSONL (konuşma geçmişi) + Markdown (bilgi notları/bağlantılar).

---

## 3. Teknoloji Yığını

### 3.1 Desktop Framework: Tauri 2.x (Rust)
**Neden Tauri, Electron değil?**

| Ölçüm | Tauri 2 | Electron 33 | Kazanan |
|:---|:---|:---|:---|
| Kurulum boyutu | ~5-15 MB | 120-200 MB | **Tauri (10-20x küçük)** |
| Boşta RAM | ~50-80 MB | 200-400 MB | **Tauri** |
| Soğuk başlangıç | ~300-800 ms | 2-3 sn | **Tauri** |
| Arka plan performansı | Rust native | Node.js V8 | **Tauri (PTY yönetimi kritik)** |
| Güvenlik modeli | Capability-based izinler, minimum yetki | Geniş Node erişimi | **Tauri** |
| Cross-platform | Win/macOS/Linux/mobil | Win/macOS/Linux | Eşit |
| Ekosistem olgunluğu | Gelişiyor (v2 production) | Çok olgun | **Electron** |
| UI tutarlılığı | System WebView (küçük farklılıklar) | Kendi Chromium'u (tam tutarlılık) | **Electron** |

**Karar: Tauri 2**
- Ağırlıklı olarak backend süreç yönetimi (PTY yaratma, stdio köprüleme, izolasyon) yapacağımız için Rust tarafı çok önemli avantaj
- Geliştirici araçları çoğunlukla Electron zaten kullanıyor (VS Code, Slack, Discord), kullanıcı ekstra Chromium yükü istemez
- Agentlar zaten 1-4 GB RAM tüketiyor, app footprint'i minimumda tutulmalı
- WebView tutarlılığı: 2026'da Tauri v2 WebView2 (Win)/WKWebView (mac)/WebKitGTK (Linux) geliştirici araçları için yeterli. Kritik canvas/animasyon tüm platformlarda standart.

### 3.2 Frontend Stack

| Katman | Teknoloji | Neden |
|:---|:---|:---|
| Framework | React 19 + Vite | Ekosistem, bileşen zenginliği |
| Stil | Tailwind CSS 4 + shadcn/ui | Hızlı geliştirme, tema desteği |
| Durum yönetimi | Zustand | Hafif, TypeScript-dostu |
| Kanban panosu | dnd-kit + react-window | Sürükle bırak, sanallaştırma |
| Bilgi grafı | react-force-graph-2d (d3-force tabanlı) | Force-directed layout, zoom/pan, performans |
| Terminal görünümü | xterm.js + Rust PTY | Kanıtlanmış (VS Code kullanıyor), ANSI destek |
| Diyagram/akış | React Flow (opsiyonel toplantı/görev akışı) | Esnek düğüm düzenleyici |
| Animasyonlar | Framer Motion | Ofis içi hareketlendirme için |

### 3.3 Backend (Rust) Kütüphaneler

| Amaç | Kütüphane |
|:---|:---|
| Async çalışma zamanı | tokio |
| PTY yönetimi | portable-pty |
| Git işlemleri | git2 (libgit2) |
| Veritabanı | rusqlite (SQLite) + FTS5 tam metin arama |
| HTTP/MCP | reqwest |
| JSON/seri | serde + serde_json |
| Süreç izleme | sysinfo |
| Şifreleme (API key saklama) | keyring.rs (OS keychain) |
| CLI argüman | clap |
| Log/tracing | tracing + tracing-subscriber |
| Dosya izleme | notify |

### 3.3 AI Motorlar (Desteklenecek İlk Dalga)

Adaptör arayüzü üzerinden plugin benzeri eklenir. İlk sürüm destek listesi:

1. **Claude Code** (`claude`) — Anthropic resmi CLI
2. **OpenAI Codex CLI** (`codex`)
3. **Google Gemini CLI** (`gemini`)
4. **OpenCode** (`opencode`) — Açık kaynak
5. **Aider** (`aider`) — Git-aware coding agent
6. **Cursor CLI** (`cursor-agent`)
7. **GitHub Copilot CLI** (`gh-copilot`)
8. **Qwen Code** (`qwen`)

---

## 4. Ajan Hiyerarşisi

### 4.1 Organizasyon Şeması

```
                           ┌─────────────────┐
                           │      CEO        │
                           │ (Orkestratör)   │
                           │ Ana model: Claude│
                           │   Sonnet/Opus   │
                           └────────┬────────┘
                                    │
           ┌────────────┬───────────┼───────────┬────────────┐
           │            │           │           │            │
    ┌──────▼─────┐ ┌────▼────┐ ┌───▼───┐ ┌─────▼─────┐ ┌────▼─────┐
    │    CTO     │ │   PM    │ │ QA    │ │ DevOps/   │ │ Designer │
    │ (Mimari)   │ │ Analyst │ │ Eng.  │ │ Infra     │ │ (UI/UX)  │
    └──────┬─────┘ └─────────┘ └───────┘ └───────────┘ └──────────┘
           │
    ┌──────┴──────────────┐
    │                     │
┌───▼─────┐         ┌─────▼─────┐
│ Frontend│         │ Backend   │
│  Dev    │         │ Dev       │
└─────────┘         └───────────┘
```

### 4.2 Yerleşik (Hazır) Roller

Uygulama ilk kurulumda bir "starter company" şablonu ile gelir. Kullanıcı bu rolleri düzenleyebilir/silebilir/yeni ekleyebilir:

| Rol | Açıklama | Varsayılan Motor | Model Seviyesi |
|:---|:---|:---|:---|
| **CEO** | Tüm şirketin orkestratörü. Görevleri alır, böler, dağıtır, çıktıları birleştirir. Her zaman çalışan tek ajan. | Claude Code | **Opus/Fable** (en yüksek zeka) |
| **CTO** | Mimari kararlar, teknoloji seçimi, code review, şema tasarımı. | Claude Code | Opus |
| **PM Analyst** | Gereksinim analizi, acceptance criteria yazar, görevleri yıkar. | Claude/Codex | Sonnet |
| **Backend Dev** | API, veritabanı, iş mantığı kodu yazar. | Claude Code / Codex | Sonnet/Haiku |
| **Frontend Dev** | React/UI kodu, CSS, etkileşim. | Claude Code / Gemini | Sonnet |
| **QA Engineer** | Test yazar, hata avlar, regression çalıştırır. Sıfır bağlam adversarial reviewer. | DeepSeek/Codex/Haiku | Ucuz model |
| **DevOps Engineer** | CI/CD, docker, dağıtım scriptleri, izleme. | Claude Code | Sonnet |
| **Designer** | UI/UX, CSS/Figma, erişilebilirlik. | Claude/Gemini | Sonnet |
| **Memory Keeper** (opsiyonel) | Bilgi grafını günceller, notları düzenler, çelişkileri çözer. | Haiku | Ucuz model |

### 4.3 CEO Agent Davranışı

CEO klasik "süpervizör-worker" mimarisinin orchestrator'ıdır. Ana sorumlulukları:

1. **Görev Alma:** Kullanıcıdan kanbana bırakılan kartları alır.
2. **Görev Analizi:** Kartın gereksinimlerini okur, alt görevlere böler.
3. **Yetkilendirme (Delegasyon):** Alt görevleri en uygun uzman çalışana atar.
4. **Paralel Yönetim:** Bağımsız görevleri paralel çalıştırır.
5. **Bağımlılık Yönetimi:** Bir görev başka birinin çıktısına ihtiyaç duyarsa sıraya koyar.
6. **Kalite Kontrol:** Çıkan işi QA/CTO'ya inceletir.
7. **Sentez:** Çalışanların çıktılarını birleştirir, ana branch'e merge hazırlar.
8. **İlerleme Raporu:** Durumu kullanıcıya ofis katı ve kanban üzerinde yansıtır.

CEO her zaman ana oturumda çalışmaz; o da diğer ajanlar gibi bir worktree içinde çalışır, ama ana branch'e en yakın olan ve koordinasyondan sorumlu olandır.

### 4.4 İletişim: Toplantı Özelliği
Tartışmalı veya karmaşık konularda CEO bir "toplantı" düzenleyebilir; birden fazla ajan aynı konu hakkında sırayla (veya debate/akran inceleme modunda) konuşur. Bu, Mixture-of-Agents tekniğini uygular.

---

## 5. Ofis Görünümü (Office Floor UI)

### 5.1 Ana Ekran
Uygulama açıldığında kullanıcıyı interaktif bir "ofis katı" karşılar. Bu 2.5D/izo veya üstten görünüm bir plan olabilir. Her çalışanın bir "masası" vardır ve masası üzerinde mevcut durumu görsel olarak ifade edilir:

**Görsel Durum Göstergeleri:**
- 🟢 **Yeşil / Çalışıyor:** Başında aktif bir terminal var, kod yazıyor/test koşuyor.
- 🟡 **Sarı / Düşünüyor:** Model reasoning anında, terminal boş, zihin meşgul.
- 🔵 **Mavi / Beklemede:** İnsan onayı bekliyor (permission required).
- 🟢✅ **Yeşil tik / Tamamlandı:** Görev bitti, çıktı hazır, inceleme bekliyor.
- 🔴 **Kırmızı / Hata:** Hata aldı, döngüde, yardıma ihtiyacı var.
- ⚪ **Gri / İzinde / İşsiz:** Üzerinde görev yok, boş duruyor.
- ☕ **Kahve molası:** 5 dk boştayken "dinlenme" animasyonu.

**Etkileşim:**
- Masaya/ajan kartına tıkla → o ajanın terminalini, mevcut görevini ve son çıktısını gösteren yan panel açılır.
- Sürükle bırak → kanbandan masaya kart bırakmak görevi o ajana atar.
- Sağ tık → "Ajanı duraklat", "Yeniden başlat", "İşten çıkar", "Ajan ayarları" menüsü.
- Ofiste gezinme: Zoom, pan, odalara ayrılmış görünüm (backend odası, tasarım köşesi, yönetici ofisi).

### 5.2 Görsel Stil
- **Stil:** Soft, modern, "cozy office" estetiği (çizgi film değil; temiz, profesyonel, biraz karakterli).
- **Tema:** Açık/koyu mod, özel renk şemaları.
- **Ajan avatarları:** Her ajan için özelleştirilebilir avatar/renk.
- **Mini terminal:** Her masanın üstünde mini konsol çıktısının son 5 satırı akan bir ticker.

### 5.3 Sol/Sağ Panel
- **Sol Panel:** Ajan listesi + anlık durum özeti (metin olarak).
- **Sağ Panel:** Seçilen ajanın tam terminali, mevcut görev detayı, son commitler, harcanan token/maliyet.
- **Alt Panel:** Sistem bildirimleri, maliyet özeti, toplam çalışan ajan sayısı.

---

## 6. İşe Alım ve İşten Çıkarma

### 6.1 "İşe Al" Sihirbazı
Ayarlar menüsünden "New Employee" tıklandığında çok adımlı bir sihirbaz açılır:

**Adım 1: Rol Seçimi**
- Hazır roller (CTO, Backend Dev, vb.) veya "boş özel rol"

**Adım 2: İsim ve Kişilik**
- İsim (örn: "Ayşe"), avatar/renk seçimi
- Sistem prompt/Kişilik (hazır şablonlardan veya özel düzenleme)

**Adım 3: AI Motor Seçimi**
- Desteklenen motorlardan birini seç (dropdown: Claude Code, Codex, Gemini CLI, OpenCode, vb.)
- Motor içi model seçimi (Opus/Sonnet/Haiku/GPT-5 vb. motor destekliyorsa)
- API key yönetimi (OS keychain'de saklanır)
- Effort seviyesi: low/medium/high/xhigh
- Maksimum tur limiti, bütçe limiti ($)

**Adım 4: İzinler ve Yetenekler**
- Dosya sistemi erişim yolları (sadece `src/backend` vb.)
- İzin verilen komutlar (test, lint, build; yok deny-list)
- Network erişimi (açık/kapalı/izinli domain)
- MCP sunucuları ekle
- Otomatik merge yetkisi var mı?

**Adım 5: Skills ve Hafıza**
- Hangi becerileri/skill dosyalarını yükleyecek?
- Hafıza grafiğine erişim seviyesi (okuma/yazma/sadece kendi notları)

**Adım 6: Onay ve Oluştur**
- Özet ekranı → "İşe al" → Rust tarafı o ajan için:
  - Konfigürasyonu veritabanına kaydeder
  - İlk çalıştırma için giriş komutlarını hazırlar
  - Ofis katına masayı/avatarı yerleştirir
  - Boştaysa beklemeye alır

### 6.2 "İşten Çıkar"
- İkna edici bir onay dialogu ("Bu ajanın işine son vermek istediğinize emin misiniz?")
- Ajanın mevcut branch/worktree'sini ne yapacaksınız?:
  - Worktree'yi sil ve değişiklikleri ziyan et
  - Worktree'yi koru, bir başkası devralacak
  - Değişiklikleri commit'le ve branch olarak sakla
- Ajan konfigürasyonunu sil
- Ofisten kaldır

### 6.3 "Terfi / Transfer"
Ajanı bir rolden diğerine taşıma; motorunu, yetkilerini düzenleme. Yeni çalışma alanı atama.

---

## 7. Çoklu AI Motor Adaptör Katmanı

Her CLI farklı arayüz ve komutlarla çalıştığından, Rust tarafında ortak bir **AgentAdapter** trait tanımlanır:

```rust
#[async_trait]
pub trait AgentAdapter: Send + Sync {
    fn name(&self) -> &str;                        // "claude", "codex" vb.
    fn binary(&self) -> &str;                      // çalıştırılabilir komut
    fn default_args(&self) -> Vec<String>;         // varsayılan CLI argümanları
    async fn spawn(&self, config: AgentSpawnConfig) -> Result<AgentProcess>;
    async fn send_message(&mut self, msg: &str) -> Result<()>;
    async fn send_interrupt(&mut self) -> Result<()>; // Ctrl+C
    async fn set_model(&mut self, model: &str) -> Result<()>;
    async fn read_stdout(&mut self) -> Result<String>;
    fn supports_feature(&self, feature: AgentFeature) -> bool;
    // Özellikler: paralel_agent, worktree_arg, resume, background_mode
}
```

### Adaptör Uygulamaları
Her motor için bu trait implemente edilir:

| Adaptör | Spawn Komutu | Özellikler |
|:---|:---|:---|
| Claude Code | `claude --print` veya interaktif PTY | `--worktree`, MCP, native resume |
| Codex CLI | `codex exec` | OpenAI modelleri |
| Gemini CLI | `gemini run` | Google multimodal destek |
| OpenCode | `opencode run` | Açık kaynak, çoklu model |
| Aider | `aider --message` | Git-aware, tek dosya odaklı |
| Cursor CLI | `cursor-agent` | Cursor aboneliği |
| Copilot CLI | `gh copilot` | GitHub ekosistemi |
| Qwen Code | `qwen` | Alibaba açık kaynak |

### Motor Çalıştırma Modeli
Her ajan için Rust tarafında ayrı bir çocuk süreç spawn edilir. PTY (pseudo-terminal) üzerinden stdin/stdout bağlanır. Çıktı anlık olarak:
- Terminal paneline yansıtılır (ANSI renkleri korunur)
- Çıktı analizi yapılır (hata ayıkla, başarı durumu tespiti, izin bekleme)
- JSONL olarak log dosyasına yazılır (denetim için)
- Durum makinesi güncellenir (çalışıyor/düşünüyor/hata/bitti)

### Çıktı Anlamlandırma (Event Parsing)
Her adaptör kendi motorunun çıktı kalıplarını tanır:
- `[2/5]` tur sayısı → ilerleme çubuğu
- "Allow this command?" gibi onay istemi → mavi (beklemede) durumu
- "Error:" veya stack trace → kırmızı durumu
- "Task complete" veya testlerden geçme → yeşil tik

---

## 8. Kanban Sistemi

### 8.1 Sütunlar
Varsayılan sütunlar, özelleştirilebilir:

| Sütun | Amaç |
|:---|:---|
| **Backlog** | Kullanıcının veya CEO'nun eklediği işler |
| **Analiz (PM)** | PM Analyst'in üzerinde çalıştığı, parçalara ayırdığı kartlar |
| **To Do** | Atama bekleyen, hazır görevler |
| **In Progress** | Bir çalışana atanmış, aktif iş |
| **Review** | QA/CTO incelemesinde |
| **Done** | Tamamlanmış, merge edilmiş iş |

### 8.2 Kart Yapısı
Her görev kartı şunları içerir:
- `id`, başlık, açıklama, kabul kriterleri
- Atanan ajan(lar)
- Bağlı worktree/branch
- Üst görev (parent task)
- Bağlı bilgi notları (memory graph linkleri)
- Öncelik, etiketler
- Tahmini token/maliyet
- Durum, oluşturma/bitiş zamanları
- Bloklayan/blocklanan görevler

### 8.3 Etkileşim
- Sürükle bırak ile sütunlar arası taşı
- Kartı doğrudan ofisteki bir ajanın masasına bırak → o ajana atanır
- Kartı aç: detaylar, sohbet, çıktı geçmişi, dosyalar
- "CEO'ya ver" → kartı CEO queue'suna bırak, CEO kendi böler/dağıtır
- "Paralel çalıştır" → aynı kart için birden fazla ajan (aynı veya farklı motor) aynı sorunu çözer, en iyi sonucu seç (competitive mode)

### 8.4 CEO Otomasyonu
Kullanıcı kartları sadece Backlog'a bırakır. CEO otomatik:
1. Kartta görev tanımını okur
2. Analiz ve parçalama için PM'e verir (gerekirse)
3. Bağımsız parçaları tespit eder
4. Uygun uzmanlara dağıtır
5. Bitince review zincirine alır
6. Tüm parçalar bittiğinde sentezler

---

## 9. Hafıza Sistemi (Bağlantılı Bilgi Grafı)

### 9.1 Hedef
Obsidian'ın `[[]]` çift bağlantı mantığını ajan hafızasıyla birleştirmek. Amaç:
- Her bilgi bir "not" (markdown dosyası)
- Notlar arası `[[]]` ile çift yönlü bağlantı
- Notlar tipine göre kategorize (Entity, Decision, Incident, Gotcha, ADR, Pattern)
- Bu notlar bir graf veri yapısına otomatik indekslenir
- Ofis içindeki Memory Keeper ajanı (veya istek üzerine çalışanlar) bu notları otomatik oluşturur/günceller
- Görsel bir force-directed graf olarak gezilebilir

### 9.2 Dizin Yapısı
```
~/AgentCompany/vault/
├── README.md
├── ADR/                       ┐
│   ├── ADR-001-postgres.md     │
│   └── ADR-002-redis-queue.md  │ Kararlar ve mimari notlar
├── Incidents/                 │
│   ├── INC-042-redis-oom.md   │
│   └── INC-053-auth-bug.md    ┘
├── Patterns/                  ┐
│   ├── tdd-workflow.md         │
│   └── react-server-components │ Bilinen desenler
├── Gotchas/                   ┐
│   └── nextjs-cache.md         │ Tuzaklar
├── Entities/                  ┐
│   ├── Redis.md                │
│   ├── Auth0.md                │ Servisler, kütüphaneler
│   └── Stripe.md              ┘
└── People/                    ┐
    └── team-conventions.md     │ Takım bilgileri
```

### 9.3 Wiki-link Sözdizimi
Not içinden diğer notlara referans için standart çift köşeli parantez kullanılır:

```markdown
# Redis

Redis in-memory veri deposu olarak kullanılıyor.

## İlişkiler
- [[ADR-002-redis-queue]] sonrası queue için BullMQ'ya geçildi
- [[INC-042-redis-oom]] nedeniyle deprecate edildi
- [[Postgres]] ile birlikte kullanılıyor
```

Bu sözdizimi otomatik olarak çift yönlü bağlantı kurar. Backlink paneli notun kimden referans aldığını gösterir.

### 9.4 Veri Katmanı
- **Markdown dosyaları** kaynak olarak kullanılır (insan tarafından okunabilir, Git ile versiyonlanır)
- **SQLite indeksi** anlık sorgu için:
  - `notes` tablosu (id, başlık, yol, tip, oluşturma/güncelleme zamanı)
  - `links` tablosu (source_note → target_note, çift yönlü)
  - `tags` tablosu
  - `note_fts` FTS5 sanal tablo (tam metin arama)
- Bir dosya değiştiğinde Rust tarafında notify ile izlenir, indeks otomatik güncellenir.

### 9.5 Görsel Graf Ekranı
Memory sekmesinde 3 görselleşme modu:

1. **Graf Görünümü:** Force-directed düğüm-ağ diyagramı (react-force-graph). Düğümler tipine göre renkli (ADR mavi, incident kırmızı, service yeşil). Yakınlaştır, sürükle, düğüme tıkla → notu aç. Filter by type, tarih, etiket.
2. **Not Görünümü:** Obsidian benzeri markdown editör + backlink paneli.
3. **Graph Sorgu (CEO için):** CEO bilgi ihtiyacında grafı otomatik sorgular ("Redis neden kaldırıldı" → ADR/incident zincirini izleyip cevap oluşturur). Bu 3-hop sorgulama RAG'dan daha doğru sonuç verir (raporumuzda kanıtlandığı gibi).

### 9.6 Otomatik Hafıza Güncellemesi
Her önemli karardan, hatadan veya dersden sonra Memory Keeper ajanı tetiklenir:
- Olayı not eder
- Bağlantıları kurar
- Zaten varsa üstüne ekler
- Çelişki varsa "supersedes" linki ile eski bilgiyi güncellediğini işaretler

Hafıza hazard'ları (duplicate identity, stale commitment, contradictory decisions) tespit edilir ve kullanıcıya bildirilir.

---

## 10. İzolasyon ve Güvenlik

### 10.1 Her Ajan İçin Ayrı Git Worktree
Her ajan için kendi iş dizini, branch'i ve worktree'si otomatik oluşturulur:
```
<project>/.agentcompany/worktrees/
├── ceo/                 # CEO worktree (ana branch)
├── cto/
├── backend-1/
├── frontend-2/
└── qa-1/
```

Worktree oluşturma iş akışı:
1. Görev atanınca Rust `git worktree add` çağrısı yapar
2. `.env.local` dosyası otomatik oluşturulur (port çakışmasını önlemek için her ajan için farklı PORT atanır)
3. node_modules sembolik link ile paylaşılır (disk tasarrufu)
4. Ajan process'i o dizinde spawn edilir
5. Görev bitiminde/ajan işten çıkartıldığında worktree ya silinir ya inceleme için tutulur

### 10.2 Süreç İzolasyonu
- Her ajan kendi OS sürecinde çalışır
- PTY aracılığıyla izole edilir
- İsteğe bağlı Docker sandbox desteği (ileri seviye): ajan konteyner içinde çalışabilir (internet erişimi, dosya sistemi kısıtları ile)
- Ana proje dizini hiçbir ajan tarafından doğrudan değiştirilemez, sadece kendi worktree'sinde değişiklik yapar.

### 10.3 Runtime İzolasyonu (Port/DB Çakışma)
Port numaraları atanırken ajan ID'sine göre offset kullanılır:
```
PORT = 3000 + (agent_id * 10)
REDIS_DB = agent_id
TEST_DB = `test_${agent_id}`
```
.env.local otomatik oluşturulur.

### 10.4 Ana Branch Koruma
Hiçbir ajan direkt ana branch'e push/commit atamaz. Tüm değişiklikler kendi branch'inde kalır. Merge işlemi:
1. QA incelemesinden geçer
2. CEO tarafından onaylanır
3. İnsana sunulur (opsiyonel otomatik merge sadece onayla)

---

## 11. MCP Entegrasyonu

MCP (Model Context Protocol) tüm AI ajanlarının dış araçlara erişiminde ortak standart olarak kullanılır.

### 11.1 Şirket Genelinde MCP
Proje için tanımlı MCP sunucuları tüm çalışanların erişimine sunulabilir (veya role bazlı kısıtlanabilir):

| MCP Sunucusu | Rol |
|:---|:---|
| GitHub MCP | Tüm çalışanlar (issue/PR erişimi) |
| Linear MCP | PM, CEO |
| Sentry MCP | QA, Backend Dev |
| Postgres MCP | Backend Dev (readonly başlangıçta) |
| Playwright MCP | QA, Frontend |
| Figma MCP | Designer |
| Slack/Notification MCP | CEO (bildirim için) |
| File system | Roller bazlı izinler |

MCP'ler Rust tarafında yönetilir ve ajan adaptörü aracılığıyla spawn edilirken CLI argümanı/konfigürasyon olarak aktarılır.

### 11.2 MCP Yönetim Ekranı
Ayarlar menüsünden MCP ekle/kaldır; hangi rollerin erişimi olduğunu işaretle.

---

## 12. Veri Kalıcılığı

### 12.1 Veritabanı Şeması (SQLite)
Tüm operasyonel veri yerel SQLite dosyasında tutulur (proje başına veya global):

```sql
-- Ajanlar
CREATE TABLE agents (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  role TEXT NOT NULL,
  avatar_color TEXT,
  motor TEXT NOT NULL,           -- claude, codex vb.
  model TEXT,
  system_prompt TEXT,
  config_json TEXT,               -- yetkiler, bütçe, tur limiti
  worktree_path TEXT,
  status TEXT DEFAULT 'idle',     -- idle/running/thinking/waiting/error/done
  created_at TEXT,
  hired_at TEXT,
  fired_at TEXT
);

-- Görevler (Kanban)
CREATE TABLE tasks (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  acceptance_criteria TEXT,
  column TEXT DEFAULT 'backlog',
  assigned_agent_id INTEGER REFERENCES agents(id),
  parent_task_id INTEGER REFERENCES tasks(id),
  priority INTEGER DEFAULT 3,
  budget REAL,                    -- USD limiti
  spent_tokens_input INTEGER DEFAULT 0,
  spent_tokens_output INTEGER DEFAULT 0,
  spent_cost REAL DEFAULT 0,
  worktree_path TEXT,
  created_at TEXT,
  started_at TEXT,
  completed_at TEXT,
  blocked_by INTEGER
);

-- Hafıza notları (indeks; markdown dosyaları kaynak)
CREATE TABLE notes (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  path TEXT NOT NULL UNIQUE,
  note_type TEXT,                 -- adr, incident, entity, gotcha...
  created_at TEXT,
  updated_at TEXT
);
CREATE TABLE note_links (
  source_id INTEGER REFERENCES notes(id),
  target_id INTEGER REFERENCES notes(id),
  context TEXT,
  PRIMARY KEY (source_id, target_id)
);
CREATE VIRTUAL TABLE note_fts USING fts5(title, content, path, content='notes', content_rowid='id');

-- Olay/aktivite logu (denetim)
CREATE TABLE events (
  id INTEGER PRIMARY KEY,
  agent_id INTEGER REFERENCES agents(id),
  task_id INTEGER REFERENCES tasks(id),
  event_type TEXT,                -- spawn, output, error, approval, complete
  payload TEXT,
  timestamp TEXT
);

-- Ayarlar
CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT);
```

### 12.2 Konuşma Geçmişi
- Her ajan oturumunun çıktısı `~/.agentcompany/logs/<agent>/<task>-<timestamp>.jsonl` dosyasına JSONL olarak yazılır
- Her satır: `{timestamp, type (input/output/system/event), content, tokens, cost}`
- Bu log'lar incelemek ve hafızayı güncellemek için de kullanılır

### 12.3 Konfigürasyon
- Global konfigürasyon: `~/.agentcompany/config.toml`
- Proje bazlı konfigürasyon: `<proje>/.agentcompany.toml`
- API anahtarları OS keychain'de saklanır (Windows Credential Manager, macOS Keychain, Linux Secret Service)

---

## 13. İletişim Protokolü

### 13.1 CEO → Çalışan
CEO bir görevi çalışana devrettiğinde:
1. Worktree oluşturulur
2. Görev tanımı, ilgili context (ilgili dosyalar, kabuller, önceki çalışmalar), kabul kriterleri bir `AGENT_TASK.md` dosyasına yazılır
3. Bilgi grafından ilgili notlar linklenir
4. Ajan `claude` (veya motor komutu) çalıştırılır
5. Ajan çıktısı anlık CEO paneline akar

### 13.2 Çalışan → CEO
Çalışan görevi tamamladığında (veya tıkandığında):
- `TASK_COMPLETE.md` veya `TASK_BLOCKED.md` oluşturur
- Durum değişikliği Rust tarafında event olarak yakalanır
- CEO bilgilendirilir
- Review aşaması tetiklenir

### 13.3 Onay (Human-in-the-loop)
Ajan bir izin istediğinde veya kritik bir eyleme (migrate, deploy, büyük harcama, force push) geldiğinde:
- Süreç duraklatılır
- UI üzerinden mavi bildirim görünür
- Kullanıcı "İzin ver", "Reddet", "Düzenle" seçebilir
- Verilen cevap ajan sürecine stdin ile geri beslenir
- 10 dakika onay gelmezse ajan "blocked" durumuna alınır (üretim israfını önler)

---

## 14. Performans ve Ölçeklenebilirlik

### 14.1 Hedefler
- Tek seferde **12 ajan**a kadar paralel çalıştırma (kullanıcı makinesinin CPU/RAM sınırları dahilinde)
- UI 60 FPS (graf görünümü 1000 düğüme kadar akıcı)
- Bellek kullanımı ~100-150 MB (app'in kendisi; ajanların tüketimi ayrı)
- Soğuk başlangıç <1 saniye
- Tüm veriler yerel (bulut zorunlu değil), opsiyonel bulut senkronizasyon sonraki sürümlerde

### 14.2 Paralellik Stratejisi
- Her ajan kendi OS süreci, PTY non-blocking (tokio::process)
- stdout okuma akışları tokio tasks ile paralel
- UI tarafı React virtualization ile büyük log/terminal çıktılarını render
- Force-directed graf hesaplaması web worker içinde, ana thread bloke olmaz

### 14.3 Token Bütçesi Takibi
- Her ajanın çıktısından token tahmini yapılır (anthropic/codex CLI'ların `/usage` çıktısı veya çıktıdaki `[tur X/Y]` kalıpları)
- Anlık toplam harcama sol üst köşede görünür
- Ajan bazlı veya proje bazlı bütçe limiti aşıldığında ajan duraklatılır ve bildirim verilir

---

## 15. Güvenlik

### 15.1 İzin Modeli (Capability-based)
- Her ajan için yetkiler açıkça verilir (White-list yaklaşımı)
- Tehlikeli komutlar (rm -rf, DROP TABLE, force push, sudo, curl|sh) varsayılan olarak kısıtlanır
- PreToolUse/PreBash hook mantığına benzer bir komut filtreleme Rust tarafında yapılır (CLI'dan önce)
- `.env` ve gizli dosyalar worktree'lere KOPYALANMAZ; izinli ise sadece readonly sembolik link verilebilir

### 15.2 Onay Gerektiren Eylemler
Varsayılan olarak şu durumlarda insan onayı istenir:
- Production ile ilgili komutlar (deploy, prod deploy)
- Migration çalıştırma
- Yeni dış paket kurulumu (npm install dışında)
- 500 satırı geçen tek seferde değişiklik
- $2 bütçeyi aşan görev
- Dış ağ erişimi (MCP'den değil, doğrudan curl/wget)
- Ana branch'e merge

### 15.3 API Anahtarları
- Hiçbir zaman düz metin konfigürasyonda saklanmaz
- OS keychain kullanılır (Rust `keyring` crate)
- Ajan süreçleri çevre değişkeni olarak alır, loglarda maskelenir

### 15.4 Denetim Kaydı
- Tüm komutlar, çıktılar ve aksiyonlar JSONL olarak loglanır
- Değiştirilen dosyalar ve commit'ler tam geçmiş ile tutulur
- "Time Machine" özelliği: herhangi bir ajan oturumunu geriye sarıp ne olduğunu inceleme

---

## 16. Klasör Yapısı

```
agentcompany/
├── src-tauri/                # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── commands/         # Tauri IPC komutları
│   │   ├── agents/           # AgentAdapter trait ve implementasyonlar
│   │   │   ├── mod.rs
│   │   │   ├── claude.rs
│   │   │   ├── codex.rs
│   │   │   └── ...
│   │   ├── orchestrator.rs   # CEO mantığı
│   │   ├── pty.rs            # PTY yönetimi
│   │   ├── worktree.rs       # Git worktree işlemleri
│   │   ├── kanban.rs         # Görev yönetimi
│   │   ├── memory.rs         # Bilgi grafı indeksleme
│   │   ├── security.rs       # İzin kontrolü
│   │   ├── db.rs             # SQLite bağlantısı + migration
│   │   └── events.rs         # Event bus
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                      # React frontend
│   ├── App.tsx
│   ├── main.tsx
│   ├── components/
│   │   ├── OfficeFloor/      # Ofis görünümü
│   │   │   ├── OfficeFloor.tsx
│   │   │   ├── AgentDesk.tsx
│   │   │   └── StatusBadge.tsx
│   │   ├── Kanban/           # Kanban panosu
│   │   │   ├── Board.tsx
│   │   │   ├── Column.tsx
│   │   │   └── Card.tsx
│   │   ├── Memory/           # Hafıza grafı
│   │   │   ├── GraphView.tsx
│   │   │   ├── NoteEditor.tsx
│   │   │   └── Backlinks.tsx
│   │   ├── Terminal/         # Ajan terminali
│   │   │   └── XTerminal.tsx
│   │   ├── Settings/
│   │   │   ├── HireWizard.tsx
│   │   │   ├── MCPSettings.tsx
│   │   │   └── AgentSettings.tsx
│   │   └── common/           # shadcn/ui bileşenleri
│   ├── store/                # Zustand store
│   │   ├── agents.ts
│   │   ├── kanban.ts
│   │   └── memory.ts
│   ├── lib/
│   │   ├── ipc.ts            # Tauri invoke köprüsü
│   │   └── utils.ts
│   └── styles/
│
├── package.json
├── vite.config.ts
├── tailwind.config.ts
└── README.md
```

---

## 17. MVP Yol Haritası (3 Ay)

### Faz 1: Çekirdek (2 Hafta) — Prototip
- [x] Tauri 2 projesi iskeletini kur
- [x] Temel React UI
- [x] SQLite şeması oluştur
- [x] Claude Code adaptörünü yaz (tek ajan spawn, stdout oku)
- [x] Terminal paneli (xterm.js)

### Faz 2: Çoklu Ajan (3 Hafta)
- [ ] Tüm adaptörleri yaz (Codex, Gemini, OpenCode)
- [ ] Git worktree yönetimi
- [ ] Ajan ekleme/silme ayar ekranı
- [ ] Ofis görünümü basit hali (masalar + durum)

### Faz 3: CEO Orkestrasyonu (4 Hafta)
- [ ] CEO mantığı (görev bölme, dağıtma)
- [ ] Kanban sürükle bırak
- [ ] Onay akışı (mavi bildirim)
- [ ] Temel komut izinleri (güvenlik)

### Faz 4: Hafıza Grafı (3 Hafta)
- [ ] Markdown not sistemi, wiki-link ayrıştırma
- [ ] SQLite indeks + FTS
- [ ] Force-directed graf görselleştirme
- [ ] Otomatik Memory Keeper ajanı

### Faz 5: Cila ve Yayın (2 Hafta)
- [ ] Tema, animasyonlar
- [ ] Maliyet/Token takibi dashboard
- [ ] Import/export ayarlar
- [ ] macOS/Windows/Linux paketleme ve imzalama
- [ ] İlk sürüm yayın

Toplam süre: ~14 hafta (tek bir geliştirici için; ekiple daha kısa).

---

## 18. Fark Analizi (Rakiplere Göre)

| Özellik | Claude Squad | Vibe Kanban | Kangentic | Nimbalyst | **AjanŞirket** |
|:---|:---|:---|:---|:---|:---|
| Görsel ofis metaforu | ❌ | ❌ | ❌ | ❌ | ✅ |
| Rol hiyerarşisi (CEO-uzman) | ❌ | ❌ | ❌ | ❌ | ✅ |
| Bağlantılı hafıza grafı | ❌ | ❌ | ❌ | ❌ | ✅ |
| Obsidian-tarzı wiki-notlar | ❌ | ❌ | ❌ | ❌ | ✅ |
| Hire/fire akıcı UX | ❌ | ❌ | ❌ | ❌ | ✅ |
| Kanban sistemi | ❌ | ✅ | ✅ | ✅ | ✅ |
| Worktree izolasyonu | ✅ | ✅ | ✅ | ✅ | ✅ |
| Çoklu motor (11+) | 6 | 10+ | 11 | 3 | **11+** |
| Desktop native (Tauri hafifliği) | TUI | Web | Electron | Electron | **Tauri** |
| Debat/toplantı modu | ❌ | ❌ | ❌ | ❌ | ✅ |
| Üretim-ready güvenlik modeli | Basit | Basit | Basit | Orta | **Kapsamlı** |
| Yerel SQLite ile veri sahipliği | tmux | ? | Local | ? | **Tam** |

---

## Sonuç

Bu mimari:
- Endüstri standardı Hierarchical Orchestrator-Worker paternini uygular.
- Modern ve hafif Tauri 2 + Rust teknolojisini kullanır.
- 11+ AI motoru destekleyen genişletilebilir adaptör katmanı sunar.
- Git worktree tabanlı tam izolasyon sağlar.
- Obsidian-tarzı bağlantılı hafıza ile bilgi kaybını önler.
- Kullanıcıyı bir "şirket CEO'su" konumuna yerleştirir ve ofis metaforuyla görsel bir deneyim sunar.
- Kanban + interaktif terminal + memory graph ile tam üretim çevrimini destekler.
- Güvenlik ve izin modeli ile production kullanımına uygun olur.

Başlangıçta yerleşik rollerle (CEO, CTO, Backend, Frontend, QA, DevOps, PM) gelir; kullanıcı zamanla kendi çalışanlarını "işe alarak" şirketi büyütebilir.

İsterseniz herhangi bir bölümün (örneğin Rust AgentAdapter trait kodunun veya Office Floor React bileşeninin) implementasyonunu da detaylı olarak oluşturabilirim.

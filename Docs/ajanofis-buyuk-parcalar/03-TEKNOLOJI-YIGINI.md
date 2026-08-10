## 3. Teknoloji Yığını

### 3.1 Desktop Framework: Tauri 2.x (Rust Backend)

**Neden Tauri 2, Electron değil?** 2026 bağımsız benchmarklar kararımızı doğruluyor:

| Ölçüm | Tauri 2.9.6 | Electron 33 | Kazanan |
|:---|:---|:---|:---|
| Hello-world bundle | **~3 MB** | ~96 MB | Tauri (32x küçük) |
| Tipik production bundle | **5-12 MB** | 150-244 MB | Tauri |
| Boşta RAM (tek pencere) | **45-85 MB** | 160-450 MB | Tauri |
| 6 açık pencere RAM | **172 MB** | 409 MB | Tauri (%58 daha az) |
| Cold start (M2 Air) | **190 ms** | 640 ms | Tauri (3.4x hızlı) |
| Backend dil | Rust (derlenmiş native) | Node.js (V8 JIT) | Tauri |
| Rendering motoru | OS-native WebView (WebView2/WKWebView/WebKitGTK 4.1) | Paketli Chromium | Electron (tutarlılık) |
| Mobil hedefler | iOS 9+ / Android 8+ | Yok | Tauri |
| IPC modeli | Capability-permission bridge | Geniş Node erişimi | Tauri |
| Auto-updater | Yerleşik, imzalı binary | electron-updater (3. parti) | Eşit |
| 2025 Q4 CVE sayısı | 2 (Rust std) | 14 (Chromium + Node) | Tauri (%86 az) |
| Lisans | MIT/Apache 2.0 | MIT | Eşit |

Kaynaklar: Tauri 2.9.6 release ölçümleri (9 Aralık 2025), johal.in Nisan 2026 benchmark, buildmvpfast Haziran 2026, fyrosofttech Mart 2026, dev.to Temmuz 2025 gerçek uygulama karşılaştırması (8.6 MiB vs 244 MiB).

**Karar: Tauri 2.9+**
- Ajanlar zaten 1-4 GB RAM tüketiyor; app footprint minimumda kalmalı
- PTY süreç yönetimi ve ağır I/O Rust native performansında çalışmalı
- Capability-based izin modeli "en az yetki" güvenlik prensibine uyuyor
- WebView tutarsızlık riski: 2026'da WebView2/WKWebView/WebKitGTK modern Canvas/WebGL/React'i tam destekliyor; UI kodunu bu üç motorla test eden CI matrisi kurarız
- Electron sadece Node ekosistemine ağır bağımlılık veya pixel-mükemmel çapraz-platform gerektiğinde tercih edilir; bizim durumumuzda her ikisi de yok

### 3.2 Frontend Stack (React 19 + Vite)

| Katman | Teknoloji | Gerekçe |
|:---|:---|:---|
| Framework | React 19 + TypeScript + Vite | Ekosistem, tip güvenliği, HMR |
| Stil | Tailwind CSS 4 + shadcn/ui | Tema, hızlı iterasyon, erişilebilir bileşenler |
| Durum | Zustand | Hafif, middleware (devtools/persist/immer), boilerplate yok |
| Sürükle bırak (kanban) | @dnd-kit | shadcn/ui uyumlu, erişilebilir |
| Sanallaştırma | @tanstack/react-virtual | Büyük listeler/kanban kartları |
| **Bilgi grafı (birincil)** | **Sigma.js v3 + Graphology** | WebGL üstü, 10K-100K düğüme kadar akıcı; topluluk algoritmaları (Louvain), PageRank, BFS/DFS hazır; AjanŞirket ölçeği için en güvenilir seçim. canvas/WebGL tabanlı. |
| Bilgi grafı (3D opsiyonel) | three-forcegraph + react-force-graph-3d | Büyük grafı 3B keşif modu için (isteğe bağlı özellik); ana 2D görünüm Sigma.js |
| Terminal | **xterm.js 5 + xterm-addon-fit + xterm-addon-web-links + xterm-addon-search** | VS Code tarafından kullanılıyor, endüstri standardı; WebGL renderer ile 60 FPS. Tempest, Terax/TERAX, Terminon gibi Tauri 2 terminal projelerinin hepsi bunu kullanıyor. |
| Akış/diyagram | React Flow | Toplantı akışı, görev bağımlılık grafı |
| Animasyon | Framer Motion | Ofis içi hareketlendirme (ajan yürüme, durum değişimi) |
| Formlar | react-hook-form + zod | Hire/fire wizard ve ayarlar |
| Tablo | TanStack Table | Ajan listesi, log görüntüleme |

> **react-force-graf notu:** 2D canvas (force-graph) ve 3D (three-forcegraph) olarak mevcut. 1000+ düğümde performansı Sigma.js'ten geridir; D3-force simülasyonu CPU'da kalır. AjanŞirket'in bilgi grafı uzun vadede binlerce not/bağlantı içereceği için varsayılan görünümde Sigma.js + Graphology kullanıyoruz; 3B "starlapse" görünümü isteğe bağlı bir mod olarak react-force-graph-3d.

### 3.3 Backend (Rust) Kütüphaneler

| Amaç | Kütüphane | Not |
|:---|:---|:---|
| Async runtime | tokio | Çoklu PTY yönetimi için multi-thread runtime |
| PTY yönetimi | **portable-pty** (git: https://github.com/wez/wezterm, portable-pty crate) | Cross-platform (ConPTY Win, posix Unix); Tempest, Terminon, Terax gibi açık kaynak Tauri+Rust+xterm.js projelerinin hepsi bunu kullanıyor |
| Süreç/işlem yönetimi | tokio::process + portable-pty PTY wrapper | Non-blocking stdout okuma, pty resize |
| Git işlemleri | git2 (libgit2) veya `git` CLI wrapper | Worktree yaratma/silme, diff, log. CLI wrapper daha esnek |
| Veritabanı | rusqlite | SQLite senkron, FTS5 dahil |
| Vektör benzerlik / hybrid search | **sqlite-vec** | Rust tarafından da kullanılabilen SQLite vektör eklentisi; tek dosya DB içinde BLOB float vektörler; Reciprocal Rank Fusion ile FTS5 melez arama. obra/knowledge-graph ve sqlite-graph bu mimariyi kullanıyor |
| Graph traversal | Özel recursive CTE'ler + rust sürüm graphology-benzeri Louvain/PageRank | İhtiyaç olursa graphlib veya petgraph |
| Wikilink ayrıştırma (markdown) | Frontend'de `remark-wiki-link-plus` (npm), Rust tarafında basit regex/pulldown-cvt event işleme | Flowershow/remark-wiki-link "shortestPossible" çözümlemeyi kullanır (Obsidian'le aynı) |
| HTTP istemcisi | reqwest | Remote MCP sunucularına erişim için |
| **MCP client (Rust tarafı)** | **`rmcp` (resmi Rust MCP SDK)** v0.8+ | Model Context Protocol'ün resmi Rust SDK'sı; child-process transport ile CLI MCP sunucularını spawn edebilir; Streamable HTTP, OAuth, macro ile tool tanımı. Alternatif: **`mcp_client_rs`** (daha basit, sadece stdio). Yüksek performanslı çok-istemci sunucu ihtiyacında **`rust-mcp-sdk`** (Axum entegrasyonlu, DNS-rebinding korumalı). |
| Serileştirme | serde + serde_json |  |
| Sistem izleme | sysinfo | Çalışan ajan süreçleri, CPU/RAM ölçümü |
| Şifreli saklama | **keyring** crate | OS keychain (Windows Credential Manager, macOS Keychain, Linux Secret Service) |
| CLI argüman | clap | Yardımcı CLI |
| Log/tracing | tracing + tracing-subscriber | Yapılandırılmış log |
| Dosya izleme | notify | Vault/not klasörü değişikliklerini anlık indeksle |
| Zaman | chrono |  |
| UUID | uuid | Agent/task ID |
| UI komut kanalı | Tauri events (Channel<T>) | PTY çıktısını anlık olarak React tarafına stream; Tempest ve Terax bu deseni kullanıyor |

### 3.4 Desteklenecek AI Motorları (İlk Dalga)

Adaptör trait üzerinden eklenir. İlk sürüm:

1. **Claude Code** (`claude`) — resmi CLI, native installer; en yüksek SWE-bench skoru (88.6%)
2. **OpenAI Codex CLI** (`codex`) — Apache 2.0, OpenAI tarafından destekleniyor; cloud sandbox
3. **Google Gemini CLI** (`gemini`) — Apache 2.0, 105K+ GitHub stars
4. **OpenCode** (`opencode`) — MIT, 180K+ stars, 75+ sağlayıcı
5. **Aider** (`aider`) — Apache 2.0, Git-aware pair-programming
6. **Cline / Roo Code** (`cline`) — VS Code kökenli, MCP-öncelikli (CLI modu)
7. **Kilo Code** (`kilo`) — MIT, 500+ model, BYOK, 0 markup
8. **GitHub Copilot CLI** (`gh copilot`) — Resmi GitHub entegrasyonu
9. **Cursor Agent CLI** (`cursor-agent`) — Cursor IDE'nin CLI yüzü
10. **Qwen Code** (`qwen`) — Alibaba açık kaynak CLI

İkinci dalga: Amazon Q Developer, Tabnine, Devin (Cognition), Zed AI, Replit Agent, Augment Code.

### 3.5 Projeksiyon: Neden Bu Stack 2026'da Üretim Sınıfı?

- Bu seçimlerin birleştiği nokta: **Webview yoklaması yapmadan yerel, hafif, hızlı, güvenli**.
- Tempest, Terax, Terminon gibi 2026'da çıkan open-source AI terminal projeleri de tam olarak aynı stack'i kullanıyor (Tauri 2 + Rust + portable-pty + xterm.js/WebGL + React 19 + Zustand). Bu demek ki ekosistem olgun, örnek kod ve desenler mevcut.
- Obsidian-tarzı hafıza için kanıtlanmış mimari: obra/knowledge-graph (SQLite + FTS5 + sqlite-vec + graphology), MCP olarak da dağıtılıyor, Claude Code plugin'i var. AjanŞirket aynı kalıbı Rust'ta uygularsa tamamen yerel ve hızlı çalışır.
- A2A ve MCP açık protokoller olduğu için yarın yeni bir ajan/motor çıktığında entegrasyon maliyeti minimum.

---

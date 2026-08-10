# AJANOFİS GELİŞTİRME — KESİN TAVSİYE EDİLEN GITHUB REPOLARI

> **Tarih:** 10 Ağustos 2026
> **Kapsam:** AjanOfis AAA masaüstü uygulaması (Tauri 2 + React 19 + Rust tokio) için üretim seviyesi kanıtlanmış GitHub repoları.
> **Metodoloji:** Web araştırması (depth=3), crates.io, docs.rs ve GitHub doğrulaması. Her repo için neden, nasıl kullanıldığı ve alternatifleri belirtilmiştir.
> **Derece:** **MUST** (zorunlu temel) → **STUDY** (referans mimari / pattern) → **NICE** (opsiyonel) → **AVOID** (kaçın).

---

## KISALTMALAR VE ÖNCELİK SİSTEMİ

| Etiket | Anlamı |
|---|---|
| 🔴 **MUST** | Kod tabanına doğrudan dahil et / çatalla (fork) / referans al. Üretim kalitesinde, ekosistem standardı. |
| 🟠 **STUDY** | Kaynak kodunu oku, mimari kararları ve pattern'leri ödünç al. Doğrudan bağımlılık yapma ama mimari referans. |
| 🟡 **NICE** | Opsiyonel; belirli bir fazda veya edge-case için ihtiyaç olursa kullan. |
| ⚫ **AVOID** | Kullanma; deprecated, security risk, ya da AjanOfis mimarisiyle uyumsuz. |

---

## 1. TAURI 2 ÇEKİRDEK VE PROJE İSKELETİ

### 🔴 MUST `tauri-apps/tauri` (107k★)
**URL:** https://github.com/tauri-apps/tauri
**Neden:** Çekirdek framework v2.9.6. Tüm pencere, event, command ve güvenlik altyapısı. Tauri ekibinin resmi deposu; pluginler, CLI ve updater bu monorepo'dan besleniyor.
**Nasıl kullan:** Cargo.toml'da `tauri = { version = "2", features = ["tray-icon", "devtools"] }`; production build'de devtools feature'ını kapat.

### 🔴 MUST `tauri-apps/create-tauri-app`
**URL:** https://github.com/tauri-apps/create-tauri-app
**Neden:** Resmi scaffold aracı. React+TS+Vite+pnpm combo'su için `pnpm create tauri-app --template react-ts` komutu AjanOfis için en temiz başlangıç noktası.
**Nasıl kullan:** İlk commit'ten sonra projeyi iskeletten çıkarıp kendi mimarine evrilt.

### 🟠 STUDY `dannysmith/tauri-template`
**URL:** https://github.com/dannysmith/tauri-template
**Neden:** Tauri 2 + React 19 + TypeScript + Tailwind 4 + shadcn/ui + Zustand + TanStack Query + command palette + auto-update (GitHub Actions) + notification + preferences pane + unified title bar. AjanOfis ile tam olarak aynı teknoloji yığını.
**Nasıl kullan:** Kaynak kodu incele: title bar yönetimi, release automation, preferences persistence ve komut paleti pattern'leri kopyalanabilir.

### 🟠 STUDY `MrLightful/create-tauri-react`
**URL:** https://github.com/MrLightful/create-tauri-react
**Neden:** bulletproof-react prensipleriyle yapılandırılmış feature-based klasör yapısı; Biome + Husky + lint-staged dahil. AjanOfis'in src/features/ ajan, kanban, memory, terminal modüler klasörleşmesi için doğrudan referans.
**Nasıl kullan:** Klasör yapısını benimse (app/routes, features, components, lib, stores, hooks, utils).

### ⚫ AVOID `MPMcIntyre/ReacTaur_TS`, `guilhermeprokisch/tauri-fastapi-react-app`
**Neden:** İlki deprecated (Create React App + eski Tauri v1), ikincisi Python backend gömüyor (AjanOfis'te tüm backend Rust olmalı).

---

## 2. TERMİNAL / PT YÖNETİMİ (Ajan konsolları için)

### 🔴 MUST `wez/wezterm` — portable-pty
**URL:** https://github.com/wez/wezterm (portable-pty sub-crate)
**Neden:** `portable-pty` WezTerm'den çıkan, ConPTY (Windows) ve POSIX PTY (macOS/Linux) için tek API veren crate. WezTerm tarafından production'da kanıtlanmış (en popüler GPU-terminal). AjanOfis'in her ajan paneli için PTY başlatmasında çekirdek kütüphane.
**Nasıl kullan:** Cargo.toml'da `portable-pty = "0.8"`. PTY master/slave ayrımı, CommandBuilder, Child kill, resize yönetimi birebir kullanılacak.

### 🔴 MUST `xtermjs/xterm.js` (18k★)
**URL:** https://github.com/xtermjs/xterm.js
**Neden:** Web tabanlı terminal renderer. xterm-addon-fit, xterm-addon-web-links, xterm-addon-search, WebGL renderer ile kullan. Tauri webview içinde her ajan için bir terminal paneli render etmek için standart.
**Nasıl kullan:** `pnpm add xterm @xterm/addon-fit @xterm/addon-web-links @xterm/addon-search`; WebGL renderer'ı zorunlu kıl (Canvas renderer 10K+ satırda yavaş kalır).

### 🟠 STUDY `Shabari-K-S/terminon`
**URL:** https://github.com/Shabari-K-S/terminon
**Neden:** Tauri 2 + React 19 + portable-pty + xterm.js stack'inin çalışan tam örneği. Multi-tab + split, borderless custom title bar, glassmorphism. AjanOfis'in çoklu ajan terminal panelleri için en yakın referans mimari.
**Nasıl kullan:** PTY reader thread'inin Tauri event ile frontend'e nasıl bağlandığını, resize/shell detection kodunu incele.

### 🟠 STUDY `yofabr/tauri-pty` (agent skill)
**URL:** https://github.com/yofabr/tauri-pty
**Neden:** Tauri 2 + React için tam PTY kurulum rehberi (SKILL.md + 5 referans doküman: rust-backend, react-frontend, xterm-integration, multi-tab, best-practices). Adım adım implementation kılavuzu.
**Nasıl kullan:** Terminal modülünü yazarken checklist olarak kullan.

### 🟡 NICE `Tnze/tauri-plugin-pty`
**URL:** https://github.com/Tnze/tauri-plugin-pty
**Neden:** Hazır Tauri plugin olarak PTY spawn, i/o stream, resize yönetimi. Vanilla JS example mevcut.
**Neden MUST değil:** AjanOfis kendi PTY yöneticisini Rust tarafında `AgentHandle` içine gömecek (plugin abstraction'ı ajan izolasyonu/bütçe/onay için yetersiz). Ancak plugin pattern'ini inceleyebilirsin.

### 🟡 NICE `marc2332/tauri-terminal`
**URL:** https://github.com/marc2332/tauri-terminal
**Neden:** Tauri + xterm.js + portable-pty minimal örnek. Tek terminal için; multi-tab yok ama temel kurulumu görmek için referans.

---

## 3. MCP (MODEL CONTEXT PROTOCOL) RUST ENTEGRASYONU

### 🔴 MUST `modelcontextprotocol/rust-sdk` → crate: `rmcp`
**URL:** https://github.com/modelcontextprotocol/rust-sdk
**Crates.io:** `rmcp` v3.0.1 (resmi MCP organizasyonu tarafından yayınlanıyor)
**Neden:** **Resmi** Rust MCP SDK'sı. 2026-07-28 protokol sürümüyle tam uyumlu. Tokio async, child-process transport, Streamable HTTP, #[tool] macro, sampling/roots/logging/completions/subscriptions/tasks (SEP-2663 long-running tool) hepsi mevcut. AjanOfis'in MCP client'ı (ajanları MCP server'lara bağlama) ve kendi internal MCP server'ı (bilgi grafı/terminal araçlarını expose etme) için kullanılacak.
**Nasıl kullan:**
```toml
[dependencies]
rmcp = { version = "3", features = ["client", "server", "macros", "child-process-transport"] }
```
Client: `TokioChildProcess::new(Command::new("npx").arg("-y").arg("@modelcontextprotocol/server-everything"))`
Server: `#[tool]` macro ile AjanOfis'in hafıza/kanban/graf araçlarını dışa aç.

### 🟡 NICE `rust-mcp-stack/rust-mcp-sdk` → crate: `rust-mcp-sdk` v1.0.1
**URL:** https://github.com/rust-mcp-stack/rust-mcp-sdk
**Neden:** Alternatif topluluk SDK; 2025-11-25 protokol sürümü, Axum/Actix HTTP server, 100% conformance testleri, procedural macros. Resmi `rmcp` yetersiz kalırsa (örneğin OAuth veya özel auth) yedek.
**Not:** rmcp v3.0'da streamable-http + OAuth desteği geldiği için şu an ihtiyaç yok. İzlemeye al.

### ⚫ AVOID `darinkishore/mcp_client_rust`, `jeanlucthumm/modelcontextprotocol-rust-sdk`
**Neden:** Eski/erken-dönem çalışmalar; tam spec coverage yok; aktif bakım yok. `mcp_client_rs` sadece stdio, sampling/roots yok.

### 🟠 STUDY `conikeec/mcp-probe`
**URL:** GitHub `conikeec/mcp-probe` (mcp-probe MCP debug aracı, Rust)
**Neden:** MCP server'larını test/debug etmek için production-ready CLI. Geliştirme sırasında AjanOfis'in MCP server'ını doğrulamak için kullan.

---

## 4. A2A (AGENT-TO-AGENT) PROTOKOLÜ — AJANLAR ARASI İLETİŞİM

### 🟠 STUDY `tomtom215/a2a-rust` → crate: `a2a-protocol-sdk` v0.5
**URL:** https://github.com/tomtom215/a2a-rust
**Neden:** A2A v1.0.0 için tam saf Rust implementasyonu. Quad transport (JSON-RPC 2.0 / REST / WebSocket / gRPC), SSE streaming, push notification (webhook), agent-card discovery, JWS/ES256 signing, HTTP caching (ETag), retry policy, enterprise hardening (body limit, SSRF, CORS, TTL). 81 E2E testi var. TCK conformance mevcut.
**Nasıl kullan:** Faz 3'te ajanlar arası doğrudan iletişim (CEO → CTO → Developer hiyerarşisi) için A2A kullanacağız. Önce internal event bus ile başla, Faz 4'te bu crate'i entegre et ve ajan-card'lar ile dış dünyaya açıl.

### 🟡 NICE `EmilLindfors/a2a-rs`
**URL:** https://github.com/EmilLindfors/a2a-rs
**Neden:** Hexagonal mimari, ConnectRPC, HTTP (Axum), SQLite/Postgres storage (SQLx), JWT/OAuth2, a2a-mcp köprüsü. Modüler crate yapısı. `a2a-rust`'a alternatif ama onun kadar conformance testi yok.

### 🟡 NICE `qntx/ra2a`
**URL:** https://github.com/qntx/ra2a
**Neden:** v1.0 spec (12 Mart 2026), event-driven server, Axum router mount, transport-agnostic client (AgentCard.supported_interfaces'dan otomatik seçim), SSE streaming, HMAC-SHA256 webhook push. Daha minimal, öğrenmek için kolay.

### 🔴 MUST `a2aproject/a2a-js` (resmi JS/TS SDK)
**URL:** https://github.com/a2aproject/a2a-js
**Neden:** Resmi TypeScript SDK. Frontend (React) tarafında task yaşam döngüsü (submitted→working→input-required→completed/failed/canceled) durumlarını görüntülemek için A2A tiplerini ve client'ı kullan. Rust tarafı a2a-rust ile, frontend tarafı bununla.

### 🟡 NICE `a2aproject/a2a-rs` (resmi Rust SDK)
**URL:** https://github.com/a2aproject/a2a-rs
**Neden:** Linux Foundation resmi Rust SDK (a2a organizasyonu altına taşındı, 55★). a2aproject repos altında göründü. Tom215'in a2a-rust ile karşılaştır — hangisi resmi olarak benimsenirse onu kullan.

### 🟠 STUDY `ai-boost/awesome-a2a`
**URL:** https://github.com/ai-boost/awesome-a2a
**Neden:** Ekosistem katalog. Tüm SDK, tool, framework ve platformları listeliyor. A2A entegrasyonu öncesi son durumu kontrol etmek için kullan.

---

## 5. BİLGİ GRAFI — HAFIZA VE VEKTÖR ARAMA

### 🔴 MUST `rusqlite/rusqlite` + bundled feature
**URL:** https://github.com/rusqlite/rusqlite
**Neden:** SQLite için standart Rust kütüphanesi. `bundled` feature'ı ile libsqlite3'ü statik linkle (Tauri binary'sine göm). WAL mode kullan (concurrent read/single write). AjanOfis'in tüm state'i (agents, tasks, events, settings, entities, edges) tek SQLite dosyasında.

### 🔴 MUST `asg017/sqlite-vec` → crate: `sqlite-vec` (Rust binding)
**URL:** https://github.com/asg017/sqlite-vec
**Neden:** SQLite için vektör benzerlik eklentisi (SIMD, HNSW). vec0 sanal tablo 384-dim float BLOB. Tek dosya DB, external vektör DB (Qdrant/Pinecone) gerekmez. AjanOfis semantic search'ü bununla.

### 🟠 STUDY (MUST-aday) `hiyenwong/sqlite-knowledge-graph` v0.13
**URL:** https://github.com/hiyenwong/sqlite-knowledge-graph
**Neden:** Rust'ta yazılmış hazır bilgi grafı kütüphanesi: entity/edge CRUD, BFS/DFS traversal, shortest path, PageRank, Louvain community, connected components, vector search, SmartVector four-signal retrieval, ripple propagation, QuaQue versioning, async API (tokio), paper-driven iki-aşamalı RAG engine (RagConfig top_k_candidates/top_k_rerank, graph expansion). all-MiniLM-L6-v2 384-dim varsayılan.
**Nasıl kullan:** Doğrudan crate olarak çek (`cargo add sqlite-knowledge-graph`) veya kendi grafnı bu API'ye göre şekillendir. Ciddi alternatif: kendin yazmak yerine bunun üstüne inşa et.

### 🟠 STUDY `fluencerlabs/sqlite-graph` → crate: `sqlite-graph` v0.1.0
**URL:** crates.io/crates/sqlite-graph
**Neden:** Recursive CTE traversal, bi-temporal edges (valid_from/valid_until + recorded_at), FTS5 trigger-sync, RRF hybrid, Jaro-Winkler fuzzy dedup, 384-dim BLOB, single file. Daha minimal, grafiği tam olarak kontrol etmek istiyorsan iyi bir temel.

### 🟠 STUDY `obra/knowledge-graph`
**URL:** https://github.com/obra/knowledge-graph
**Neden:** Claude Code kullanıcısı Jesse Vincent'ın referans bilgi grafı implementasyonu. SQLite + FTS5 + sqlite-vec + graphology + Xenova/all-MiniLM-L6-v2 (22 MB ONNX). TypeScript. Tam da AjanOfis'in yapmak istediği mimariyi production'da kanıtlamış. Kodunu oku, schema ve retrieval pattern'ini ödünç al.

### 🟡 NICE `bozbuilds/AIngram`
**URL:** https://github.com/bozbuilds/AIngram
**Neden:** Tek SQLite dosyasında üç sinyal (FTS5 + sqlite-vec/QJL + knowledge graph) + RRF fusion + Ed25519 imzalı entry + MCP server. Python ama mimari konsept (QJL quantization, crypto signing) referans.

### 🟡 NICE `iAchilles/memento`
**URL:** https://github.com/iAchilles/memento
**Neden:** MCP memory server (SQLite + FTS5 + sqlite-vec, BGE-M3 1024-dim, entity/observation/relation graph). Claude Desktop ile hazır entegre. Önce kendi çözümünü yazmadan MCP üzerinden deneyebilirsin.

### ⚫ AVOID Neo4j / external vector DB'ler (Pinecone, Qdrant, Chroma)
**Neden:** AAA desktop app için ağır bağımlılıklar; ayrı servis gerektirir; Tauri felsefesine (tek binary, zero-install) aykırı. SQLite + sqlite-vec + FTS5 100K entity'ye kadar yeterli (benchmark: ~50ms/query).

---

## 6. AI CLI ORKESTRASYONU — REFERANS MİMARİLER

AjanOfis'in en önemli katmanı AgentAdapter ve orkestrasyon. Aşağıdaki açık kaynak projeleri **STUDY** et — doğrudan kullanma ama mimari/API/pattern ödünç al:

### 🟠 STUDY `awslabs/cli-agent-orchestrator` (CAO)
**URL:** https://github.com/awslabs/cli-agent-orchestrator
**Neden:** AWS Labs'ın tmux tabanlı çoklu ajan orkestratörü (Claude Code, Kiro, Codex, Antigravity, Hermes, Kimi, Copilot, OpenCode, Cursor destekli). cao-server + tmux session isolation + supervisor/worker model. 10+ provider için spawn/detect pattern'lerini incele. AjanOfis'in worktree izolasyonuna ek olarak tmux yerine portable-pty kullanacak ama task hiyerarşisi ve provider detection konusu referans.

### 🟠 STUDY `bradAGI/awesome-cli-coding-agents`
**URL:** https://github.com/bradAGI/awesome-cli-coding-agents
**Neden:** Tüm terminal-native ajanların ve orchestratorlerin küratörlü listesi (Ağustos 2026). Hangi ajan hangi yeteneklere sahip (fork, approval, MCP, resume), hangi orchestrator ne yapıyor (Bernstein, Parallel Code, Traycer, h5i, kodo, OpenCastle, ORCH). AjanOfis'in destekleyeceği ajanları (Claude Code, Codex, Gemini, OpenCode, Aider, Cline/Roo, Kilo, Copilot, Cursor, Qwen) seçerken ve AgentAdapter trait'ini tasarlarken tek kaynak.

### 🟠 STUDY `agent-of-empires/agent-of-empires` (AoE)
**URL:** https://github.com/agent-of-empires/agent-of-empires
**Neden:** **Rust** ile yazılmış çoklu ajan session manager (TUI + web dashboard + PWA). 15+ ajan destekli (Claude, OpenCode, Vibe, Codex, Gemini, Antigravity, Cursor, Copilot, Pi, OMP, Factory, Hermes, Kiro, Qwen, Kimi). Agent Client Protocol ile structured event'ler; Docker sandbox; Tailscale Funnel ile uzaktan erişim. AjanOfis ile aynı sorunu (Rust multi-agent desktop app) çözüyor. Cargo build ile tek binary. **En yakın mimari akraba** — mutlaka kaynak kodunu incele (özellikle agent detection ve event parsing).

### 🟠 STUDY `parallelcodeapp/parallel-code`
**URL:** (bradAGI listesinde referanslı)
**Neden:** Git worktree izolasyonu ile birden fazla CLI ajanı (Claude Code, Codex, Gemini) aynı anda çalıştıran desktop app. Worktree + port offset pattern'i AjanOfis için direkt referans.

### 🟠 STUDY `asheshgoplani/agent-deck`
**URL:** https://github.com/asheshgoplani/agent-deck
**Neden:** Go + Bubble Tea ile TUI ajan session manager. Claude/Gemini/OpenCode/Codex/Copilot/Crush/Cursor/Hermes/Custom araçları için fork, conductor (CLAUDE.md/AGENTS.md/POLICY.md/LEARNINGS.md), durum tespiti pattern'leri. Fork mantığını (bağlam mirası) AjanOfis'in task fork özelliği için incele.

### 🟡 NICE `kingbootoshi/codex-orchestrator`
**URL:** https://github.com/kingbootoshi/codex-orchestrator
**Neden:** Claude Code plugin olarak Codex ajanlarını tmux'ta koşturma pattern'i. Claude→Codex handoff referans ama JS/Bun. Rust'ta aynı paterni yeniden yazmak için fikir verir.

### 🟡 NICE `hoangsonww/AI-Agents-Orchestrator`
**URL:** https://github.com/hoangsonww/AI-Agents-Orchestrator
**Neden:** Adaptör mimarisi (Claude/Codex/Gemini/Copilot/Ollama), 7 built-in workflow, Nuxt 3 + Flask + Socket.IO UI + REPL. İki mod: pipeline orchestration ve agentic team (free role-to-role, lead-gated final). AjanOfis'in CEO-merkezli lead-gated yaklaşımı için fikir verici.

### ⚫ AVOID CodePilot, OpenCode UI, Cline Desktop gibi Electron+Next.js ağır desktop app'ler
**Neden:** AjanOfis Tauri ile hafiflik hedefliyor; bu app'ler 300-800 MB bundle üretiyor. İlham al ama mimari kopyalama.

---

## 7. REACT GRAF GÖRSELLEŞTİRME (Bilgi Grafı için)

### 🔴 MUST `jacomyal/sigma.js` v3 + `graphology`
**URL:** https://github.com/jacomyal/sigma.js
**Neden:** WebGL hızlı graf renderer. 10K+ düğüm/saniye frame. Graphology ile aynı ekip tarafından geliştiriliyor. React binding'leri mevcut. Tauri WebView'da DOM/Canvas yetmediğinde WebGL şart.

### 🔴 MUST `sim51/react-sigma-v2` → npm: `@react-sigma/core` + `@react-sigma/layout-forceatlas2`
**URL:** https://github.com/sim51/react-sigma-v2
**Neden:** React için Sigma binding. useSigma, useLoadGraph, useRegisterEvents, useSetSettings hook'ları; ZoomControl, SearchControl, ControlsContainer, ForceAtlasControl. Worker tabanlı layout'lar (FA2 ForceAtlas2 WebWorker'da çalışıyor).

### 🟠 STUDY `johnymontana/sigma-graph-examples`
**URL:** https://github.com/johnymontana/sigma-graph-examples
**Neden:** 14 örnek (başlangıçtan ileri seviyeye): drag&drop, layout switch, search/filter, minimap, dark theme, dynamic data, network metrics (centrality), multi-graph karşılaştırma, Neo4j tarzı property graph. AjanOfis bilgi grafı görünümü için hazır örnekler.

### 🟡 NICE `graphology/graphology`
**URL:** https://github.com/graphology/graphology
**Neden:** JS graph veri yapısı kütüphanesi. Sigma'nın zorunlu bağımlılığı; graph traversal, metrics (centrality, modularity), layout algoritmaları (FA2, circular, force), seri/de-serializasyon. Frontend state'inde ajan durum grafı ve entity grafı olarak graphology kullanılacak.

### ⚫ AVOID D3.js force graph, react-force-graph, Cytoscape
**Neden:** 1000+ node'da performans düşüşü (SVG/DOM tabanlı). AjanOfis'in bilgi grafı ajan geçmişi/entity/edge ile 10K+'ya ulaşabilir; WebGL zorunlu. Sadece 3D opsiyonel görünüm için `react-force-graph-3d` (Three.js) düşünülür.

---

## 8. REACT KANBAN + DRAG&DROP

### 🔴 MUST `clauderic/dnd-kit` (`@dnd-kit/core`, `@dnd-kit/sortable`, `@dnd-kit/modifiers`)
**URL:** https://github.com/clauderic/dnd-kit
**Neden:** Modern React drag&drop kütüphanesi (react-beautiful-dnd'nin yerine geçti). Accessibility, keyboard, touch desteği, composable sensor/modifier API. Kanban kartlarını sütunlar arasında taşımak için standart.

### 🟠 STUDY `Georgegriff/react-dnd-kit-tailwind-shadcn-ui`
**URL:** https://github.com/Georgegriff/react-dnd-kit-tailwind-shadcn-ui
**Neden:** `@dnd-kit + Tailwind + shadcn/ui` ile yapılmış erişilebilir Kanban board. Demo: georgegriff.github.io. Keyboard, screen-reader duyuruları, tam erişilebilirlik. AjanOfis'in görevlendirme için kanban→masa drag&drop'u ile aynı primitivleri kullanıyor.

### 🟠 STUDY `janhesters/shadcn-kanban-board`
**URL:** GitHub `janhesters/shadcn-kanban-board`
**Neden:** Sıfır bağımlı, erişilebilirlik-öncelikli production Kanban. Eğer dnd-kit'i minimumda tutmak istersen referans.

### 🟡 NICE `mehrdadrafiee/recursive-dnd-kanban-board`
**URL:** https://github.com/mehrdadrafiee/recursive-dnd-kanban-board
**Neden:** Recursive (iç içe) kolon desteği; Next.js + dnd-kit + shadcn. Task hierarchy (epic→task→subtask) AjanOfis'te gerekirse referans.

---

## 9. TAURI PLUGIN EKOSİSTEMİ

AjanOfis'te kullanacağın resmi Tauri v2 plugin'leri (tümü `tauri-apps/plugins-workspace` altından, `cargo tauri add <plugin>` ile eklenir):

| Crate / Plugin | Amaç | Kullanım |
|---|---|---|
| 🔴 `tauri-plugin-dialog` | Native dosya seçme/mesaj kutusu | Proje seç, import/export bilgi grafı |
| 🔴 `tauri-plugin-shell` | Sidecar/child process spawn etme | 🔴 Shell access ajan onay akışında (izin yönetimi burada kritik) |
| 🔴 `tauri-plugin-notification` | Native OS notification | Ajan blocked/error/approval gerektiğinde kullanıcıya bildirim |
| 🔴 `tauri-plugin-store` | Key-value persistent config | Settings, tercihler, son kullanılan projeler |
| 🔴 `tauri-plugin-updater` | Auto-update (ED25519 imzalı) | AAA release zorunluluğu |
| 🔴 `tauri-plugin-single-instance` | Tek pencere | AjanOfis'in singleton çalışması |
| 🔴 `tauri-plugin-window-state` | Pencere boyut/konum hafızası | UX için |
| 🟡 `tauri-plugin-autostart` | Sistemin açılışında çalış | Opsiyonel |
| 🟡 `tauri-plugin-clipboard-manager` | Clipboard | Ajan çıktısının kopyalanması |
| 🟡 `tauri-plugin-global-shortcut` | Kısayol (Ctrl/Cmd+Shift+X kill-switch) | Kill switch ve hızlı çağrı |
| 🟡 `tauri-plugin-http` | Rust HTTP client (Reqwest üstü) | Plugin çağrısı / güncelleme kontrolü |
| 🟡 `tauri-plugin-log` | Yapılandırılabilir log | Dev/prod loglama |
| 🟡 `tauri-plugin-fs` | Dosya sistemi erişimi | Proje klasörü için scope-based izin |
| 🟡 `tauri-plugin-os` | OS bilgisi | Platform detection |
| 🟡 `tauri-plugin-positioner` | Pencereyi dock/sağ-üst vb konumlandır | Opsiyonel |

### 🔴 MUST `tauri-apps/plugins-workspace`
**URL:** https://github.com/tauri-apps/plugins-workspace
**Neden:** Yukarıdaki tüm resmi pluginlerin monorepo'su. Capability-based permission sistemi (AjanOfis güvenlik modeli için kritik) buradan yönetiliyor.

### 🟡 NICE `HuakunShen/tauri-plugin-keyring`
**URL:** https://github.com/HuakunShen/tauri-plugin-keyring
**Neden:** OS keychain için Tauri plugin wrapper (keyring crate'i üstünde). API key'leri saklamak için kullan (sk-ant-... vs). Alternatif olarak doğrudan keyring crate'i kullan.

---

## 10. GİZLİLİK/GÜVENLİK

### 🔴 MUST `open-source-cooperative/keyring-rs` (crate: `keyring`)
**URL:** https://github.com/open-source-cooperative/keyring-rs
**Neden:** Cross-platform keychain: macOS Keychain Services, Windows Credential Manager, Linux Secret Service (GNOME Keyring/KWallet). API anahtarlarını, token'ları plaintext config dosyasına yazmak yerine OS keychain'de tutmak için zorunlu.

### 🟠 STUDY `HardyKrustacean/destructive_command_guard` (dcg)
**URL:** (claude-hardening ekosistemi içinde geçiyor — crates.io üzerinden bakılacak, kaynak: cc.bruniaux.com security hardening raporu)
**Neden:** Rust+SIMD ile yazılmış destructive command guard. 49 güvenlik paketi regex'i; hızlıca (mikrosaniyelerle) `rm -rf /`, `mkfs`, `curl ... | bash` gibi yıkıcı komutları tespit. Claude Code hook'ları için tasarlanmış ama AjanOfis'in onay katmanına (PreToolUse/command guard) Rust tarafında doğrudan entegre edilebilir.

### 🟡 NICE `tauri-apps/tauri-plugin-stronghold`
**URL:** Tauri plugins-workspace altında
**Neden:** IOTA Stronghold tabanlı şifreli yerel depolama. P2P ajan mesh veya paylaşımlı bilgisayar senaryolarında ek güvenlik için.

---

## 11. YEREL EMBEDDİNG / ONNX RUNTIME (Bilgi grafı vektörleri için)

### 🔴 MUST `pykeio/ort` (ONNX Runtime Rust binding)
**URL:** https://github.com/pykeio/ort
**Neden:** Microsoft ONNX Runtime için en olgun Rust wrapper. CPU/GPU/ WASM, dynamic load veya static link. Hugging Face TEI (Text Embeddings Inference), FastEmbed-rs, Magika gibi projeler production'da kullanıyor. all-MiniLM-L6-v2'yi ONNX'e çevirip bununla 384-dim embedding üretmek için standart.
**Dikkat:** C++ libonxruntime bağımlılığı var (direct download feature'ı ile otomatik indirilebilir veya sistemde kurulu olanı kullan).

### 🟠 STUDY `Anush008/fastembed-rs`
**URL:** (ort kullanan projeler listesinde geçiyor)
**Neden:** Rust ile fast text embedding üretimi (all-MiniLM-L6-v2, BGE serisi). ort crate'i üstünde, quantized modellerle çok hızlı. Eğer ort'u doğrudan kullanmak çok low-level kalırsa hazır seçenek.

### 🟡 NICE `huggingface/candle`
**URL:** https://github.com/huggingface/candle
**Neden:** Saf Rust ML framework. C++ bağımlılığı yok. ONNX modelleri import edilebilir; ama CPU performansı ort'tan daha düşük. Eğer binary dağıtımında C++ runtime kısıtı varsa yedek seçenek. AjanOfis için C++ runtime'ı Tauri binary'sine gömmek sorun değilse ort tercih et.

### 🟡 NICE `jwnz/sentence-transformers-rs`
**URL:** GitHub `jwnz/sentence-transformers-rs`
**Neden:** Rust sentence-transformers portu. Embedding'ler neredeyse aynı; performans kıyaslanabilir. candle veya ort üzerine kurulu.

### ⚫ AVOID rust-bert (torch bağımlılığı)
**Neden:** Libtorch 200MB+ ekler; AAA desktop app için binary boyutunu patlatır.

---

## 12. MARKDOWN / WİKİ-LINK İŞLEME

### 🔴 MUST `flowershow/remark-wiki-link` (yeni isim: `@flowershow/remark-wiki-link`)
**URL:** https://github.com/flowershow/remark-wiki-link
**Neden:** Obsidian tarzı `[[hedef]]`, `[[hedef#başlık|alias]]`, `^blockid`, `![[embed]]` sözdizimini parse/rehype eden remark plugin. `format: "shortestPossible"` modu Obsidian ile aynı shortest-path çözümlemesi yapar. AjanOfis'in markdown vault'undaki wiki-link'leri render etmek için kullan.
**Not:** Eski paket `remark-wiki-link-plus` yerine `@flowershow/remark-wiki-link` (v1.1.1+, aktif bakım). Plus sürümü deprecated.

### 🟡 NICE `micromark/micromark-extension-wiki-link` + `syntax-tree/mdast-util-wiki-link`
**URL:** remark-wiki-link'in bağımlılıkları, düşük seviyeli parser'lar. Kendi custom processor'unu yazmak gerekirse kullan.

---

## 13. DİĞER FRONTEND/REACT KÜTÜPHANELERİ

| Paket | Amaç | Derece |
|---|---|---|
| `tailwindlabs/tailwindcss` v4 | CSS framework | 🔴 MUST |
| `shadcn-ui/ui` | Bileşenler (command, dialog, dropdown, tabs, card, avatar, badge) | 🔴 MUST |
| `pmndrs/zustand` | Global state (ajanlar, kanban, active view) | 🔴 MUST |
| `tanstack/virtual` | Sanal liste (10K event/log için) | 🔴 MUST |
| `framer/motion` | Animasyon (ajan durum renk geçişleri) | 🟠 NICE |
| `react-hook-form` + `colinhacks/zod` | Form + validation (onay dialogları, ayarlar) | 🔴 MUST |
| `xyflow/react` (React Flow) | Toplantı/bağımlılık grafı (Sigma'ya ek olarak) | 🟡 NICE |
| `react-hook-form/resolvers` | Zod resolver | 🔴 MUST |
| `TanStack/table` | Tablo (maliyet dökümü, event log) | 🟡 NICE |
| `prc5/zoom-pan-pinch` (`react-zoom-pan-pinch`) | Ofis katı zoom/pan | 🟠 NICE |
| `sindresorhus/ky` veya `tauri-plugin-http` | HTTP (MCP Streamable HTTP) | 🟠 NICE |

---

## 14. GELİŞTİRME VE KALİTE ARACI OLARAK KULLANILACAKLAR

### 🔴 MUST `biomejs/biome`
**URL:** https://github.com/biomejs/biome
**Neden:** ESLint+Prettier'a göre 10-100x hızlı tek tool. TS/JS/JSON format/lint. Pre-commit hook'ta kullan.

### 🔴 MUST Rust tarafında: `tokio-rs/tokio`, `serde-rs/serde`, `tokio-rs/tracing` + `tracing-subscriber`, `dtolnay/async-trait`, `chronotope/chrono`, `uuid-rs/uuid`, `notify-rs/notify`, `sysinfo-rs/sysinfo`, `seanmonstar/reqwest`, `rust-cli/clap`
**Neden:** Tokio async runtime standart. Tracing structured logging. Serde serialization. Async-trait AgentAdapter gibi trait tanımları için. Chrono zaman. UUID görev/ajan ID. notify dosya izleme (markdown vault için). sysinfo sistem kaynak izleme. reqwest HTTP (MCP/A2A client). clap CLI argümanlar.

### 🟠 STUDY `tauri-apps/awesome-tauri`
**URL:** https://github.com/tauri-apps/awesome-tauri
**Neden:** Tauri ekosisteminin tamamı (plugin'ler, örnekler, araçlar, öğrenme kaynakları). Periyodik bakılacak.

---

## 15. CARGO.TOML / PACKAGE.JSON ÖNERİSİ (ÖZET)

**`src-tauri/Cargo.toml` (üretim başlangıç seti):**
```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-dialog = "2"
tauri-plugin-shell = "2"
tauri-plugin-notification = "2"
tauri-plugin-store = "2"
tauri-plugin-updater = "2"
tauri-plugin-single-instance = "2"
tauri-plugin-window-state = "2"
tauri-plugin-fs = "2"
tauri-plugin-http = "2"
tauri-plugin-log = "2"
tauri-plugin-os = "2"
tauri-plugin-global-shortcut = "2"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "process", "signal"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
async-trait = "0.1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
portable-pty = "0.8"
rusqlite = { version = "0.32", features = ["bundled", "chrono"] }
sqlite-vec = "0.1"
sqlite-knowledge-graph = "0.13"
rmcp = { version = "3", features = ["client", "server", "macros", "child-process-transport"] }
a2a-protocol-sdk = "0.5"  # ya da seçilen A2A crate
keyring = "2"
ort = { version = "2", features = ["load-dynamic"] }  # ONNX embedding
sysinfo = "0.32"
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
notify = "7"
clap = { version = "4", features = ["derive"] }
anyhow = "1"
thiserror = "1"
```

**`package.json` (üretim başlangıç seti):**
```json
{
  "dependencies": {
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-dialog": "^2",
    "@tauri-apps/plugin-shell": "^2",
    "@tauri-apps/plugin-notification": "^2",
    "@tauri-apps/plugin-store": "^2",
    "react": "^19",
    "react-dom": "^19",
    "zustand": "^5",
    "@tanstack/react-virtual": "^3",
    "xterm": "^5",
    "@xterm/addon-fit": "^0.10",
    "@xterm/addon-web-links": "^0.10",
    "@xterm/addon-search": "^0.10",
    "@xterm/addon-webgl": "^0.18",
    "sigma": "^3",
    "graphology": "^0.25",
    "@react-sigma/core": "^4",
    "@react-sigma/layout-forceatlas2": "^4",
    "@dnd-kit/core": "^6",
    "@dnd-kit/sortable": "^8",
    "@dnd-kit/modifiers": "^7",
    "framer-motion": "^11",
    "react-hook-form": "^7",
    "zod": "^3",
    "@hookform/resolvers": "^3",
    "@xyflow/react": "^12",
    "@tanstack/react-table": "^8",
    "react-zoom-pan-pinch": "^3",
    "unified": "^11",
    "remark-parse": "^11",
    "remark-rehype": "^11",
    "rehype-react": "^8",
    "@flowershow/remark-wiki-link": "^1",
    "class-variance-authority": "^0.7",
    "clsx": "^2",
    "tailwind-merge": "^2",
    "lucide-react": "^0.468"
  }
}
```

---

## 16. SON ÖZET LİSTE (KOPYALA-YAPIŞTIR CHECKLIST)

### İlk Hafta (Faz 0 — İskelet)
- [x] `tauri-apps/create-tauri-app --template react-ts` ile scaffold
- [ ] Bulletproof-react klasör yapısını kur (`MrLightful/create-tauri-react` referans)
- [ ] Tüm resmi pluginleri ekle (dialog, shell, notification, store, updater, single-instance, window-state, fs, log, os)
- [ ] Biome + Husky + lint-staged + GitHub Actions release CI (`dannysmith/tauri-template` referans)
- [ ] xterm.js + WebGL renderer + fit/weblinks/search addon'ları
- [ ] Zustand global state şeması (AgentStore, TaskStore, MemoryStore, UISettingsStore)
- [ ] shadcn/ui init + Tailwind v4

### Faz 1 (Tek Ajan + Terminal)
- [ ] portable-pty ile PTY yönetim katmanı (bkz. `Shabari-K-S/terminon`, `marc2332/tauri-terminal`)
- [ ] Claude Code Adapter ilk concrete implementasyon
- [ ] Event bus (Tauri Channel + tokio::sync::broadcast)
- [ ] rusqlite + WAL mode; events/tasks/agents tabloları

### Faz 2 (Çoklu Motor + Worktree + Hire)
- [ ] AgentAdapter trait + spawn/approval/kill/event lifecycle
- [ ] Tüm CLI ajanları için detect/install logic (`bradAGI/awesome-cli-coding-agents` ve `awslabs/cli-agent-orchestrator` referans)
- [ ] git2 ile worktree yönetimi; port offset
- [ ] dnd-kit kanban (`Georgegriff/react-dnd-kit-tailwind-shadcn-ui` referans)
- [ ] Keyring entegrasyonu (API key saklama)

### Faz 3 (CEO + Politika + Bütçe)
- [ ] Policy engine (allow/allow-always/ask/deny); regex guard (dcg referans)
- [ ] Bütçe takibi (tokio time + dönen token sayımı)
- [ ] Lead-gated output (CEO onayı)
- [ ] A2A entegrasyonu (tomtom215/a2a-rust veya resmî a2a-rs)

### Faz 4 (Bilgi Grafı + MCP Hafıza)
- [ ] sqlite-vec vec0 kurulumu (384-dim)
- [ ] sqlite-knowledge-graph ile entity/edge katmanı
- [ ] ort + all-MiniLM-L6-v2 ONNX ile lokal embedding
- [ ] Sigma.js + graphology + ForceAtlas2 ile ofis/bilgi grafı render (`johnymontana/sigma-graph-examples` referans)
- [ ] remark-wiki-link ile markdown vault render
- [ ] rmcp ile MCP server ve client

### Faz 5 (Toplantı + Paketleme + v1.0)
- [ ] React Flow ile toplantı/bağımlılık view
- [ ] Auto-update (ED25519)
- [ ] Code signing (macOS Developer ID, Windows EV cert)
- [ ] Audit log append-only JSONL
- [ ] Kill switch (global shortcut Ctrl/Cmd+Shift+X)
- [ ] Tauri updater son test, GitHub Releases

---

## Kaynakça
- Starterpick Tauri boilerplates rehberi (Mar 2026)
- docs.rs/rmcp/3.0.1, modelcontextprotocol/rust-sdk (Temmuz 2026)
- crates.io/rust-mcp-sdk/1.0.1 (Haziran 2026)
- crates.io/sqlite-graph/0.1.0, crates.io/sqlite-knowledge-graph/0.13
- tomtom215/a2a-rust (v0.5, TCK conformance)
- bradAGI/awesome-cli-coding-agents (Ağu 2026)
- awslabs/cli-agent-orchestrator (CAO)
- agent-of-empires/agent-of-empires (Rust multi-agent)
- pykeio/ort, johnymontana/sigma-graph-examples
- flowershow/remark-wiki-link, Georgegriff/react-dnd-kit-tailwind-shadcn-ui
- cc.bruniaux.com security hardening (dcg/guard araçları)
- ai-boost/awesome-a2a (A2A ekosistem kataloğu)

---
*Bu rapor AjanOfis Faz 0 öncesi dependency/architecture karar dokümanı olarak kullanılır. Tüm MUST repoları Ağustos 2026 itibarıyla aktif bakımda ve production kullanım kanıtına sahiptir.*

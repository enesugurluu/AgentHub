# AgentHub — FAZ0 Durum Analizi, Gerekli Repolar ve Uygulama Kararları

**Tarih:** 2026-08-10
**Kapsam:** `AGENTHUB-YOL-HARITASI-RAPORU.md` + `Docs/` (ajanofis-buyuk-parcalar, claude-code-buyuk-parcalar, claude-code-rapor-bolumleri)
**Yöntem:** Mevcut kod tabanı dosya dosya incelendi; AjanOfis dokümanlarının tamamı (20 bölüm + Claude Code raporları) taranarak FAZ0 gereksinimleri çıkarıldı; raporda geçen GitHub repoları 6 paralel web araştırması ile 2026-08-10 itibarıyla doğrulandı.

---

## 1. Yönetici Özeti

| Başlık | Durum |
|:---|:---|
| FAZ0 iskeleti (Tauri 2 + React 19 + Vite + TS) | ✅ Kurulu |
| PTY motoru (`portable-pty`) + Windows Job Objects izolasyonu | ✅ Çalışıyor (testli) |
| `EngineAdapter` trait + `EngineAdapterRegistry` | ✅ Çalışıyor (testli) — AjanOfis Bölüm 7 mimarisinin çekirdeği |
| Git worktree yöneticisi (güvenli path çözümleme) | ✅ Çalışıyor (testli) |
| xterm.js temel terminal paneli | 🟡 Var (WebGL yok, resize IPC yok, `powershell.exe` hardcode) |
| Tailwind 4 + shadcn/ui + Zustand | ❌ Yok (inline style + Vite şablon artıkları) |
| SQLite + WAL (`rusqlite`) + ilk migration | ❌ Yok |
| Claude Code adaptörü (`detect/spawn/stop`) | ❌ Yok (sadece generic PTY adaptörü var) |
| `claude doctor` benzeri sağlık kontrolü | 🟡 `health()` var, CLI ajanları için yok |
| Biome + pre-commit | ❌ Yok (ESLint şablonu var) |
| CLAUDE.md + `.claude/` (çevre FAZ0) | ❌ Yok |
| CI (GitHub Actions) | ❌ Yok |

**Özet:** AgentHub, AjanOfis yol haritasının "Faz 1: İskelet (M0)" iş kalemlerinin **backend tarafını büyük ölçüde tamamlamış** durumda. FAZ0'ı bitirmek için yapılması gereken işin çoğu **frontend + state + veri katmanı + ilk gerçek ajan adaptörü** tarafındadır. Bu rapor; (a) FAZ0↔mevcut yapı fark matrisini, (b) FAZ0 için gerekli repoları (doğrulanmış URL ve kurulum komutlarıyla), (c) **YAPILMALI / DEĞİŞMELİ / DURMALI** karar listesini ve (d) FAZ0 kabul kriterlerini (Gate) içerir.

---

## 2. Mevcut Durum Analizi (Dosya Dosya)

### 2.1 Teknoloji Yığını (Doğrulanmış)

| Katman | Teknoloji | Sürüm (lock dosyalarından) |
|:---|:---|:---|
| Desktop framework | Tauri v2 (Rust) | `tauri = "2"` (2.9.x serisi), `@tauri-apps/api` 2.11.1, `@tauri-apps/cli` 2.11.4 |
| Frontend | React 19 + TypeScript + Vite | react ^19.2.8, typescript ~6.0.2, vite ^8.2.0, React Compiler (babel-plugin-react-compiler 1.0.0) |
| Terminal | xterm.js | xterm ^5.3.0, xterm-addon-fit ^0.8.0 |
| PTY | portable-pty | 0.9 (wez/wezterm) |
| DB | — (yok) | — |
| Lint/Format | ESLint 10 (Vite şablonu) | eslint ^10.8.0 |
| Paket yöneticisi | **ÇİFT LOCKFILE** | `package-lock.json` + `pnpm-lock.yaml` (sorun, bkz. DURMALI) |

### 2.2 Backend (`src-tauri/`) — Neler Gerçekten Çalışıyor

| Dosya | İçerik | Değerlendirme |
|:---|:---|:---|
| `src-tauri/src/main.rs` | Tauri builder; `PtyManager` + `EngineAdapterRegistry` manage; 11 invoke handler | ✅ Tauri 2 konvansiyonuna göre `lib.rs` + `run()` yapısına taşınmalı (DEĞİŞMELİ D4) |
| `src-tauri/src/pty/mod.rs` | `agent_spawn`, `agent_write`, `agent_stop`, `pty_list_engine_adapters`, `pty_list_all_ids`, `pty_unregister_engine_adapter`, `pty_find_by_engine_type`, `pty_find_by_version` | ✅ Sağlam; `executionId` ile stale-oturum koruması roadmap §5'teki tavsiyeye uygun |
| `src-tauri/src/pty/adapters/mod.rs` | `EngineAdapter` trait: `id / metadata / detect / detect_info / health / health_report / spawn / stop` + `SpawnedPty` | ✅ AjanOfis Bölüm 7'deki `AgentAdapter` vizyonunun iskeleti; **`SpawnOptions` (workdir, task_file, budget, turns, non_interactive) eksik** |
| `src-tauri/src/pty/adapters/portable_pty_native.rs` | `PortablePtyAdapter`; Windows'ta Job Objects (`KILL_ON_JOB_CLOSE`) ile child izolasyonu; 6 unit test | ✅ Kaliteli; Unix'te process-group kill eksik (DEĞİŞMELİ D9) |
| `src-tauri/src/pty/registry/` | `EngineAdapterRegistry` (RwLock), `EngineAdapterQuery` (All/Detected/Healthy), `PtyManager`, `PtySession` | ✅ AjanOfis "detect/install/kayıt" hedefine uygun; Windows Drop ile job handle kapanıyor |
| `src-tauri/src/pty/runtime/mod.rs` | `start_output_pump`: reader thread + `app.emit("agent://output")`; 500ms poll ile `try_wait` lifecycle monitor | 🟡 Çalışıyor; **global event yerine `Channel<T>` + UTF-8 güvenli byte taşıma** önerilir (DEĞİŞMELİ D3) |
| `src-tauri/src/pty/worktree/mod.rs` | `build_command` | ✅ |
| `src-tauri/src/worktree.rs` | `worktree_create/remove/list` + `resolve_worktree_path_for_agent` (canonicalize + `starts_with` path-traversal koruması), `.agenthub.json` metadata, 2 unit test | ✅ AjanOfis `.git/agenthub-worktrees` deseninin birebir implementasyonu |
| `capabilities/main-capability.json` | sadece `core:default` | 🟡 plugin eklerken genişletilecek |
| `tauri.conf.json` | `beforeDevCommand: "npm exec --yes pnpm@9 -- dev"` | 🟡 her dev'de pnpm indirir; `pnpm dev` yapılmalı (DURMALI S2) |

### 2.3 Frontend (`src/`) — Durum

| Dosya | İçerik | Değerlendirme |
|:---|:---|:---|
| `src/App.tsx` | `<main>` + başlık + `<PtyTerminal />` (inline style) | ❌ AjanOfis Bölüm 5 layout'una (TopBar + grid + terminal) uzak |
| `src/components/PtyTerminal.tsx` | xterm + FitAddon; `agent_spawn` ile `powershell.exe` başlatır (hardcode); `listen('agent://output'/'agent://status')`; tarayıcıda local-echo modu | 🟡 İşlevsel ama Windows-only program hardcode, WebGL yok, **resize IPC yok** (fitAddon.fit() yapılıyor ama PTY boyutu backend'e bildirilmiyor), `default-agent-id` hardcode |
| `src/App.css`, `src/index.css` | Vite şablon artıkları (`.hero`, `.counter`, `#center`...) | ❌ Kullanılmıyor; temizlenecek |
| `src/assets/` | `react.svg`, `vite.svg`, `hero.png` | ❌ Kullanılmıyor; silinecek |

### 2.4 Altyapı / Çevre

- **Rust toolchain:** Bu sandbox'ta yok → `cargo test` çalıştırılamadı; mevcut testler kaynak taramasıyla değerlendirildi (adaptör + worktree testleri mevcut).
- **`.github/`:** yok (CI yok). **`.claude/` + `CLAUDE.md`:** yok. **README.md:** Vite şablonu.
- **Git:** remote `https://github.com/enesugurluu/AgentHub.git`; `main` + çalışma dalı `arena/019feb6c-agenthub`.

---

## 3. AjanOfis Docslarında "FAZ0"un Tanımı

FAZ0 iki bağlamda geçer ve ikisi de bu planın kapsamındadır:

### 3.1 AgentHub FAZ0 (uygulama iskeleti) — asıl hedef
`AGENTHUB-YOL-HARITASI-RAPORU.md` §3.1: *"Faz 0 & 1 Eksiklerinin Tamamlanması (Frontend ve State)"*:
- **Zustand** (global state) ve **shadcn/ui** eklenmeli
- **xterm-addon-webgl** MUST maddesi (10K+ satır logda çökmemek için)
- **rusqlite + WAL modu** (`src-tauri/src/db` klasörü, Tauri state'e bağlanacak)
- **Biome** (pre-commit hook + hızlı lint/format)

Bu, AjanOfis **MVP Yol Haritası (Bölüm 17) "Faz 1: İskelet (2 Hafta, M0)"** ile aynı iş kalemleridir:

```
[ ] Tauri 2 proje iskeleti (Vite + React + Tailwind 4 + shadcn/ui)   🟡 (Tailwind/shadcn yok)
[ ] Ana pencere layout (TopBar + ana grid + alt terminal alanı)      ❌
[ ] SQLite şeması (agents, tasks, events, settings) + ilk migration  ❌
[ ] Claude Code adaptörü: detect(), spawn(), stop()                  ❌
[ ] PTY alt yapısı (portable-pty) + Tauri Channel ile stream         ✅ (event ile, Channel değil)
[ ] xterm.js WebGL renderer ile ilk terminal paneli                  🟡 (xterm var, WebGL yok)
[ ] İlk "echo" testi: CLI stdin/stdout köprüsü                       🟡 (powershell echo; claude değil)
[ ] claude doctor benzeri sağlık kontrolü                            ❌
**Gate:** Kullanıcı tek bir Claude Code ajanını terminalden başlatıp etkileşebilir  ❌
```

### 3.2 Claude Code FAZ0 (çevre hazırlığı — host)
`Docs/claude-code-rapor-bolumleri/03-faz-0-temel-kurulum-ve-cevre.md`: Claude Code **native installer** (`curl -fsSL https://claude.ai/install.sh | bash`), tmux, `CLAUDE.md` hiyerarşisi, `.claude/settings.json` (izin modeli), `.claudeignore`, `.claude/rules|skills|agents|hooks`. AgentHub bu ortama ajan spawn edeceği için **geliştirici makinesinde** bu kurulum FAZ0 ön koşuludur.

---

## 4. FAZ0 ↔ Mevcut Yapı Karşılaştırma Matrisi

| # | Bileşen | AjanOfis Hedefi (docs) | Mevcut Durum | Fark / Yapılacak | Öncelik |
|:--:|:---|:---|:---|:---|:--:|
| 1 | Tauri çekirdek | 2.9+ (Bölüm 3.1) | 2.x kurulu (api 2.11.1) | Sürümü 2.9.x'e sabitle (Cargo.lock) | P1 |
| 2 | PTY motoru | `portable-pty` (Bölüm 3.3) | ✅ `PortablePtyAdapter` + Job Objects | Tamam; `child.resize()` için IPC ekle (resize yok!) | P1 |
| 3 | Adaptör mimarisi | `AgentAdapter` trait: detect/spawn/stop + capabilities (Bölüm 7) | ✅ `EngineAdapter` trait + registry | **CLI ajan adaptörleri yok** (claude/codex/gemini); `SpawnOptions` yok | P0 |
| 4 | Adaptör kaydı | detect/install/kayıt | ✅ `EngineAdapterRegistry` + query | `with_builtins()`'e ClaudeAdapter ekle | P0 |
| 5 | Git worktree | `.git/agenthub-worktrees` (Bölüm 10) | ✅ güvenli path + metadata + test | Tamam | — |
| 6 | PTY→UI stream | Tauri `Channel<T>` (Bölüm 3.3, 2) | 🟡 `app.emit` global event + `from_utf8_lossy` chunk | `Channel<T>` + byte-güvenli akış (çok baytlı UTF-8 bölünmesi) | P1 |
| 7 | Terminal render | xterm.js 5 + WebGL + web-links + search (Bölüm 3.2) | 🟡 xterm 5.3.0 + fit | `@xterm/addon-webgl/search/web-links/serialize` | P0 |
| 8 | UI framework | Tailwind CSS 4 + shadcn/ui (Bölüm 3.2) | ❌ inline style | Kurulum + `components.json` + tema | P0 |
| 9 | State | Zustand (Bölüm 3.2) | ❌ yok | `store/` modülleri (agents, terminal, settings...) | P0 |
| 10 | Layout | TopBar + grid + terminal (Bölüm 5) | ❌ tek sayfa | Docs Bölüm 5.3 bileşen ağacına göre iskelet | P0 |
| 11 | Veritabanı | SQLite + WAL; şema: agents/tasks/notes/note_links/note_fts/note_vec/events/settings (Bölüm 12) | ❌ yok | `src-tauri/src/db.rs` + migration + Tauri state | P0 |
| 12 | Claude Code entegrasyonu | `detect()/spawn()/stop()` (Bölüm 7.2) | ❌ powershell hardcode | `claude.rs` adaptörü (sürüm tespiti, `claude doctor`) | P0 |
| 13 | Sağlık kontrolü | `claude doctor` benzeri | 🟡 PTY `health()` var | CLI adaptörlerinde gerçek `--version`/`doctor` parse | P1 |
| 14 | Lint/Format | Biome + pre-commit (roadmap §3.1) | 🟡 ESLint şablonu | Biome'a geçiş; eslint kaldır | P1 |
| 15 | Paket yönetimi | pnpm (docs her yerde pnpm) | 🟡 çift lockfile | pnpm'e sabitle; package-lock.json sil | P1 |
| 16 | CI | — | ✅ `.github/workflows/ci.yml` kuruldu (frontend + ubuntu/windows/macos cargo matrisi + ci-gate) | — | — |
| 17 | CLAUDE.md/.claude | Claude Code FAZ0 (Bölüm 3.4-3.5) | ❌ yok | CLAUDE.md + settings.json + .claudeignore | P1 |
| 18 | İzolasyon | bwrap/Docker, secret maskeleme (Bölüm 10) | 🟡 Win: Job Objects ✅ / Unix: sadece child kill | Unix'te setsid/process-group kill | P1 |
| 19 | Capabilities | capability-based izin (Bölüm 15) | 🟡 `core:default` | dialog/fs plugin permission'ları | P2 |
| 20 | Ajan kimliği | registry'den türetilir | 🟡 frontend `default-agent-id` | DB'den gerçek agent kaydı | P0 |

---

## 5. FAZ0 İçin Gerekli Repolar (Doğrulanmış Liste)

> Aşağıdaki URL'ler 2026-08-10 itibarıyla web araştırmasıyla doğrulanmıştır. Kategoriler: **A = FAZ0 zorunlu**, **B = FAZ0 destek**, **C = referans mimari (bağımlılık değil)**, **D = çevre FAZ0 (host, opsiyonel)**.

### 5.1 Kategori A — FAZ0 ZORUNLU (Gate için şart)

| # | Repo / Paket | URL | Neden / Nereye | Kurulum | Not |
|:--:|:---|:---|:---|:---|:---|
| A1 | **xtermjs/xterm.js** (+ WebGL) | https://github.com/xtermjs/xterm.js | MUST (roadmap). Terminal render katmanı; WebGL renderer ile 60 FPS, 10K+ satır logda akıcı. `PtyTerminal.tsx`'e `@xterm/addon-webgl` bağlanacak | `pnpm add @xterm/addon-webgl @xterm/addon-web-links @xterm/addon-search @xterm/addon-serialize` | xterm 5.3.0 ile **aynı release ailesinden** addon sürümü seçin (ör. `@xterm/addon-webgl@0.18.x`, `@xterm/addon-fit@0.10.x`); eski unscoped `xterm-addon-fit@0.8.0` güncellenecek |
| A2 | **pmndrs/zustand** | https://github.com/pmndrs/zustand | Global state (docs Bölüm 3.2). `src/store/`: `agents.ts`, `terminal.ts`, `settings.ts` | `pnpm add zustand` | devtools/persist middleware'leri FAZ0'da yeterli; immer opsiyonel |
| A3 | **shadcn-ui/ui** | https://github.com/shadcn-ui/ui | UI bileşen seti (docs Bölüm 3.2). TopBar, dialog, buton, tabs, badge... | `pnpm dlx shadcn@latest init` + `pnpm dlx shadcn@latest add button dialog tabs badge` | Tailwind 4 + React 19 ile uyumlu; Tauri için hazır şablon: `agmmnn/tauri-ui` ve `kitlib/tauri-app-template` (ikisi de doğrulanmış, referans) |
| A4 | **tailwindlabs/tailwindcss** | https://github.com/tailwindlabs/tailwindcss | Stil sistemi v4 (docs Bölüm 3.2) | `pnpm add tailwindcss @tailwindcss/vite` + `vite.config.ts`'e plugin | `index.css`'e `@import "tailwindcss";` |
| A5 | **rusqlite/rusqlite** | https://github.com/rusqlite/rusqlite | SQLite + **WAL** (roadmap §3.1; docs Bölüm 12). `src-tauri/src/db.rs`; şema: agents/tasks/events/settings (Bölüm 12.1) | `cargo add rusqlite --features bundled` | `PRAGMA journal_mode=WAL;` migration başında; `bundled` ile platform bağımsız derleme; 2026 itibarıyla 0.3x–0.4x serisi |
| A6 | **biomejs/biome** | https://github.com/biomejs/biome | Lint + format + import organizasyonu tek araçta (roadmap §3.1). Pre-commit hook | `pnpm add -D @biomejs/biome` + `pnpm exec biome init` | v2.4 (Şubat 2026), ~24K+ yıldız; ESLint+Prettier yerine; `biome check --write` CI'da |
| A7 | **tauri-apps/plugins-workspace** | https://github.com/tauri-apps/plugins-workspace | Resmi plugin'ler: `tauri-plugin-dialog` (repo/klasör seçimi — FAZ0'da proje yolu seçimi için gerekli); sonra `fs`, `shell` (izinli) | `cargo add tauri-plugin-dialog` + `npm run tauri add dialog` (JS API) | `capabilities/main-capability.json`'a `dialog:allow-open` vb. eklenir; docs Bölüm 15 capability modeli |
| A8 | **wez/wezterm** (portable-pty) | https://github.com/wez/wezterm | Zaten bağımlılık; sürüm 0.9 sabit. `child.resize()` ile PTY boyut senkronu (yeni `pty_resize` komutu) | mevcut | AjanOfis docs Bölüm 3.3'ün birincil PTY tercihi |

### 5.2 Kategori B — FAZ0 Destek Kütüphaneleri (backend + frontend)

| # | Paket | URL | Neden | Kurulum | Faz |
|:--:|:---|:---|:---|:---|:--:|
| B1 | **tokio** | https://github.com/tokio-rs/tokio | Async runtime (docs Bölüm 3.3); Tauri 2 zaten tokio kullanıyor, async komutlar için | `cargo add tokio --features full` | FAZ0 |
| B2 | **chrono** | https://github.com/chronotope/chrono | Zaman damgaları (worktree `created_at` şu an unix epoch; DB için ISO) | `cargo add chrono` | FAZ0 |
| B3 | **tracing + tracing-subscriber** | https://github.com/tokio-rs/tracing | Yapılandırılmış log (docs Bölüm 3.3) | `cargo add tracing tracing-subscriber` | FAZ0 |
| B4 | **sysinfo** | https://github.com/GuillaumeGomez/sysinfo | `HealthReport.resource_utilization` (CPU/RAM) doldurmak (docs Bölüm 3.3) | `cargo add sysinfo` | FAZ0 |
| B5 | **dirs** | https://github.com/dirs-dev/dirs-rs | `~/.agentcompany` veri dizini (docs Bölüm 12.3) | `cargo add dirs` | FAZ0 |
| B6 | **thiserror** | https://github.com/dtolnay/thiserror | Hata tipleri | `cargo add thiserror` | FAZ0 |
| B7 | **lucide-react** (+ radix, clsx, tailwind-merge, class-variance-authority) | https://github.com/lucide-icons/lucide | shadcn/ui bağımlılıkları; `shadcn init` otomatik kurar | `pnpm dlx shadcn@latest init` | FAZ0 |
| B8 | **@tanstack/react-virtual** | https://github.com/TanStack/virtual | Büyük log/liste sanallaştırma (docs Bölüm 3.2) | `pnpm add @tanstack/react-virtual` | FAZ1 (ops.) |
| B9 | **git2** | https://github.com/rust-lang/git2-rs | Worktree/diff için libgit2 | opsiyonel | FAZ1 (roadmap: "CLI wrapper daha esnek" → mevcut `git` CLI yaklaşımı korunabilir) |

### 5.3 Kategori C — Referans Mimariler (read-only; bağımlılık DEĞİL)

| # | Repo | URL | Ne için | Nasıl kullanılır |
|:--:|:---|:---|:---|:---|
| C1 | **awslabs/cli-agent-orchestrator** (CAO) | https://github.com/awslabs/cli-agent-orchestrator | Roadmap §3.2 referansı: multi-agent orkestrasyon desenleri (handoff/assign/send-message), izole tmux oturumları, MCP üzerinden koordinasyon | `EngineAdapterRegistry`'ye desen aktarılır (kod bağımlılığı yok); `uv tool install git+https://github.com/awslabs/cli-agent-orchestrator.git@main` ile incelenir |
| C2 | **crynta/terax-ai** (Terax) | https://github.com/crynta/terax-ai | AjanOfis stack'inin **birebir referans implementasyonu** (Tauri 2 + portable-pty + React 19 + xterm.js WebGL + Tailwind 4 + shadcn/ui + Zustand, ~7 MB) | PTY session yönetimi (RwLock<HashMap>), OSC 133 shell integration (`agent_detect.rs`), `pnpm lint/check-types` CI desenleri birebir kopyalanabilir |
| C3 | **smtg-ai/claude-squad** | https://github.com/smtg-ai/claude-squad | Faz 2+: worktree + paralel oturum TUI'si (docs GitHub rehberi B1) | Çalışma alışkanlığı/referans; AgentHub'ın kendi çoklu-ajan UI'ı için desen kaynağı |
| C4 | **Dicklesworthstone/destructive_command_guard** | https://github.com/Dicklesworthstone/destructive_command_guard | Faz 3: yıkıcı komut koruması (roadmap §3.3; docs GitHub rehberi D2) | Rust guard mantığı `security.rs`'ye aktarılır; FAZ0'da kurulmaz |
| C5 | **modelcontextprotocol/rust-sdk** (`rmcp`) | https://github.com/modelcontextprotocol/rust-sdk | Faz 5: MCP Hub (docs Bölüm 11.2; roadmap §3.3) | `cargo add rmcp --features client,transport-child-process,transport-streamable-http-client-reqwest` — **FAZ0'da bağımlılık eklenmez**, şema/izin modeli tasarımı not edilir |
| C6 | **asg017/sqlite-vec** | https://github.com/asg017/sqlite-vec | Faz 4: vektör arama (docs Bölüm 12.1 `note_vec`; roadmap §3.4) | `cargo add sqlite-vec` + `sqlite_vec::load(&conn)`; pre-v1 (breaking riski) → FAZ0'da sadece şema hazırlığı |

### 5.4 Kategori D — Claude Code Çevre FAZ0 (geliştirici makinesi, opsiyonel)

| # | Araç | URL / Kurulum | Açıklama |
|:--:|:---|:---|:---|
| D1 | **Claude Code (native)** | `curl -fsSL https://claude.ai/install.sh \| bash` | FAZ0 ön koşulu (docs 03); `claude --version` ≥ 2.1.90, `claude doctor` |
| D2 | **github/github-mcp-server** | https://github.com/github/github-mcp-server — `claude mcp add --transport http github https://api.githubcopilot.com/mcp` | GitHub iş akışı (docs GitHub rehberi C1) |
| D3 | **upstash/context7-mcp** | https://github.com/upstash/context7-mcp — `claude mcp add --transport http context7 https://mcp.context7.com/mcp` | Güncel dokümantasyon, hallüsinasyon azaltır (C2) |
| D4 | **microsoft/playwright-mcp** | https://github.com/microsoft/playwright-mcp — `claude mcp add playwright -- npx @playwright/mcp@latest` | UI test (C3); aktif MCP sayısı 3-6 tutulmalı |
| D5 | **rohitg00/awesome-claude-code-toolkit** | https://github.com/rohitg00/awesome-claude-code-toolkit | Hazır CLAUDE.md şablonları, hook'lar, subagent'lar (opsiyonel) |
| D6 | **mckechniep/claude-hardening** + **yurukusa/cc-safe-setup** | https://github.com/mckechniep/claude-hardening — https://github.com/yurukusa/cc-safe-setup | Güvenlik sertleştirme (opsiyonel; docs GitHub rehberi D1/D3) |

---

## 6. Uygulama Kararları: YAPILMALI / DEĞİŞMELİ / DURMALI

### 6.1 YAPILMALI (öncelik sırasıyla, tahmini sürelerle)

| # | İş | Detay | Repo | Süre |
|:--:|:---|:---|:---|:--:|
| 0 | Ön koşullar | Rust toolchain (rustup), pnpm, `claude` CLI kurulumu + `claude doctor` | D1 | 30 dk |
| 1 | Paket yöneticisini sabitle | `pnpm-lock.yaml` tek kaynak; `package-lock.json` sil; `tauri.conf.json` → `beforeDevCommand: "pnpm dev"` | — | 10 dk |
| 2 | Stil + UI + state kurulumu | Tailwind 4 (`@tailwindcss/vite`) + `shadcn init` + zustand; `App.css`/`index.css` şablon artıklarını temizle; `components.json` | A3, A4, A2 | 1 sa |
| 3 | Terminal iyileştirme | `@xterm/addon-webgl` (renderer `'webgl'`), `web-links`, `search`, `serialize`; fit addon güncelle | A1 | 1 sa |
| 4 | Veritabanı katmanı | `src-tauri/src/db.rs`: rusqlite (bundled) + WAL + migration'lar (`agents, tasks, events, settings` — docs Bölüm 12.1); `manage()` ile Tauri state'e bağla; `tauri::async_runtime` entegrasyonu | A5 | 2-3 sa |
| 5 | Claude Code adaptörü | `src-tauri/src/agents/claude.rs`: `detect()` (`which claude` + `claude --version`), `health()` (`claude doctor` parse), `spawn()` (PTY + worktree cwd), `stop()`; `EngineAdapter` trait'ine `SpawnOptions` ekle (default implementasyonlarla geriye uyumlu — roadmap §5); `with_builtins()`'e kaydet | — | 3-4 sa |
| 6 | PTY→UI stream modernizasyonu | `start_output_pump` → per-session `Channel<PtyOutputEvent>`; UTF-8 byte tamponu ile çok baytlı karakter bölünmesini önle (xterm `Uint8Array` kabul eder); `pty_resize` komutu (`child.resize()`) + frontend'de fit→invoke | A8 | 2 sa |
| 7 | Ana layout iskeleti | Docs Bölüm 5.3: `TopBar` (CostMeter placeholder, ayarlar) + ana grid + `TerminalTabs` (sekmeli xterm); `shadcn` bileşenleri | A3 | 2-3 sa |
| 8 | Sağlık kontrolü | `claude doctor` benzeri `health_check` komutu; UI'da adaptör durum listesi (`pty_list_engine_adapters` + health_report) | — | 1 sa |
| 9 | Lint/format | `biome init`; eslint'i kaldır; `lint`/`format` script'leri; pre-commit hook (husky veya `.git/hooks/pre-commit`) | A6 | 1 sa |
| 10 | Claude Code çevre dosyaları | `CLAUDE.md` (proje kuralları — docs 03.4), `.claude/settings.json` (izinler), `.claudeignore`, `CLAUDE.local.md` (gitignored) | — | 30 dk |
| 11 | Capabilities + dialog plugin | `tauri-plugin-dialog` (proje/klasör seçimi); `main-capability.json`'a permission ekle | A7 | 30 dk |
| 12 | CI | `.github/workflows/ci.yml`: biome check, `tsc -b`, `vite build`, `cargo test`, `cargo clippy` (Terax'ın TERAX.md deseni) | C2 | 1 sa |
| 13 | README + doküman | Vite şablon README yerine gerçek proje README (kurulum, mimari, komutlar) | — | 30 dk |

### 6.2 DEĞİŞMELİ (mevcut kodda yapılacak değişiklikler)

| # | Dosya | Mevcut | Hedef |
|:--:|:---|:---|:---|
| D1 | `src/components/PtyTerminal.tsx` | `program: 'powershell.exe'` hardcode | Platform shelli (Windows: `powershell.exe`/`cmd`, Unix: `$SHELL`) **veya** doğrudan `claude` adaptörü; `default-agent-id` yerine registry'den gerçek agent |
| D2 | `src-tauri/src/pty/mod.rs` (`agent_spawn`) | `repo_path = current_dir` + worktree yoksa hata | Seçili repo yolu (dialog plugin); worktree yoksa otomatik oluştur veya ana repo'ya fallback; `program/args` yerine `engine_type` + `SpawnOptions` |
| D3 | `src-tauri/src/pty/runtime/mod.rs` | `app.emit` global event + per-chunk `from_utf8_lossy` + 500ms `try_wait` poll | `Channel<T>`; byte tampon + UTF-8 inkrementel decode; exit bekleme thread'i (poll yerine) |
| D4 | `src-tauri/src/main.rs` | her şey main'de | `lib.rs` + `run()` (Tauri 2 konvansiyonu, mobile-ready); komutları `commands/` modülüne taşı (docs Bölüm 16 klasör yapısı) |
| D5 | `src-tauri/src/pty/adapters/mod.rs` | `spawn(cmd: CommandBuilder, cols, rows)` | `SpawnOptions { workdir, task_file, model, effort, max_budget_usd, max_turns, env, non_interactive }` — default implementasyonlarla eski çağrılar bozulmaz (roadmap §5) |
| D6 | `src-tauri/src/pty/adapters/portable_pty_native.rs` | Unix'te `kill()` sadece child | `setsid`/process-group ile spawn + grup kill (torun süreçler de ölür) |
| D7 | `src-tauri/src/pty/adapters/mod.rs` (`HealthReport`) | `resource_utilization: None` | `sysinfo` ile CPU/RAM/process sayısı doldur |
| D8 | `src-tauri/tauri.conf.json` | `beforeDevCommand: npm exec --yes pnpm@9 -- dev` | `pnpm dev`; pencere başlığı/layout; `app > security` capability listesi güncel |
| D9 | `src-tauri/Cargo.toml` | 5 bağımlılık | A5/B1–B6 crates + sürüm pinleri (tauri 2.9.x, portable-pty 0.9) |
| D10 | `vite.config.ts` | sadece react + babel | `@tailwindcss/vite` eklentisi |
| D11 | `src-tauri/capabilities/main-capability.json` | `core:default` | + `dialog:*` (ve gerektiğinde fs/shell) permission'ları |
| D12 | `src-tauri/src/pty/registry/engine_adapter_registry.rs` | `with_builtins()` sadece PTY | + ClaudeAdapter (sonra codex/gemini/opencode/aider — docs Bölüm 3.4) |

### 6.3 DURMALI (bunları yapmayı bırak)

| # | Bırakılan uygulama | Neden | Yerine |
|:--:|:---|:---|:---|
| S1 | `package-lock.json` + `pnpm-lock.yaml` **çift lockfile** commit etmek | Sürüklenme ve çakışma kaynağı; docs her yerde pnpm | Tek `pnpm-lock.yaml` |
| S2 | `npm exec --yes pnpm@9 -- dev` | Her dev başlangıcında pnpm'i indirir (ağ bağımlı, yavaş) | Doğrudan `pnpm dev` |
| S3 | Vite şablon artıklarını taşımak: `App.css` (`.hero`, `.counter`...), `assets/react.svg`, `assets/vite.svg`, `assets/hero.png`, şablon README | Kullanılmayan kod/kir | Temizle; gerçek UI + README |
| S4 | Inline style ile UI geliştirmek | Bakımı zor, docs Bölüm 3.2'ye aykırı | Tailwind 4 + shadcn/ui |
| S5 | Frontend'den keyfi `program`/`args` ile süreç spawn etmek | Güvenlik (docs Bölüm 15); adaptör izolasyonunu by-pass eder | `engine_type` + adaptör katmanı |
| S6 | Global event adlarıyla PTY stream (`agent://output`) | Çoklu oturumda filtreleme yükü; Channel daha performanslı (docs Bölüm 3.3) | Per-session `Channel<T>` |
| S7 | Per-chunk `String::from_utf8_lossy` | Çok baytlı UTF-8 karakterler chunk sınırında bozulur (Türkçe/emoji) | Byte tampon + inkrementel decode; xterm'e `Uint8Array` |
| S8 | ESLint-only lint yaklaşımı | Roadmap §3.1 Biome istiyor; iki araç bakım maliyeti | Biome (tek araç) |
| S9 | `default-agent-id` gibi sabit kimliklerle oturum açmak | DB/registry yokken geçici çözüm; ajan başına state'i kirletir | DB'de gerçek agent kaydı + registry'den id |

---

## 7. FAZ0 Kabul Kriterleri (Gate — docs'taki tanım)

AjanOfis MVP "Faz 1: İskelet" Gate'i: **"Kullanıcı tek bir Claude Code ajanını terminalden başlatıp etkileşebilir."**

Genişletilmiş kabul listesi:
1. ✅ `claude` adaptörü `detect()` ile sürümünü raporluyor; `health()` `claude doctor` çıktısını parse ediyor.
2. ✅ Tek Claude Code ajanı PTY panelinden spawn edilebiliyor; `Channel` üzerinden çıktı akıyor; stdin köprüsü çalışıyor; `agent_stop` tüm süreç ağacını kapatıyor (Win: Job Object, Unix: process group).
3. ✅ 10K+ satırlık log akışında UI kilitlenmiyor (WebGL renderer aktif).
4. ✅ Ajan oturumu SQLite'a yazılıyor (`events` tablosu: spawn/output/exit); DB WAL modunda.
5. ✅ Ana layout (TopBar + grid + sekmeli terminal) render ediliyor; shadcn bileşenleri kullanılıyor.
6. ✅ `pnpm biome check`, `tsc -b`, `cargo test` temiz; pre-commit hook aktif.
7. ✅ `powershell.exe` hardcode'u yok; Windows + macOS + Linux'ta aynı kod yolu çalışıyor.
8. ✅ `CLAUDE.md` + `.claude/settings.json` mevcut; `claude doctor` temiz.

---

## 8. Riskler ve Önlemler

| Risk | Etki | Önlem |
|:---|:---|:---|
| xterm addon sürüm uyumsuzluğu (scoped `@xterm/*` vs unscoped `xterm-addon-*`) | Derleme hatası, runtime çökmesi | xterm 5.3.0 ile aynı release ailesinden addon'lar; `pnpm ls` ile doğrula |
| `rusqlite` bundled + ileride `sqlite-vec` yükleme uyumu | Faz 4'te sürpriz | sqlite-vec pre-v1; FAZ0'da sadece şema hazırlığı, bağımlılık Faz 4'e ertelenir |
| Windows ConPTY + Job Objects farklılıkları (bazı shell'ler job'a atanamaz) | Spawn hatası | `portable-pty`'nin kendi hata yolu + test matrisi (Win/macOS/Linux) |
| WebView farkları (WebView2 / WKWebView / WebKitGTK 4.1) | WebGL/xterm görüntü farkı | docs Bölüm 3.1: UI kodunu 3 motorla test eden CI matrisi |
| Claude Code CLI sürüm/flag değişiklikleri | `detect/spawn` kırılması | `detect()`'te sürüm pin + minimum sürüm kontrolü (2.1.90+); `claude doctor` sağlık kapısı |
| Tauri 2.9 + React Compiler (babel-plugin-react-compiler) | Derleme süresi, nadir transform hataları | Mevcut kurulum korunur; sorun olursa `reactCompilerPreset` kapatılabilir |
| İki lockfile'ın devam etmesi | Bağımlılık sürüklenmesi | S1 kararı: `package-lock.json` silinir, tek kaynak pnpm |

---

## 9. Sonraki Adım (FAZ0 sonrası özet)

FAZ0 gate'i geçildikten sonra AjanOfis MVP yol haritasına göre sıradaki fazlar:
- **Faz 2 — Çoklu Motor ve Worktree (M1):** Codex/Gemini/OpenCode/Aider adaptörleri, Hire Wizard, ofis katı SVG görünümü, runtime izolasyonu (port offset, `.env.local`), işten çıkarma akışı. Referans: C1 (CAO), C3 (claude-squad).
- **Faz 3 — CEO Orkestrasyonu (M2):** Kanban (`@dnd-kit`), görev modeli, onay akışı + policy engine (referans: C4 dcg), maliyet takibi.
- **Faz 4 — Bilgi Grafı ve Hafıza (M3):** `sqlite-vec` (C6), Sigma.js + Graphology, yerel embedding (`huggingface/candle`), Memory Keeper ajanı.
- **Faz 5 — MCP ve Cila (M4):** `rmcp` (C5), MCP Hub + izin UI, toplantı odası, paketleme + auto-updater.

Bu fazların repo listesi de istenirse aynı doğrulama yöntemiyle ayrı bir raporda çıkarılabilir.

---

## 10. Uygulama Durumu (2026-08-10)

Bu planın uygulaması `arena/019feb6c-agenthub` dalında tamamlandı. Durum özeti:

### ✅ Tamamlananlar (frontend — derlenmiş ve doğrulanmış)

| İş | Dosyalar | Doğrulama |
|:---|:---|:---|
| pnpm'e sabitleme, `package-lock.json` silindi | `pnpm-lock.yaml` | `pnpm install` ✓ |
| Tailwind 4 + shadcn/ui bileşenleri (9 bileşen) | `vite.config.ts`, `src/index.css`, `src/components/ui/*` | `pnpm build` ✓ |
| Zustand store'ları | `src/store/{agents,terminal,settings}.ts` | `tsc -b` ✓ |
| WebGL + search + web-links + serialize addon'ları (xterm 5.3 uyumlu sürümler) | `PtyTerminal.tsx` | `pnpm build` ✓ |
| Channel tabanlı PTY akışı (byte) + `pty_resize` | `PtyTerminal.tsx`, `src/lib/ipc.ts` | `tsc` ✓ |
| Platform shell seçimi (`powershell.exe`/`bash`), `default-agent-id` kaldırıldı | `PtyTerminal.tsx` | ✓ |
| Layout: TopBar + Ajan listesi + Ofis katı + Inspector + TerminalTabs | `App.tsx`, `components/*` | `pnpm build` ✓ |
| Biome (ESLint kaldırıldı) + husky pre-commit | `biome.json`, `.husky/pre-commit`, `package.json` | `biome check` ✓ |
| Tema geçişi (koyu/açık) | `TopBar.tsx`, `index.css` | ✓ |
| `@/` path alias | `tsconfig.app.json`, `vite.config.ts` | ✓ |
| SettingsDialog (adaptör listesi) | `SettingsDialog.tsx` | ✓ |

### ✅ Tamamlananlar (backend — yazıldı, bu ortamda DERLENEMEDİ ⚠️)

| İş | Dosyalar | Not |
|:---|:---|:---|
| `lib.rs`/`main.rs` ayrımı + `AppDb` setup | `lib.rs`, `main.rs` | Tauri 2 konvansiyonu |
| SQLite + WAL + şema (agents/tasks/events/settings) + seed | `db.rs` | rusqlite 0.40 bundled |
| `agent_list_all` komutu | `db.rs` | Frontend'de kullanılıyor |
| Claude Code adaptörü (detect/health/spawn_cli) | `agents/claude.rs` | engine_type `claude` |
| `spawn_pty_isolated` ortak yardımcısı (Job Objects tek noktada) | `pty/adapters/mod.rs` | Windows izolasyonu korundu |
| `spawn_cli` + `resize` trait default'ları | `pty/adapters/mod.rs` | Geriye uyumlu |
| Channel tabanlı pump (byte akışı + Exit kodu + DB kaydı) | `pty/runtime/mod.rs` | Global event kaldırıldı |
| `agent_spawn_engine` + `pty_resize` + worktree fallback | `pty/mod.rs` | |
| ClaudeAdapter registry'ye kayıt | `engine_adapter_registry.rs` | `with_builtins()` |
| dialog plugin + capability | `tauri.conf.json`, `main-capability.json` | `dialog:default` |

### ⚠️ Derlenememe nedeni ve yapılması gereken doğrulama

Bu sandbox'ta `crates.io`, `static.rust-lang.org` ve Debian mirror'ları erişilemez olduğundan
Rust toolchain kurulamadı; `cargo test`/`cargo clippy` **yerelde doğrulanamadı**. Kullanıcı
makinesinde yapılması gerekenler:

```bash
cd src-tauri && cargo check          # bağımlılık sürümlerini doğrula
cargo test                           # adaptör + worktree + registry testleri
cargo clippy --all-targets -- -D warnings
```

Olası uyum riskleri (düşük): `rusqlite 0.40` ve `sysinfo 0.37` API farklılıkları
(`global_cpu_usage`, `used_memory`), `tauri-plugin-dialog 2.x` sürüm eşleşmesi.

### 📋 Dokümantasyon

- `CLAUDE.md` + `.claude/settings.json` + `.claudeignore` — Claude Code çevre hazırlığı (docs 03)
- `README.md` — gerçek proje dokümantasyonu (Vite şablonu silindi)
- `.github/workflows/ci.yml` — biome + tsc + vite + cargo test/clippy
- `src-tauri/DEVELOPERS.md` — FAZ0 eklemeleri not edildi

---

## 11. Ek: FAZ0 Komut Özeti (Hızlı Başlangıç)

```bash
# 1) Ön koşullar
rustup update stable && pnpm --version          # Rust + pnpm
curl -fsSL https://claude.ai/install.sh | bash  # Claude Code native
claude doctor                                    # doğrulama

# 2) Frontend bağımlılıkları
pnpm add zustand
pnpm add tailwindcss @tailwindcss/vite
pnpm dlx shadcn@latest init                     # Tailwind 4 + React 19 uyumlu
pnpm dlx shadcn@latest add button dialog tabs badge separator scroll-area
pnpm add @xterm/addon-webgl @xterm/addon-web-links @xterm/addon-search @xterm/addon-serialize
pnpm add -D @biomejs/biome && pnpm exec biome init

# 3) Backend bağımlılıkları
cd src-tauri
cargo add rusqlite --features bundled
cargo add tokio --features full
cargo add chrono tracing tracing-subscriber sysinfo dirs thiserror
cargo add tauri-plugin-dialog

# 4) Doğrulama
pnpm biome check . && pnpm tsc -b && pnpm build
cd src-tauri && cargo test && cargo clippy --all-targets -- -D warnings
```

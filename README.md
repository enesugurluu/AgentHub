# AgentHub

[![CI](https://github.com/enesugurluu/AgentHub/actions/workflows/ci.yml/badge.svg)](https://github.com/enesugurluu/AgentHub/actions/workflows/ci.yml)

AjanOfis (AjanŞirket) vizyonunun Tauri 2 masaüstü uygulaması — çoklu AI CLI ajanını
(Claude Code, ileride Codex/Gemini/OpenCode...) izole git worktree'lerinde yöneten
bir "ajan şirketi" yönetim paneli.

> **Durum: FAZ0 ✅** — iskelet + PTY motoru + Claude Code adaptörü + SQLite + terminal UI.
> FAZ0 durum analizi: [`FAZ0-DURUM-ANALIZI-VE-UYGULAMA-PLANI.md`](./FAZ0-DURUM-ANALIZI-VE-UYGULAMA-PLANI.md)
> Sıradaki: **FAZ1 (M1)** — çoklu motor, worktree otomasyonu, işe alım: [`FAZ1-PLANI.md`](./FAZ1-PLANI.md)

## Teknoloji Yığını

| Katman | Teknoloji |
|:---|:---|
| Desktop | Tauri 2 (Rust) |
| Frontend | React 19 + TypeScript + Vite 8, Tailwind CSS 4 + shadcn/ui, Zustand |
| Terminal | xterm.js 5 + WebGL renderer (+ fit, search, web-links, serialize) |
| PTY | `portable-pty` (ConPTY / POSIX) + Windows Job Objects izolasyonu |
| Veri | SQLite (`rusqlite`, bundled, WAL) — `agents`, `tasks`, `events`, `settings` |
| Lint/Format | Biome (+ husky pre-commit) |

## Gereksinimler

- **Node.js 22+** ve **pnpm 9** (`npm i -g pnpm@9` veya corepack)
- **Rust stable** (rustup) — Linux'ta ayrıca `libwebkit2gtk-4.1-dev`, `libgtk-3-dev` vb. (Tauri ön koşulları)
- **Claude Code CLI** (Claude ajanı için): `curl -fsSL https://claude.ai/install.sh | bash` → `claude doctor` ile doğrula

## Kurulum ve Çalıştırma

```bash
pnpm install
pnpm tauri:dev        # masaüstü uygulaması (Tauri + Vite)
# veya sadece frontend önizlemesi:
pnpm dev              # http://localhost:5173 (tarayıcıda local-echo modu)
```

## Komutlar

| Komut | Açıklama |
|:---|:---|
| `pnpm dev` | Vite dev server |
| `pnpm tauri:dev` | Tauri + Vite |
| `pnpm build` | tsc + vite production build |
| `pnpm check` / `pnpm lint` | Biome (fix / salt) |
| `pnpm typecheck` | tsc -b |
| `cd src-tauri && cargo test` | Rust unit testleri |

## Mimari (özet)

```
src-tauri/            Rust backend
├── src/
│   ├── lib.rs        Tauri builder, DB setup, komut kaydı
│   ├── main.rs       giriş noktası
│   ├── db.rs         SQLite (WAL) — agents/tasks/events/settings
│   ├── agents/       CLI ajan adaptörleri (claude.rs ilk dalga)
│   ├── pty/          adaptör trait + registry, PTY runtime (Channel), worktree
│   └── worktree.rs   git worktree yöneticisi (güvenli path)
src/                  React frontend
├── components/       TopBar, OfficeFloor, AgentSidebar, Inspector, TerminalTabs, ui/
├── store/            Zustand (agents, terminal, settings)
└── lib/              ipc.ts (Tauri invoke köprüsü), utils
```

PTY çıktısı, per-session `Channel<PtyEvent>` ile ham bayt olarak akar
(`kind.type = "output" | "exit"`); ajanlar `.git/agenthub-worktrees` altındaki
izole worktree'lerde çalışır.

## Yol Haritası

- ✅ **FAZ0** — iskelet, PTY + adaptör registry, worktree, Claude adaptörü, SQLite, terminal UI
- ⏳ **FAZ1 (M1)** — Codex/Gemini/OpenCode/Aider adaptörleri, işe alım sihirbazı, worktree otomasyonu, ofis katı SVG — [plan](./FAZ1-PLANI.md)
- ⏳ **Faz 3 (M2)** — Kanban, CEO orkestrasyonu, onay akışı + policy engine, maliyet takibi
- ⏳ **Faz 4 (M3)** — Bilgi grafı (SQLite + FTS5 + sqlite-vec), Sigma.js, yerel embedding
- ⏳ **Faz 5 (M4)** — MCP Hub (`rmcp`), toplantı odası, paketleme + auto-updater

Detay: [`AGENTHUB-YOL-HARITASI-RAPORU.md`](./AGENTHUB-YOL-HARITASI-RAPORU.md) ve `Docs/`

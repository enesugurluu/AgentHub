# Proje: AgentHub

AjanOfis (AjanŞirket) vizyonunun Tauri 2 masaüstü uygulaması. Çoklu AI CLI ajanını
(Claude Code, sonra Codex/Gemini/OpenCode...) izole git worktree'lerinde yöneten
"ajan şirketi" yönetim paneli.

## Kim Bu Proje?

- **Tech Stack:** Tauri 2 (Rust) + React 19 + TypeScript + Vite 8, Tailwind CSS 4 + shadcn/ui, Zustand, xterm.js 5 (WebGL), portable-pty, SQLite (rusqlite, WAL)
- **Mimari:** `src-tauri/` (Rust: `db`, `agents`, `pty`, `worktree` modülleri) + `src/` (React: `components`, `store`, `lib`)
- **Dil:** Türkçe yorumlar, İngilizce kod ve commit mesajları
- **Dokümanlar:** `Docs/` — AjanOfis mimari raporları; `FAZ0-DURUM-ANALIZI-VE-UYGULAMA-PLANI.md` — FAZ0 planı

## Çalışma Komutları

- `pnpm dev` — Vite dev server (5173)
- `pnpm tauri:dev` — Tauri + Vite birlikte (masaüstü)
- `pnpm build` — tsc + vite production build
- `pnpm check` — Biome format + lint (auto-fix)
- `pnpm lint` — Biome lint (salt)
- `pnpm typecheck` — tsc -b
- `cd src-tauri && cargo test` — Rust unit testleri (adaptör + worktree)
- `pnpm tauri:build` — Paketleme

## Kod Standartları

- TypeScript strict mode açık; `@/` path alias'ı kullanılır
- Yeni UI bileşeni: önce `src/components/ui/` altında shadcn deseninde, sonra kullan
- State: Zustand store'ları (`src/store/`); bileşen içi state sadece lokal UI için
- Stil: Tailwind 4 sınıfları; inline style yok
- Lint/Format: Biome (ESLint/Prettier yok); pre-commit hook aktif (`pnpm exec biome check --staged`)
- Rust: `tracing` ile log; yeni komutlar `#[tauri::command]` + `lib.rs` invoke_handler listesine eklenmeli
- Tauri komut argümanları camelCase (JS) ↔ snake_case (Rust) otomatik eşlenir

## Mimari Notlar

- **PTY akışı:** Per-session `Channel<PtyEvent>`; çıktı ham bayt (`Vec<u8>`); `kind.type = "output" | "exit"`
- **Adaptörler:** `EngineAdapter` trait'i; `with_builtins()` içinde `portable-pty-native` + `claude-code`
- **Veri:** `agenthub.db` (WAL) — `agents`, `tasks`, `events`, `settings`; schema `src-tauri/src/db.rs`
- **İzolasyon:** Windows Job Objects (KILL_ON_JOB_CLOSE) + Unix process kill; worktree'ler `.git/agenthub-worktrees` altında (path-traversal korumalı)

## Güvenlik Kuralları

- `.env` dosyalarını ASLA okuma/yazma
- `node_modules`, `dist`, `src-tauri/target`, `src-tauri/gen` dizinlerine dokunma
- Git'e secret/API key commit'leme
- `git push --force` kullanma
- PTY spawn işlemlerinde worktree yolunu frontend'den alma; backend `resolve_agent_workdir` kullan
- Yeni npm paketi eklerken önce kullanıcıya bildir; `package-lock.json` KULLANMA (pnpm-lock.yaml tek kaynak)

## Çalışma Prensipleri

- Önce çalışır kod, sonra güzel kod
- Rust tarafı değişikliklerinde `cargo test` + `cargo clippy --all-targets -- -D warnings` çalıştır
- Frontend değişikliklerinde `pnpm check && pnpm typecheck && pnpm build` çalıştır
- Emin olmadığın şeyleri sor, varsayım yapma
- 3 başarısız düzeltmede dur ve özetle

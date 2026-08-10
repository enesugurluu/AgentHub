# src-tauri developer notes

## FAZ0 eklemeleri (2026-08)

- **`db.rs`** — SQLite + WAL (`rusqlite`, bundled): `agents`, `tasks`, `events`, `settings` şeması (docs Bölüm 12.1 alt kümesi). Uygulama veri dizininde `agenthub.db`; `AppDb` Tauri state olarak `lib.rs` setup'ında kayıtlı. İlk çalıştırmada demo ajanlar seed edilir.
- **`agents/`** — CLI ajan adaptörleri. `claude.rs`: `EngineAdapter` implementasyonu (`id = "claude-code"`, `engine_type = "claude"`). `detect()` → `claude --version`; `health()` → sürüm kontrolü; `spawn_cli()` → worktree dizininde interaktif REPL.
- **`pty/adapters/mod.rs`** — `spawn_pty_isolated()` ortak yardımcısı (tüm adaptörler kullanır; Windows Job Objects izolasyonu tek noktada). `SpawnOptions` (FAZ1 WP-02: model/effort/budget/turns/non_interactive/task_file) ve `spawn_cli()` / `resize()` trait default'ları eklendi.
- **`pty/runtime/mod.rs`** — Global event yerine **per-session `Channel<PtyEvent>`**. Çıktı ham bayt (`Vec<u8>`), çıkış `Exit { code }`. Olaylar `events` tablosuna yazılır.
- **Yeni komutlar:** `agent_spawn_engine` (engine_type bazlı), `pty_resize` (xterm fit → PTY boyutu), `pty_adapter_metadata` (id → metadata), `agent_list_all` (DB).
- `main.rs` → `lib.rs` + `run()` (Tauri 2 konvansiyonu).

## FAZ1 WP-04/05/07…13 eklemeleri (2026-08) — kısa özet

- **WP-04 `pty/runtime/parser.rs`** — `OutputParser` trait + `OutputSignal`
  (Progress/ApprovalRequested/TaskCompleted/TaskFailed) + satır tamponlu 3 parser
  (claude stream-json, opencode jsonl, regex) + `select_parser(engine_type, non_interactive)`.
  `PtyEventKind::Signal { signal }`; `pump_loop` Channel'dan bağımsız (test edilebilir).
- **WP-05 `worktree.rs`** — `ensure_agent_worktree` (idempotent, `agent/<slug>-<suffix>`),
  `prepare_worktree_env` (`.env.local` port offset, mevcut anahtarlar korunur),
  `link_node_modules` (Unix symlink / Win junction, best-effort),
  `worktree_remove(path, options)` (delete|keep|commit_and_keep), `worktree_for_agent`.
- **WP-07** — `pty_adapter_detect_info` komutu; `src/hooks/useEngineRegistry.ts`;
  `src/lib/presets.ts` (docs 6.3); `HireWizard.tsx` (3 adım).
- **WP-10 `tasks.rs`** — `write_agent_task` (AGENT_TASK.md şablonu) + `decide_completion`
  (TASK_BLOCKED.md > TASK_COMPLETE.md > parser sinyali > exit kodu). `task_assign` komutu:
  ensure worktree → AGENT_TASK.md → non-interactive spawn; oturum `task_id`/`worktree_path`
  etiketli; exit'te `finalize_task` + `task_completed/failed` olayı.
- **WP-11 `pty/runtime/transcript.rs`** — JSONL kayıt (`~/.agentcompany/logs/<slug>/`),
  `transcript_append_session_buffer`. Not: chrono/dirs yok — epoch + HOME/USERPROFILE (CI `--locked`).
- **WP-12** — Settings "Motorlar" kartları + kurulum: `agentId = "install-<engine>"` oturumu;
  `PtyTerminal` `install-*`'da otomatik `agentInstallEngine` (ref-guard, tek sefer).
- **WP-13** — `EngineAdapter::supports(feature)` (capability listesi); desteklenmeyen
  budget/turns/effort için `tracing::warn!`; pump `Progress.cost` birikimi → exit payload
  + JSONL `totalCostUsd`; frontend `totalCostUsd` + TopBar CostMeter.

## FAZ1 WP-06 eklemeleri (2026-08)

- **`db::repo_select`** — repo yolu seçimi: canonicalize + `.git` doğrulaması + worktree kökü
  reddi (`.git/agenthub-worktrees` içi) + `settings.repo_path`'e yazma.
- **`resolve_repo_root(&app)`** (pty/mod.rs) — öncelik: `settings.repo_path` → `AGENTHUB_REPO_PATH`
  env (dev köprüsü) → cwd (son çare). WP-05 worktree otomasyonu bu kaynağı kullanır.
- **Frontend:** `@tauri-apps/plugin-dialog` eklendi (Rust plugin FAZ0'da vardı; capability
  `dialog:default` yeterli). `src/store/projects.ts` (repoPath + onboardingSkipped),
  TopBar proje çipi → dialog → `repo_select`, `src/components/OnboardingDialog.tsx`.

## FAZ1 WP-03 eklemeleri (2026-08)

- **`agents/{codex,gemini,opencode,aider}.rs`** — yeni CLI adaptörleri (docs Bölüm 7.2). Hepsi
  aynı kalıpta: `detect()` → `--version`; `health()` → sürüm kapısı; `spawn_cli()` → kendi
  komutunu kurar; `install_command()` → `agent_install_engine` (WP-12 kurulum akışı).
- **`agents/mod.rs`** — ortak yardımcılar: `detect_binary_version`, `command_from`
  (SpawnOptions → CommandBuilder), `read_task_content` (AGENT_TASK.md), `test_util`
  (mock-CLI matrisi: `with_fake_binary` Unix'te PATH'e sahte binary koyar; Windows no-op).
- **`pty/adapters/mod.rs`** — `DetectResult.install_hint` + trait'e `install_command()`
  (default None) ve `spawn()` **default** implementasyonu (CLI adaptörleri yalnızca
  `spawn_cli` yazar).
- **`pty/mod.rs::agent_install_engine`** — kurulum komutunu backend'de çözer, `pty`
  adaptörüyle ayrı oturumda çalıştırır (`agent_id = "install-<engine>"`, olay tipi `install`).
- **Capability kuralı:** budget/turn/effort desteklemeyen motorlar (codex/gemini/opencode/aider)
  bu capability'leri ilan etmez; `SpawnOptions`'taki alanlar yok sayılır (WP-13'te `supports()` + warn).
- **Flag matrisi uyarısı:** CLI flag eşleşmeleri uygulama sırasında `--help` ile doğrulanmalı;
  golden argv testleri yalnızca bizim eşlememizi sabitler (sürüm bağımsız mock).

## Çalışma dizini / olay kaydı notları

- Spawn cwd'si `AGENTHUB_REPO_PATH` env değişkeniyle override edilebilir; yoksa uygulama sürecinin `current_dir`'i kullanılır (ör. `tauri dev` altında `src-tauri/`). Dialog ile repo seçimi FAZ1'de.
- `agent_spawn` yalnızca `engine_type = "pty"` adaptörlerinden seçim yapar (`select_default_for_engine_type`); CLI adaptörleri shell spawn'ını devralmaz.
- `events.agent_id` FK korumalı: frontend sayısal ajan id'leri ("1", "2") gönderir; sayısal olmayan id'lerde olay `agent_id = NULL` ile yazılır (olay asla sessizce düşmez).

## PTY engine adapters

The PTY layer supports multiple backends via the `EngineAdapter` trait:

- **Metadata**: `metadata()` provides information about the engine type, version, and capabilities.
- **Detect**: `detect()` checks whether an adapter is applicable on the current host.
- **Detect Info**: `detect_info()` provides the result of detection along with version and capability details.
- **Health**: `health()` is a cheap sanity check for runtime readiness.
- **Health Report**: `health_report()` returns a comprehensive report including uptime, resource utilization, and overall operational status.
- **Spawn**: `spawn()` starts a PTY process and returns `{ reader, writer, child }`.
- **Spawn CLI**: `spawn_cli(SpawnOptions, ...)` lets CLI adapters (claude/codex/...) build their own command line.
- **Resize**: `resize()` forwards xterm size changes to the PTY.
- **Stop**: `stop()` shuts down a previously spawned child process.

Core types:

- `EngineAdapter` and Metadata Structures: `src-tauri/src/pty/adapters/mod.rs`
- `EngineAdapterRegistry`: `src-tauri/src/pty/registry/engine_adapter_registry.rs`
- Built-in adapter `PortablePtyAdapter`: `src-tauri/src/pty/adapters/portable_pty_native.rs`
- CLI adapter `ClaudeAdapter`: `src-tauri/src/agents/claude.rs`

### Adding a new adapter

1. Implement `EngineAdapter` for your type. Note that your `id` must not be empty and your `metadata().engine_type` must also not be empty.
2. Register it at startup (`EngineAdapterRegistry::with_builtins` in `engine_adapter_registry.rs`):

```rust
use std::sync::Arc;
use pty::registry::EngineAdapterRegistry;

let registry = EngineAdapterRegistry::with_builtins();
registry.register(Arc::new(MyAdapter::new()))?; // Registration validates ID and metadata
```

3. New sessions store `adapter_id` in `PtySession`, and `agent_stop` resolves the adapter from the registry to stop the child process.
4. You can query adapters at runtime using the registry's `find_by_engine_type`, `find_by_version`, and custom `query_metadata` methods, which are also exposed via Tauri commands.

### Testing

The adapter registry is designed to be mockable:

- `cargo test` (src-tauri dizininde) — adaptör + worktree unit testleri.
- Frontend kontrolleri: `pnpm check` (biome), `pnpm typecheck` (tsc -b), `pnpm build`.

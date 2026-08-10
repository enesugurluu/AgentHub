# src-tauri developer notes

## FAZ0 eklemeleri (2026-08)

- **`db.rs`** — SQLite + WAL (`rusqlite`, bundled): `agents`, `tasks`, `events`, `settings` şeması (docs Bölüm 12.1 alt kümesi). Uygulama veri dizininde `agenthub.db`; `AppDb` Tauri state olarak `lib.rs` setup'ında kayıtlı. İlk çalıştırmada demo ajanlar seed edilir.
- **`agents/`** — CLI ajan adaptörleri. `claude.rs`: `EngineAdapter` implementasyonu (`id = "claude-code"`, `engine_type = "claude"`). `detect()` → `claude --version`; `health()` → sürüm kontrolü; `spawn_cli()` → worktree dizininde interaktif REPL.
- **`pty/adapters/mod.rs`** — `spawn_pty_isolated()` ortak yardımcısı (tüm adaptörler kullanır; Windows Job Objects izolasyonu tek noktada). `SpawnOptions` (FAZ1 WP-02: model/effort/budget/turns/non_interactive/task_file) ve `spawn_cli()` / `resize()` trait default'ları eklendi.
- **`pty/runtime/mod.rs`** — Global event yerine **per-session `Channel<PtyEvent>`**. Çıktı ham bayt (`Vec<u8>`), çıkış `Exit { code }`. Olaylar `events` tablosuna yazılır.
- **Yeni komutlar:** `agent_spawn_engine` (engine_type bazlı), `pty_resize` (xterm fit → PTY boyutu), `pty_adapter_metadata` (id → metadata), `agent_list_all` (DB).
- `main.rs` → `lib.rs` + `run()` (Tauri 2 konvansiyonu).

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

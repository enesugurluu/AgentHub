# src-tauri developer notes

## FAZ0 eklemeleri (2026-08)

- **`db.rs`** — SQLite + WAL (`rusqlite`, bundled): `agents`, `tasks`, `events`, `settings` şeması (docs Bölüm 12.1 alt kümesi). Uygulama veri dizininde `agenthub.db`; `AppDb` Tauri state olarak `lib.rs` setup'ında kayıtlı. İlk çalıştırmada demo ajanlar seed edilir.
- **`agents/`** — CLI ajan adaptörleri. `claude.rs`: `EngineAdapter` implementasyonu (`id = "claude-code"`, `engine_type = "claude"`). `detect()` → `claude --version`; `health()` → sürüm kontrolü; `spawn_cli()` → worktree dizininde interaktif REPL.
- **`pty/adapters/mod.rs`** — `spawn_pty_isolated()` ortak yardımcısı (tüm adaptörler kullanır; Windows Job Objects izolasyonu tek noktada). `CliSpawnOptions` ve `spawn_cli()` / `resize()` trait default'ları eklendi.
- **`pty/runtime/mod.rs`** — Global event yerine **per-session `Channel<PtyEvent>`**. Çıktı ham bayt (`Vec<u8>`), çıkış `Exit { code }`. Olaylar `events` tablosuna yazılır.
- **Yeni komutlar:** `agent_spawn_engine` (engine_type bazlı), `pty_resize` (xterm fit → PTY boyutu), `agent_list_all` (DB).
- `main.rs` → `lib.rs` + `run()` (Tauri 2 konvansiyonu).

## PTY engine adapters

The PTY layer supports multiple backends via the `EngineAdapter` trait:

- **Metadata**: `metadata()` provides information about the engine type, version, and capabilities.
- **Detect**: `detect()` checks whether an adapter is applicable on the current host.
- **Detect Info**: `detect_info()` provides the result of detection along with version and capability details.
- **Health**: `health()` is a cheap sanity check for runtime readiness.
- **Health Report**: `health_report()` returns a comprehensive report including uptime, resource utilization, and overall operational status.
- **Spawn**: `spawn()` starts a PTY process and returns `{ reader, writer, child }`.
- **Spawn CLI**: `spawn_cli(CliSpawnOptions, ...)` lets CLI adapters (claude/codex/...) build their own command line.
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

# src-tauri developer notes

## PTY engine adapters

The PTY layer supports multiple backends via the `EngineAdapter` trait:

- **Metadata**: `metadata()` provides information about the engine type, version, and capabilities.
- **Detect**: `detect()` checks whether an adapter is applicable on the current host.
- **Detect Info**: `detect_info()` provides the result of detection along with version and capability details.
- **Health**: `health()` is a cheap sanity check for runtime readiness.
- **Health Report**: `health_report()` returns a comprehensive report including uptime, resource utilization, and overall operational status.
- **Spawn**: `spawn()` starts a PTY process and returns `{ reader, writer, child }`.
- **Stop**: `stop()` shuts down a previously spawned child process.

Core types:

- `EngineAdapter` and Metadata Structures: [adapters/mod.rs](file:///D:/Projeler/AgentHub/AgentHub/agentHub/src-tauri/src/pty/adapters/mod.rs)
- `EngineAdapterRegistry`: [engine_adapter_registry.rs](file:///D:/Projeler/AgentHub/AgentHub/agentHub/src-tauri/src/pty/registry/engine_adapter_registry.rs)
- Built-in adapter `PortablePtyAdapter`: [portable_pty_native.rs](file:///D:/Projeler/AgentHub/AgentHub/agentHub/src-tauri/src/pty/adapters/portable_pty_native.rs)

### Adding a new adapter

1. Implement `EngineAdapter` for your type. Note that your `id` must not be empty and your `metadata().engine_type` must also not be empty.
2. Register it at startup:

```rust
use std::sync::Arc;
use pty::registry::EngineAdapterRegistry;

let registry = EngineAdapterRegistry::with_builtins();
registry.register(Arc::new(MyAdapter::new()))?; // Registration validates ID and metadata
```

3. New sessions store `adapter_id` in `PtySession`, and `pty_stop` resolves the adapter from the registry to stop the child process.
4. You can query adapters at runtime using the registry's `find_by_engine_type`, `find_by_version`, and custom `query_metadata` methods, which are also exposed via Tauri commands.

### Testing

The adapter registry is designed to be mockable:

- unit tests create a `MockAdapter` implementing `EngineAdapter`
- registry selection/query logic and metadata validation is tested without spawning a real PTY
- built-in adapters (like `PortablePtyAdapter`) include tests for all adapter methods

Run tests via:

```bash
cargo test --bin agenthub
```

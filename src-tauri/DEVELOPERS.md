# src-tauri developer notes

## PTY engine adapters

The PTY layer supports multiple backends via the `EngineAdapter` trait:

- **Detect**: `detect()` checks whether an adapter is applicable on the current host.
- **Health**: `health()` is a cheap sanity check for runtime readiness.
- **Spawn**: `spawn()` starts a PTY process and returns `{ reader, writer, child }`.
- **Stop**: `stop()` shuts down a previously spawned child process.

Core types:

- `EngineAdapter`: [adapters/mod.rs](file:///D:/Projeler/AgentHub/AgentHub/agentHub/src-tauri/src/pty/adapters/mod.rs)
- `EngineAdapterRegistry`: [engine_adapter_registry.rs](file:///D:/Projeler/AgentHub/AgentHub/agentHub/src-tauri/src/pty/registry/engine_adapter_registry.rs)
- built-in adapter `PortablePtyAdapter`: [portable_pty_native.rs](file:///D:/Projeler/AgentHub/AgentHub/agentHub/src-tauri/src/pty/adapters/portable_pty_native.rs)

### Adding a new adapter

1. Implement `EngineAdapter` for your type.
2. Register it at startup:

```rust
use std::sync::Arc;
use pty::registry::EngineAdapterRegistry;

let registry = EngineAdapterRegistry::with_builtins();
registry.register(Arc::new(MyAdapter::new()))?;
```

3. New sessions store `adapter_id` in `PtySession`, and `pty_stop` resolves the adapter from the registry to stop the child process.

### Testing

The adapter registry is designed to be mockable:

- unit tests create a `MockAdapter` implementing `EngineAdapter`
- registry selection/query logic is tested without spawning a real PTY

Run:

```bash
cargo test
```


//! AgentHub — Tauri 2 Rust backend.
//!
//! Mimari (AjanOfis docs Bölüm 16 ile uyumlu):
//! - `db/`     → SQLite + WAL (agents, tasks, events, settings)
//! - `agents/` → CLI ajan adaptörleri (Claude Code ilk dalga)
//! - `pty/`    → PTY motoru (portable-pty), adaptör registry, runtime pump
//! - `worktree/` → git worktree yöneticisi (güvenli path)

pub mod agents;
pub mod db;
pub mod pty;
pub mod tasks;
pub mod worktree;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tracing_subscriber::fmt()
    .with_env_filter(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info".into()),
    )
    .init();

  tauri::Builder::default()
    .setup(|app| {
      // Uygulama veri dizininde SQLite DB'yi açar (WAL modunda).
      let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("failed to resolve app data dir: {e}"))?;
      std::fs::create_dir_all(&data_dir)?;
      let db = db::AppDb::open(data_dir)?;
      app.manage(db);
      Ok(())
    })
    .manage(pty::registry::PtyManager::default())
    .manage(pty::registry::EngineAdapterRegistry::with_builtins())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![
      pty::agent_spawn,
      pty::agent_spawn_engine,
      pty::agent_write,
      pty::agent_stop,
      pty::agent_install_engine,
      pty::transcript_append_session_buffer,
      pty::pty_resize,
      pty::pty_list_engine_adapters,
      pty::pty_list_all_ids,
      pty::pty_adapter_metadata,
      pty::pty_adapter_detect_info,
      pty::pty_unregister_engine_adapter,
      pty::pty_find_by_engine_type,
      pty::pty_find_by_version,
      db::agent_hire,
      db::agent_fire,
      db::agent_delete,
      db::agent_update,
      db::agent_get,
      db::agent_list_all,
      db::settings_get,
      db::settings_set,
      db::repo_select,
      db::task_create,
      db::task_get,
      db::task_list,
      db::task_finalize,
      pty::task_assign,
      worktree::worktree_create,
      worktree::worktree_remove,
      worktree::worktree_list,
      worktree::worktree_for_agent
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

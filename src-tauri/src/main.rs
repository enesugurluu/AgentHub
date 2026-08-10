#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pty;
mod worktree;

use pty::{
  pty_find_by_engine_type, pty_find_by_version, pty_list_all_ids,
  pty_list_engine_adapters, agent_spawn, agent_stop, pty_unregister_engine_adapter, agent_write,
};
use pty::registry::{EngineAdapterRegistry, PtyManager};
use worktree::{worktree_create, worktree_remove, worktree_list};

fn main() {
  tauri::Builder::default()
    .manage(PtyManager::default())
    .manage(EngineAdapterRegistry::with_builtins())
    .invoke_handler(tauri::generate_handler![
      agent_spawn,
      agent_write,
      agent_stop,
      pty_list_engine_adapters,
      pty_list_all_ids,
      pty_unregister_engine_adapter,
      pty_find_by_engine_type,
      pty_find_by_version,
      worktree_create,
      worktree_remove,
      worktree_list
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

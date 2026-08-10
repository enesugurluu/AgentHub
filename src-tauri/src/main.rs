#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pty;

use pty::{
  pty_find_by_engine_type, pty_find_by_version, pty_list_all_ids,
  pty_list_engine_adapters, pty_spawn, pty_stop, pty_unregister_engine_adapter, pty_write,
};
use pty::registry::{EngineAdapterRegistry, PtyManager};

fn main() {
  tauri::Builder::default()
    .manage(PtyManager::default())
    .manage(EngineAdapterRegistry::with_builtins())
    .invoke_handler(tauri::generate_handler![
      pty_spawn,
      pty_write,
      pty_stop,
      pty_list_engine_adapters,
      pty_list_all_ids,
      pty_unregister_engine_adapter,
      pty_find_by_engine_type,
      pty_find_by_version
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pty;

use pty::{pty_list_engine_adapters, pty_spawn, pty_stop, pty_write};
use pty::registry::{EngineAdapterRegistry, PtyManager};

fn main() {
  tauri::Builder::default()
    .manage(PtyManager::default())
    .manage(EngineAdapterRegistry::with_builtins())
    .invoke_handler(tauri::generate_handler![
      pty_spawn,
      pty_write,
      pty_stop,
      pty_list_engine_adapters
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

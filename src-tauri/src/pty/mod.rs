use tauri::{AppHandle, State};
use uuid::Uuid;

use self::{
  registry::{EngineAdapterQuery, EngineAdapterRegistry, PtyManager, PtySession},
  runtime::start_output_pump,
  worktree::build_command,
};
use crate::pty::adapters::EngineMetadata;

pub mod adapters;
pub mod registry;
pub mod runtime;
pub mod worktree;

#[tauri::command]
pub fn pty_list_engine_adapters(
  adapters: State<EngineAdapterRegistry>,
  query: Option<String>,
) -> Result<Vec<String>, String> {
  let query = match query.as_deref() {
    None | Some("all") => EngineAdapterQuery::All,
    Some("detected") => EngineAdapterQuery::Detected,
    Some("healthy") => EngineAdapterQuery::Healthy,
    Some(other) => {
      return Err(format!(
        "invalid query '{other}', expected one of: all | detected | healthy"
      ))
    }
  };

  adapters.query_ids(query)
}

#[tauri::command]
pub fn pty_list_all_ids(
  adapters: State<EngineAdapterRegistry>,
) -> Result<Vec<String>, String> {
  adapters.list_ids()
}

#[tauri::command]
pub fn pty_unregister_engine_adapter(
  adapters: State<EngineAdapterRegistry>,
  id: String,
) -> Result<bool, String> {
  let removed = adapters.unregister(&id)?;
  Ok(removed.is_some())
}

#[tauri::command]
pub fn pty_find_by_engine_type(
  adapters: State<EngineAdapterRegistry>,
  engine_type: String,
) -> Result<Vec<EngineMetadata>, String> {
  let matches = adapters.find_by_engine_type(&engine_type)?;
  Ok(matches.into_iter().map(|a| a.metadata()).collect())
}

#[tauri::command]
pub fn pty_find_by_version(
  adapters: State<EngineAdapterRegistry>,
  engine_type: String,
  version: String,
) -> Result<Vec<EngineMetadata>, String> {
  let matches = adapters.find_by_version(&engine_type, &version)?;
  Ok(matches.into_iter().map(|a| a.metadata()).collect())
}

#[tauri::command]
pub fn pty_spawn(
  app: AppHandle,
  manager: State<PtyManager>,
  adapters: State<EngineAdapterRegistry>,
  program: String,
  args: Vec<String>,
  cols: u16,
  rows: u16,
) -> Result<String, String> {
  let cmd = build_command(program, args);
  let adapter = adapters
    .select_default()?
    .ok_or_else(|| "no PTY engine adapter available".to_string())?;
  let adapter_id = adapter.id().to_string();
  let spawned = adapter.spawn(cmd, cols, rows)?;

  let id = Uuid::new_v4().to_string();
  {
    let mut sessions = manager
      .sessions
      .lock()
      .map_err(|_| "pty sessions lock poisoned".to_string())?;
    sessions.insert(
      id.clone(),
      PtySession {
        adapter_id,
        writer: spawned.writer,
        child: spawned.child,
      },
    );
  }

  start_output_pump(app, id.clone(), spawned.reader);
  Ok(id)
}

#[tauri::command]
pub fn pty_write(manager: State<PtyManager>, id: String, data: String) -> Result<(), String> {
  use std::io::Write;

  let mut sessions = manager
    .sessions
    .lock()
    .map_err(|_| "pty sessions lock poisoned".to_string())?;
  let session = sessions
    .get_mut(&id)
    .ok_or_else(|| "pty session not found".to_string())?;

  session
    .writer
    .write_all(data.as_bytes())
    .map_err(|e| e.to_string())?;
  session.writer.flush().map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
pub fn pty_stop(
  manager: State<PtyManager>,
  adapters: State<EngineAdapterRegistry>,
  id: String,
) -> Result<(), String> {
  let session = {
    let mut sessions = manager
      .sessions
      .lock()
      .map_err(|_| "pty sessions lock poisoned".to_string())?;
    sessions.remove(&id)
  };

  if let Some(mut session) = session {
    if let Some(adapter) = adapters.get(&session.adapter_id)? {
      let _ = adapter.stop(session.child.as_mut());
    } else {
      // sessions can outlive adapter registrations during development
      let _ = session.child.kill();
      let _ = session.child.wait();
    }
  }

  Ok(())
}

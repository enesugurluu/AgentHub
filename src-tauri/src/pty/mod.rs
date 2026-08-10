use tauri::{AppHandle, State};
use uuid::Uuid;
use serde::Serialize;

use self::{
  registry::{EngineAdapterQuery, EngineAdapterRegistry, PtyManager, PtySession},
  runtime::start_output_pump,
  worktree::build_command,
};
use crate::pty::adapters::EngineMetadata;
use crate::worktree::resolve_worktree_path_for_agent;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSpawnResult {
  pub agent_id: String,
  pub execution_id: String,
}

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
pub fn agent_spawn(
  app: AppHandle,
  manager: State<PtyManager>,
  adapters: State<EngineAdapterRegistry>,
  agent_id: String,
  program: String,
  args: Vec<String>,
  cols: u16,
  rows: u16,
) -> Result<AgentSpawnResult, String> {
  // In a real scenario, this would be resolved from the backend agent registry.
  // We resolve the worktree securely without trusting the frontend.
  let repo_path = std::env::current_dir().unwrap_or_default().to_string_lossy().to_string();
  let worktree_path = resolve_worktree_path_for_agent(&repo_path, &agent_id).ok();

  let mut envs = vec![
    ("AGENTHUB_AGENT_ID".to_string(), agent_id.clone())
  ];
  if let Some(ref wt) = worktree_path {
    envs.push(("AGENTHUB_WORKTREE".to_string(), wt.clone()));
  }

  let cmd = build_command(program, args, worktree_path, envs);
  let adapter = adapters
    .select_default()?
    .ok_or_else(|| "no PTY engine adapter available".to_string())?;
  let adapter_id = adapter.id().to_string();
  let spawned = adapter.spawn(cmd, cols, rows)?;

  let id = agent_id.clone();
  let execution_id = Uuid::new_v4().to_string();

  {
    let mut sessions = manager
      .sessions
      .lock()
      .map_err(|_| "pty sessions lock poisoned".to_string())?;

    if sessions.contains_key(&id) {
      return Err(format!("agent {} is already running", id));
    }

    sessions.insert(
      id.clone(),
      PtySession {
        adapter_id,
        execution_id: execution_id.clone(),
        writer: spawned.writer,
        child: spawned.child,
        #[cfg(target_os = "windows")]
        job_handle: spawned.job_handle,
      },
    );
  }

  start_output_pump(app, id.clone(), execution_id.clone(), spawned.reader);

  Ok(AgentSpawnResult {
    agent_id: id,
    execution_id,
  })
}

#[tauri::command]
pub fn agent_write(manager: State<PtyManager>, agent_id: String, execution_id: String, data: String) -> Result<(), String> {
  use std::io::Write;

  let mut sessions = manager
    .sessions
    .lock()
    .map_err(|_| "pty sessions lock poisoned".to_string())?;
  let session = sessions
    .get_mut(&agent_id)
    .ok_or_else(|| "pty session not found".to_string())?;

  if session.execution_id != execution_id {
    return Err("stale execution ID".to_string());
  }

  session
    .writer
    .write_all(data.as_bytes())
    .map_err(|e| e.to_string())?;
  session.writer.flush().map_err(|e| e.to_string())?;
  Ok(())
}

#[tauri::command]
pub fn agent_stop(
  manager: State<PtyManager>,
  adapters: State<EngineAdapterRegistry>,
  agent_id: String,
  execution_id: String,
) -> Result<(), String> {
  let session = {
    let mut sessions = manager
      .sessions
      .lock()
      .map_err(|_| "pty sessions lock poisoned".to_string())?;
    if let Some(session) = sessions.get(&agent_id) {
       if session.execution_id != execution_id {
           return Err("stale execution ID".to_string());
       }
    } else {
       return Err("pty session not found".to_string());
    }
    sessions.remove(&agent_id)
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

use serde::Serialize;
use tauri::ipc::Channel;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use self::{
  registry::{EngineAdapterQuery, EngineAdapterRegistry, PtyManager, PtySession},
  runtime::{start_output_pump, PtyEvent},
  worktree::build_command,
};
use crate::db::AppDb;
use crate::pty::adapters::{CliSpawnOptions, EngineMetadata};
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

/// Repo kökünü çözer. Öncelik bilinçli override'a (`AGENTHUB_REPO_PATH` env)
/// verilir; aksi halde uygulama sürecinin çalışma dizini kullanılır
/// (FAZ0 basit davranışı; dialog ile repo seçimi FAZ1'de).
fn resolve_repo_root() -> String {
  if let Ok(raw) = std::env::var("AGENTHUB_REPO_PATH") {
    let trimmed = raw.trim();
    if !trimmed.is_empty() {
      return trimmed.to_string();
    }
  }
  std::env::current_dir()
    .unwrap_or_default()
    .to_string_lossy()
    .to_string()
}

/// Oturum zaten açıksa spawn'dan ÖNCE hata ver — aksi halde register hatasında
/// yetim (kill edilmemiş) bir süreç kalırdı.
fn ensure_not_running(manager: &State<PtyManager>, agent_id: &str) -> Result<(), String> {
  let sessions = manager
    .sessions
    .lock()
    .map_err(|_| "pty sessions lock poisoned".to_string())?;
  if sessions.contains_key(agent_id) {
    return Err(format!("agent {agent_id} is already running"));
  }
  Ok(())
}

/// Ajanın çalışacağı dizini çözer: önce ajanın yönetilen worktree'si,
/// yoksa repo köküne geri düşer (FAZ0 davranışı).
fn resolve_agent_workdir(repo_path: &str, agent_id: &str) -> String {
  match resolve_worktree_path_for_agent(repo_path, agent_id) {
    Ok(path) => path,
    Err(e) => {
      tracing::warn!(agent_id, "worktree bulunamadı, repo köküne düşülüyor: {e}");
      repo_path.to_string()
    }
  }
}

/// Ajan için ortam değişkenleri (execution izolasyonu).
fn agent_envs(agent_id: &str, worktree_path: &str) -> Vec<(String, String)> {
  vec![
    ("AGENTHUB_AGENT_ID".to_string(), agent_id.to_string()),
    ("AGENTHUB_WORKTREE".to_string(), worktree_path.to_string()),
  ]
}

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
pub fn pty_list_all_ids(adapters: State<EngineAdapterRegistry>) -> Result<Vec<String>, String> {
  adapters.list_ids()
}

/// Tek adaptörün metadata'sını döndürür; Settings UI id → metadata çözümlemesini
/// bununla yapar (frontend'deki id→engine_type tahminine gerek kalmaz).
#[tauri::command]
pub fn pty_adapter_metadata(
  adapters: State<EngineAdapterRegistry>,
  id: String,
) -> Result<EngineMetadata, String> {
  adapters
    .get(&id)?
    .map(|adapter| adapter.metadata())
    .ok_or_else(|| format!("no adapter registered with id '{id}'"))
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

/// Genel shell/PTY spawn (frontend'den program+args alır).
#[tauri::command]
#[allow(clippy::too_many_arguments)] // Tauri state/channel enjeksiyonu argüman sayısını şişirir
pub fn agent_spawn(
  app: AppHandle,
  manager: State<PtyManager>,
  adapters: State<EngineAdapterRegistry>,
  agent_id: String,
  program: String,
  args: Vec<String>,
  cols: u16,
  rows: u16,
  channel: Channel<PtyEvent>,
) -> Result<AgentSpawnResult, String> {
  // Oturum çakışmasını spawn'dan önce yakala (yetim süreç önlemi).
  ensure_not_running(&manager, &agent_id)?;

  // Worktree güvenli şekilde backend'de çözülür; frontend'e güvenilmez.
  let repo_path = resolve_repo_root();
  let worktree_path = resolve_agent_workdir(&repo_path, &agent_id);

  let envs = agent_envs(&agent_id, &worktree_path);
  let cmd = build_command(program, args, Some(worktree_path), envs);

  // Yalnızca "pty" motorlu adaptörler: shell spawn'ı alfabetik sırayla
  // claude-code gibi CLI adaptörlerine kaymasın.
  let adapter = adapters
    .select_default_for_engine_type("pty")?
    .ok_or_else(|| "no PTY engine adapter available".to_string())?;
  let adapter_id = adapter.id().to_string();
  let spawned = adapter.spawn(cmd, cols, rows)?;

  let execution_id = Uuid::new_v4().to_string();
  register_session(
    &app,
    &manager,
    &agent_id,
    &execution_id,
    &adapter_id,
    spawned,
    channel,
    "spawn",
  )?;

  Ok(AgentSpawnResult {
    agent_id,
    execution_id,
  })
}

/// Motor tipine göre spawn (ör. `engine_type = "claude"`): adaptör komutu kendi
/// kurallarıyla kurar (CliSpawnOptions). `program/args` frontend'den gelmez.
#[tauri::command]
#[allow(clippy::too_many_arguments)] // Tauri state/channel enjeksiyonu argüman sayısını şişirir
pub fn agent_spawn_engine(
  app: AppHandle,
  manager: State<PtyManager>,
  adapters: State<EngineAdapterRegistry>,
  agent_id: String,
  engine_type: String,
  cols: u16,
  rows: u16,
  channel: Channel<PtyEvent>,
) -> Result<AgentSpawnResult, String> {
  // Oturum çakışmasını spawn'dan önce yakala (yetim süreç önlemi).
  ensure_not_running(&manager, &agent_id)?;

  let adapter = adapters
    .find_by_engine_type(&engine_type)?
    .into_iter()
    .next()
    .ok_or_else(|| format!("no adapter registered for engine type '{engine_type}'"))?;
  let adapter_id = adapter.id().to_string();

  let repo_path = resolve_repo_root();
  let worktree_path = resolve_agent_workdir(&repo_path, &agent_id);

  let opts = CliSpawnOptions {
    workdir: std::path::PathBuf::from(&worktree_path),
    env: agent_envs(&agent_id, &worktree_path),
    args: Vec::new(),
  };
  let spawned = adapter.spawn_cli(opts, cols, rows)?;

  let execution_id = Uuid::new_v4().to_string();
  register_session(
    &app,
    &manager,
    &agent_id,
    &execution_id,
    &adapter_id,
    spawned,
    channel,
    "spawn_engine",
  )?;

  Ok(AgentSpawnResult {
    agent_id,
    execution_id,
  })
}

/// Ortak oturum kaydı + output pump + DB olay kaydı.
#[allow(clippy::too_many_arguments)]
fn register_session(
  app: &AppHandle,
  manager: &State<PtyManager>,
  agent_id: &str,
  execution_id: &str,
  adapter_id: &str,
  spawned: adapters::SpawnedPty,
  channel: Channel<PtyEvent>,
  event_type: &str,
) -> Result<(), String> {
  let id = agent_id.to_string();
  let execution_id_owned = execution_id.to_string();

  {
    let mut sessions = manager
      .sessions
      .lock()
      .map_err(|_| "pty sessions lock poisoned".to_string())?;

    if sessions.contains_key(&id) {
      // TOCTOU yedek koruması: bu noktada süreç spawn edilmiş durumdadır;
      // hata dönerken yetim bırakmamak için öldürülür.
      drop(sessions);
      let mut child = spawned.child;
      let _ = child.kill();
      let _ = child.wait();
      return Err(format!("agent {id} is already running"));
    }

    sessions.insert(
      id.clone(),
      PtySession {
        adapter_id: adapter_id.to_string(),
        execution_id: execution_id_owned.clone(),
        writer: spawned.writer,
        master: spawned.master,
        child: spawned.child,
        #[cfg(target_os = "windows")]
        job_handle: spawned.job_handle,
      },
    );
  }

  if let Some(db) = app.try_state::<AppDb>() {
    let payload = serde_json::json!({ "executionId": execution_id, "adapter": adapter_id });
    let _ = db.record_event(Some(&id), None, event_type, Some(&payload.to_string()));
  }

  start_output_pump(
    app.clone(),
    id,
    execution_id_owned,
    spawned.reader,
    channel,
  );
  Ok(())
}

#[tauri::command]
pub fn agent_write(
  manager: State<PtyManager>,
  agent_id: String,
  execution_id: String,
  data: String,
) -> Result<(), String> {
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

/// PTY boyutunu backend'e bildirir (xterm fit → ConPTY/POSIX resize).
#[tauri::command]
pub fn pty_resize(
  manager: State<PtyManager>,
  adapters: State<EngineAdapterRegistry>,
  agent_id: String,
  execution_id: String,
  cols: u16,
  rows: u16,
) -> Result<(), String> {
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

  if let Some(adapter) = adapters.get(&session.adapter_id)? {
    adapter.resize(session.master.as_ref(), cols, rows)
  } else {
    // Sessions can outlive adapter registrations during development.
    session
      .master
      .resize(portable_pty::PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
      })
      .map_err(|e| e.to_string())
  }
}

#[tauri::command]
pub fn agent_stop(
  app: AppHandle,
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

    if let Some(db) = app.try_state::<AppDb>() {
      let _ = db.record_event(Some(&agent_id), None, "stopped", None);
    }
  }

  Ok(())
}

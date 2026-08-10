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
use crate::pty::adapters::{EngineMetadata, SpawnOptions};
use crate::worktree::{ensure_agent_worktree, link_node_modules, prepare_worktree_env};

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

/// Repo kökünü çözer (ADR-7; WP-06). Öncelik:
/// 1. `settings.repo_path` (dialog ile seçilmiş, doğrulanmış — asıl akış)
/// 2. `AGENTHUB_REPO_PATH` env (dev köprüsü — FAZ0 davranışı korunur)
/// 3. uygulama sürecinin çalışma dizini (son çare; paketli çalıştırmada uyarı)
fn resolve_repo_root(app: &AppHandle) -> String {
  if let Some(db) = app.try_state::<AppDb>() {
    if let Ok(Some(repo)) = db.setting_get("repo_path") {
      if !repo.trim().is_empty() {
        return repo;
      }
    }
  }
  if let Ok(raw) = std::env::var("AGENTHUB_REPO_PATH") {
    let trimmed = raw.trim();
    if !trimmed.is_empty() {
      return trimmed.to_string();
    }
  }
  let cwd = std::env::current_dir()
    .unwrap_or_default()
    .to_string_lossy()
    .to_string();
  if cwd.is_empty() {
    tracing::warn!("repo_path ayarlanmamış — lütfen Ayarlar'dan bir proje seçin");
  }
  cwd
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

/// Ajanın çalışacağı dizini çözer (ADR-5/WP-05): DB'den ajanı okur, worktree'sini
/// **garanti eder** (yoksa `agent/<slug>` branch'iyle oluşturur), `.env.local` +
/// node_modules paylaşımını hazırlar. Repo köküne **DÜŞMEZ** — ajan DB'de yoksa
/// açıklayıcı hata döner ("önce işe al").
fn resolve_agent_workdir(
  db: Option<&AppDb>,
  repo_path: &str,
  agent_id: &str,
) -> Result<String, String> {
  let db = db.ok_or_else(|| "veritabanı hazır değil — ajan doğrulanamadı".to_string())?;
  let id: i64 = agent_id
    .parse()
    .map_err(|_| format!("geçersiz ajan kimliği '{agent_id}' — önce ajanı işe alın"))?;
  let agent = db.get_agent(id)?;
  let base_branch = db
    .setting_get("main_branch")?
    .unwrap_or_else(|| "main".to_string());

  let info = ensure_agent_worktree(repo_path, agent_id, &agent.name, &base_branch)?;

  // İzolasyon güçlendirmeleri: hata spawn'ı engellemez (warn + devam).
  if let Err(e) = prepare_worktree_env(std::path::Path::new(&info.path), id) {
    tracing::warn!(agent_id, "worktree env hazırlanamadı: {e}");
  }
  // `repo_path` Windows'tan gelen raw yol olabilir; link_node_modules'a sanitizan geç.
  let sanitized_repo = crate::worktree::sanitize_repo_path(repo_path);
  link_node_modules(std::path::Path::new(&info.path), &sanitized_repo);

  Ok(info.path)
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

/// Tek adaptörün detect bilgisini (kurulu mu + sürüm + capability + install_hint)
/// döndürür — Hire Wizard Adım 2 ve Settings "Motorlar" (WP-07/12) bunu kullanır.
#[tauri::command]
pub fn pty_adapter_detect_info(
  adapters: State<EngineAdapterRegistry>,
  id: String,
) -> Result<crate::pty::adapters::DetectResult, String> {
  adapters
    .get(&id)?
    .map(|adapter| adapter.detect_info())
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

  // Worktree güvenli şekilde backend'de çözülür (garanti edilir — ADR-5); frontend'e güvenilmez.
  let db = app.try_state::<AppDb>().map(|s| s.inner());
  let repo_path = resolve_repo_root(&app);
  let worktree_path = resolve_agent_workdir(db, &repo_path, &agent_id)?;

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
    "pty",
    false,
    None,
    None,
  )?;

  Ok(AgentSpawnResult {
    agent_id,
    execution_id,
  })
}

/// Motor kurulumunu **backend'de çözülen komutla** (adaptörün `install_command()`)
/// ayrı bir PTY oturumunda çalıştırır (docs 7.5; FAZ0 S5 — frontend program/args göndermez).
///
/// Oturum `agent_id = "install-<engine_type>"` ile açılır; Settings "Motorlar"
/// sekmesindeki kurulum akışı (WP-12) bunu kullanır. Kurulu motor için hata döner.
#[tauri::command]
#[allow(clippy::too_many_arguments)] // Tauri state/channel enjeksiyonu argüman sayısını şişirir
pub fn agent_install_engine(
  app: AppHandle,
  manager: State<PtyManager>,
  adapters: State<EngineAdapterRegistry>,
  engine_type: String,
  cols: u16,
  rows: u16,
  channel: Channel<PtyEvent>,
) -> Result<AgentSpawnResult, String> {
  let agent_id = format!("install-{engine_type}");

  // Aynı motor için eşzamanlı kurulum engellenir (yetim süreç önlemi).
  ensure_not_running(&manager, &agent_id)?;

  let adapter = adapters
    .find_by_engine_type(&engine_type)?
    .into_iter()
    .next()
    .ok_or_else(|| format!("no adapter registered for engine type '{engine_type}'"))?;

  if adapter.detect() {
    return Err(format!("engine '{engine_type}' zaten kurulu"));
  }

  let install_cmd = adapter
    .install_command()
    .ok_or_else(|| format!("engine '{engine_type}' için kurulum komutu tanımlı değil"))?;
  if install_cmd.is_empty() {
    return Err("kurulum komutu boş".to_string());
  }

  let mut cmd = portable_pty::CommandBuilder::new(&install_cmd[0]);
  for arg in &install_cmd[1..] {
    cmd.arg(arg);
  }
  // Kurulumlar globaldir (npm -g / pip / curl|bash); cwd uygulama çalışma dizinidir.
  if let Ok(cwd) = std::env::current_dir() {
    cmd.cwd(cwd);
  }

  // Kurulum, izolasyonu hazır olan PTY adaptörüyle spawn edilir (Job Object/process group).
  let pty_adapter = adapters
    .select_default_for_engine_type("pty")?
    .ok_or_else(|| "no PTY engine adapter available".to_string())?;
  let adapter_id = pty_adapter.id().to_string();
  let spawned = pty_adapter.spawn(cmd, cols, rows)?;

  let execution_id = Uuid::new_v4().to_string();
  register_session(
    &app,
    &manager,
    &agent_id,
    &execution_id,
    &adapter_id,
    spawned,
    channel,
    "install",
    "pty",
    false,
    None,
    None,
  )?;

  Ok(AgentSpawnResult {
    agent_id,
    execution_id,
  })
}

/// Motor tipine göre spawn (ör. `engine_type = "claude"`): adaptör komutu kendi
/// kurallarıyla kurar (`SpawnOptions`). `program/args` frontend'den gelmez.
///
/// `options.workdir` boşsa backend worktree'yi çözer ve doldurur (FAZ0 davranışı);
/// `env` boşsa ajan ortam değişkenleri eklenir.
#[tauri::command]
#[allow(clippy::too_many_arguments)] // Tauri state/channel enjeksiyonu argüman sayısını şişirir
pub fn agent_spawn_engine(
  app: AppHandle,
  manager: State<PtyManager>,
  adapters: State<EngineAdapterRegistry>,
  agent_id: String,
  engine_type: String,
  options: SpawnOptions,
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

  // WP-13: adaptörün desteklemediği alanlar yok sayılır ama görünür (capability ilanı).
  if options.max_budget_usd.is_some() && !adapter.supports("budget") {
    tracing::warn!(adapter = adapter_id, "budget flag'i desteklenmiyor — yok sayılıyor");
  }
  if options.max_turns.is_some() && !adapter.supports("turns") {
    tracing::warn!(adapter = adapter_id, "max_turns flag'i desteklenmiyor — yok sayılıyor");
  }
  if options.effort.is_some() && !adapter.supports("effort") {
    tracing::warn!(adapter = adapter_id, "effort flag'i desteklenmiyor — yok sayılıyor");
  }

  let db = app.try_state::<AppDb>().map(|s| s.inner());
  let repo_path = resolve_repo_root(&app);
  let worktree_path = resolve_agent_workdir(db, &repo_path, &agent_id)?;

  // Frontend workdir/env göndermemişse backend tamamlar.
  let mut opts = options;
  if opts.workdir.as_os_str().is_empty() {
    opts.workdir = std::path::PathBuf::from(&worktree_path);
  }
  if opts.env.is_empty() {
    opts.env = agent_envs(&agent_id, &worktree_path);
  }
  let non_interactive = opts.non_interactive;
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
    &engine_type,
    non_interactive,
    None,
    None,
  )?;

  Ok(AgentSpawnResult {
    agent_id,
    execution_id,
  })
}

/// Görevi ajana atar ve spawn eder (docs 13.1–13.2; ADR-6/WP-10):
/// worktree garanti → `AGENT_TASK.md` → non-interactive `SpawnOptions` (bütçe/turn)
/// → `agent_spawn_engine` mantığıyla spawn; oturum `task_id` ile etiketlenir,
/// `tasks` satırı `in_progress`'e alınır. Tamamlanma exit'te algılanır (runtime).
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn task_assign(
  app: AppHandle,
  manager: State<PtyManager>,
  adapters: State<EngineAdapterRegistry>,
  db: State<AppDb>,
  agent_id: i64,
  task_id: i64,
  cols: u16,
  rows: u16,
  channel: Channel<PtyEvent>,
) -> Result<AgentSpawnResult, String> {
  let agent_id_str = agent_id.to_string();
  ensure_not_running(&manager, &agent_id_str)?;

  let agent = db.get_agent(agent_id)?;
  let task = db.get_task(task_id)?;
  if agent.status == "fired" {
    return Err(format!("ajan {agent_id} işten çıkarılmış — görev atanamaz"));
  }

  // Ajanın motoru için adaptör.
  let adapter = adapters
    .find_by_engine_type(&agent.motor)?
    .into_iter()
    .next()
    .ok_or_else(|| format!("ajanın motoru için adaptör yok: {}", agent.motor))?;
  let adapter_id = adapter.id().to_string();

  // Worktree garanti (ADR-5) + AGENT_TASK.md.
  let repo_path = resolve_repo_root(&app);
  let base_branch = db
    .setting_get("main_branch")?
    .unwrap_or_else(|| "main".to_string());
  let info = ensure_agent_worktree(&repo_path, &agent_id_str, &agent.name, &base_branch)?;
  let wt_path = std::path::PathBuf::from(&info.path);
  let task_file = crate::tasks::write_agent_task(&wt_path, &task, &agent, &info.branch_name)?;

  // Ajan config'inden model/effort/bütçe/turn; görev bütçesi önceliklidir.
  let config: serde_json::Value =
    serde_json::from_str(agent.config_json.as_deref().unwrap_or("{}")).unwrap_or_default();
  let budget = task.budget.or_else(|| {
    config
      .get("max_budget_usd")
      .and_then(|v| v.as_f64())
  });
  let turns = config.get("max_turns").and_then(|v| v.as_u64()).map(|v| v as u32);
  let model = config.get("model").and_then(|v| v.as_str()).map(|s| s.to_string());
  let effort = config
    .get("effort")
    .and_then(|v| v.as_str())
    .and_then(|s| {
      serde_json::from_str::<crate::pty::adapters::Effort>(&format!("\"{s}\""))
        .ok()
    });

  let opts = SpawnOptions {
    workdir: wt_path.clone(),
    env: agent_envs(&agent_id_str, &info.path),
    args: Vec::new(),
    model,
    effort,
    max_budget_usd: budget,
    max_turns: turns,
    non_interactive: true,
    task_file: Some(task_file),
  };

  // WP-13: desteklenmeyen flag'ler için görünür uyarı (capability ilanına göre).
  if opts.max_budget_usd.is_some() && !adapter.supports("budget") {
    tracing::warn!(adapter = adapter_id, "budget flag'i desteklenmiyor — yok sayılıyor");
  }
  if opts.max_turns.is_some() && !adapter.supports("turns") {
    tracing::warn!(adapter = adapter_id, "max_turns flag'i desteklenmiyor — yok sayılıyor");
  }

  let spawned = adapter.spawn_cli(opts, cols, rows)?;

  let execution_id = Uuid::new_v4().to_string();
  register_session(
    &app,
    &manager,
    &agent_id_str,
    &execution_id,
    &adapter_id,
    spawned,
    channel,
    "task_assign",
    &agent.motor,
    true,
    Some(task_id),
    Some(wt_path),
  )?;

  // tasks → in_progress.
  db.assign_task(task_id, agent_id, &info.path)?;

  Ok(AgentSpawnResult {
    agent_id: agent_id_str,
    execution_id,
  })
}

/// Ortak oturum kaydı + output pump + DB olay kaydı.
///
/// `engine_type` + `non_interactive` → `select_parser` (WP-04): claude+print →
/// stream-json, opencode → jsonl, diğer → regex.
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
  engine_type: &str,
  non_interactive: bool,
  task_id: Option<i64>,
  workdir: Option<std::path::PathBuf>,
) -> Result<(), String> {
  let id = agent_id.to_string();
  let execution_id_owned = execution_id.to_string();

  // JSONL oturum kaydı (docs 12.2; WP-11): ~/.agentcompany/logs/<agent>/manual-<epoch>.jsonl
  // İç blok DIŞINDA tanımlanır — start_output_pump'a aktarılır (E0425 önlemi).
  let transcript_path = crate::pty::runtime::transcript::agentcompany_logs_dir().and_then(
    |base| crate::pty::runtime::transcript::open_transcript(&base, agent_id, "manual").ok(),
  );

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
        last_completion: std::sync::Arc::new(std::sync::Mutex::new(None)),
        transcript_path: transcript_path.clone(),
        task_id,
        worktree_path: workdir.clone(),
        #[cfg(target_os = "windows")]
        job_handle: spawned.job_handle,
      },
    );
  }

  if let Some(db) = app.try_state::<AppDb>() {
    let payload = serde_json::json!({ "executionId": execution_id, "adapter": adapter_id });
    let _ = db.record_event(Some(&id), task_id, event_type, Some(&payload.to_string()));
  }

  start_output_pump(
    app.clone(),
    id,
    execution_id_owned,
    spawned.reader,
    channel,
    crate::pty::runtime::parser::select_parser(engine_type, non_interactive),
    transcript_path,
    task_id,
    workdir,
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

  // JSONL input satırı (docs 12.2; WP-11).
  if let Some(path) = &session.transcript_path {
    let _ = crate::pty::runtime::transcript::append_transcript_entry(
      path,
      serde_json::json!({
        "ts": crate::pty::runtime::transcript::epoch_seconds(),
        "type": "input",
        "content": data,
      }),
    );
  }
  Ok(())
}

/// Terminal buffer'ını oturum JSONL'ına `session_buffer` satırı olarak ekler
/// (xterm-addon-serialize — docs 12.2; FAZ0 serialize ertelemesi kapanır).
#[tauri::command]
pub fn transcript_append_session_buffer(
  manager: State<PtyManager>,
  agent_id: String,
  execution_id: String,
  text: String,
) -> Result<(), String> {
  let sessions = manager
    .sessions
    .lock()
    .map_err(|_| "pty sessions lock poisoned".to_string())?;
  let session = sessions
    .get(&agent_id)
    .ok_or_else(|| "pty session not found".to_string())?;

  if session.execution_id != execution_id {
    return Err("stale execution ID".to_string());
  }

  if let Some(path) = &session.transcript_path {
    crate::pty::runtime::transcript::append_transcript_entry(
      path,
      serde_json::json!({
        "ts": crate::pty::runtime::transcript::epoch_seconds(),
        "type": "session_buffer",
        "content": text,
      }),
    )?;
  }
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
      let _ = adapters::stop_child_tree(session.child.as_mut());
    }

    if let Some(db) = app.try_state::<AppDb>() {
      let _ = db.record_event(Some(&agent_id), None, "stopped", None);
    }
  }

  Ok(())
}

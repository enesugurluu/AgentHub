//! SQLite veri katmanı — AjanOfis docs Bölüm 12.1 şeması (FAZ0 alt kümesi).
//!
//! FAZ0: `agents`, `tasks`, `events`, `settings`.
//! FAZ4'te eklenecek: `notes`, `note_links`, `note_fts`, `note_vec` (sqlite-vec).

use std::path::PathBuf;
use std::sync::Mutex;

use rusqlite::{params, Connection};
use serde::Serialize;
use tauri::State;

/// Migration şeması (idempotent — CREATE IF NOT EXISTS).
const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS agents (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  role TEXT NOT NULL,
  avatar_color TEXT,
  motor TEXT NOT NULL,
  model TEXT,
  system_prompt TEXT,
  config_json TEXT,
  worktree_path TEXT,
  status TEXT DEFAULT 'idle',
  created_at TEXT DEFAULT (datetime('now')),
  hired_at TEXT,
  fired_at TEXT
);

CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  acceptance_criteria TEXT,
  column TEXT DEFAULT 'backlog',
  assigned_agent_id INTEGER REFERENCES agents(id),
  parent_task_id INTEGER REFERENCES tasks(id),
  priority INTEGER DEFAULT 3,
  budget REAL,
  spent_tokens_input INTEGER DEFAULT 0,
  spent_tokens_output INTEGER DEFAULT 0,
  spent_cost REAL DEFAULT 0,
  worktree_path TEXT,
  created_at TEXT DEFAULT (datetime('now')),
  started_at TEXT,
  completed_at TEXT,
  blocked_by INTEGER
);

CREATE TABLE IF NOT EXISTS events (
  id INTEGER PRIMARY KEY,
  agent_id INTEGER REFERENCES agents(id),
  task_id INTEGER REFERENCES tasks(id),
  event_type TEXT NOT NULL,
  payload TEXT,
  timestamp TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS settings (
  key TEXT PRIMARY KEY,
  value TEXT
);
"#;

/// Tauri state olarak tutulan paylaşılan DB bağlantısı.
pub struct AppDb {
  conn: Mutex<Connection>,
}

/// Frontend'e dönen ajan kaydı.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRecord {
  pub id: i64,
  pub name: String,
  pub role: String,
  pub motor: String,
  pub model: Option<String>,
  pub status: String,
  pub worktree_path: Option<String>,
  pub created_at: Option<String>,
}

impl AppDb {
  /// DB'yi açar; WAL modunu etkinleştirir; şemayı kurar; boşsa seed verisi ekler.
  pub fn open(data_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
    let path = data_dir.join("agenthub.db");
    let conn = Connection::open(&path)?;

    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", true)?;
    conn.execute_batch(SCHEMA)?;

    let db = AppDb {
      conn: Mutex::new(conn),
    };
    db.seed_demo_agents()?;
    Ok(db)
  }

  /// İlk çalıştırmada UI'ın boş görünmemesi için demo ajanlar (FAZ0).
  fn seed_demo_agents(&self) -> Result<(), Box<dyn std::error::Error>> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM agents", [], |row| row.get(0))?;
    if count == 0 {
      conn.execute_batch(
        r#"
        INSERT INTO agents (name, role, motor, model, status) VALUES
          ('Ada',   'Frontend Dev', 'claude', NULL, 'idle'),
          ('Orion', 'Backend Dev',  'claude', NULL, 'idle'),
          ('Nova',  'QA',           'claude', NULL, 'idle');
        "#,
      )?;
    }
    Ok(())
  }

  /// Denetim/olay kaydı (spawn, exit, stopped, ...).
  pub fn record_event(
    &self,
    agent_id: Option<&str>,
    task_id: Option<i64>,
    event_type: &str,
    payload: Option<&str>,
  ) -> Result<(), String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    conn
      .execute(
        "INSERT INTO events (agent_id, task_id, event_type, payload) VALUES (?1, ?2, ?3, ?4)",
        params![agent_id, task_id, event_type, payload],
      )
      .map_err(|e| e.to_string())?;
    Ok(())
  }
}

/// Tüm ajan kayıtlarını listeler (sol panel + inspector).
#[tauri::command]
pub fn agent_list_all(db: State<AppDb>) -> Result<Vec<AgentRecord>, String> {
  let conn = db.conn.lock().map_err(|_| "db lock poisoned".to_string())?;

  let mut stmt = conn
    .prepare(
      "SELECT id, name, role, motor, model, status, worktree_path, created_at
       FROM agents ORDER BY id",
    )
    .map_err(|e| e.to_string())?;

  let rows = stmt
    .query_map([], |row| {
      Ok(AgentRecord {
        id: row.get(0)?,
        name: row.get(1)?,
        role: row.get(2)?,
        motor: row.get(3)?,
        model: row.get(4)?,
        status: row.get(5)?,
        worktree_path: row.get(6)?,
        created_at: row.get(7)?,
      })
    })
    .map_err(|e| e.to_string())?;

  rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

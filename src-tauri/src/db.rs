//! SQLite veri katmanı — AjanOfis docs Bölüm 12.1 şeması (FAZ0 alt kümesi + FAZ1).
//!
//! FAZ0: `agents`, `tasks`, `events`, `settings`.
//! FAZ4'te eklenecek: `notes`, `note_links`, `note_fts`, `note_vec` (sqlite-vec).
//!
//! FAZ1 (WP-01): sıralı migration runner (`PRAGMA user_version`), ajan yaşam döngüsü
//! (`agent_hire/fire/delete/update/get`), `settings_get/set` komutları.

use std::path::PathBuf;
use std::sync::Mutex;

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Sıralı migration'lar. DİZİYE YENİ ŞEMA EKLEME: mevcut girdiyi DEĞİŞTİRME,
/// sona yeni bir girdi ekle (sürüm = dizin + 1). Migration'lar idempotent olmalı.
const MIGRATIONS: &[&str] = &[
    // v1 — FAZ0 taban şeması (docs Bölüm 12.1 alt kümesi).
    r#"
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
    "#,
    // v2 — FAZ1: olay sorgu indeksi + starter company (yalnızca boş tabloya; docs 4.2 alt kümesi).
    r#"
    CREATE INDEX IF NOT EXISTS idx_events_agent_ts ON events(agent_id, timestamp);

    WITH seed(name, role, motor, status) AS (
      VALUES
        ('Ada',   'Frontend Dev', 'claude', 'idle'),
        ('Orion', 'Backend Dev',  'claude', 'idle'),
        ('Nova',  'QA',           'claude', 'idle')
    )
    INSERT INTO agents (name, role, motor, status)
    SELECT name, role, motor, status FROM seed
    WHERE NOT EXISTS (SELECT 1 FROM agents);
    "#,
];

/// Tauri state olarak tutulan paylaşılan DB bağlantısı.
pub struct AppDb {
  conn: Mutex<Connection>,
}

/// Frontend'e dönen ajan kaydı (camelCase).
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
  pub avatar_color: Option<String>,
  pub config_json: Option<String>,
  pub hired_at: Option<String>,
  pub fired_at: Option<String>,
}

/// İşe alım payload'ı (docs 6.1 Adım 2/3 alanları; config_json'a serileşir).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HirePayload {
  pub name: String,
  pub role: String,
  pub motor: String,
  pub model: Option<String>,
  pub effort: Option<String>,
  pub max_budget_usd: Option<f64>,
  pub max_turns: Option<u32>,
  /// full | standard | limited | custom (docs 6.1 Adım 2 — izin profili).
  pub permissions_profile: String,
  pub system_prompt: Option<String>,
  pub avatar_color: Option<String>,
  pub skills: Vec<String>,
  pub mcp_servers: Vec<String>,
}

/// İşten çıkarma seçenekleri (docs 6.2).
///
/// `worktree_action` alanı WP-01'de yalnızca taşınır; asıl worktree davranışı
/// WP-05'te (`worktree_remove` seçenekleri) bağlanır. `keep_logs` WP-11'de
/// (`~/.agentcompany/logs`) kullanılır.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FireOptions {
  /// delete | keep | commit_and_keep
  pub worktree_action: String,
  pub move_open_tasks_to_backlog: bool,
  pub keep_logs: bool,
}

impl Default for FireOptions {
  fn default() -> Self {
    Self {
      worktree_action: "keep".to_string(),
      move_open_tasks_to_backlog: true,
      keep_logs: true,
    }
  }
}

/// Kısmi güncelleme (None = alana dokunma; Some = yaz).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AgentPatch {
  pub name: Option<String>,
  pub role: Option<String>,
  pub motor: Option<String>,
  pub model: Option<String>,
  pub status: Option<String>,
  pub avatar_color: Option<String>,
}

impl AppDb {
  /// DB'yi açar; WAL modunu etkinleştirir; migration'ları uygular.
  pub fn open(data_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
    let path = data_dir.join("agenthub.db");
    let mut conn = Connection::open(&path)?;

    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", true)?;
    migrate(&mut conn)?;

    Ok(AppDb {
      conn: Mutex::new(conn),
    })
  }

  // ---- Ajan yaşam döngüsü (FAZ1 WP-01) -------------------------------------

  /// Yeni ajan kaydı oluşturur; config_json'a hire seçeneklerini serileştirir.
  pub fn hire(&self, payload: &HirePayload) -> Result<AgentRecord, String> {
    let name = payload.name.trim().to_string();
    let role = payload.role.trim().to_string();
    let motor = payload.motor.trim().to_string();
    if name.is_empty() {
      return Err("ajan adı boş olamaz".to_string());
    }
    if role.is_empty() {
      return Err("rol boş olamaz".to_string());
    }
    if motor.is_empty() {
      return Err("motor boş olamaz".to_string());
    }
    let permissions_profile = if payload.permissions_profile.trim().is_empty() {
      "standard".to_string()
    } else {
      payload.permissions_profile.clone()
    };

    let config_json = serde_json::json!({
      "model": payload.model.as_ref(),
      "effort": payload.effort.as_ref(),
      "max_budget_usd": payload.max_budget_usd,
      "max_turns": payload.max_turns,
      "permissions_profile": permissions_profile,
      "skills": payload.skills,
      "mcp_servers": payload.mcp_servers,
    })
    .to_string();

    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    conn
      .execute(
        "INSERT INTO agents
           (name, role, motor, model, system_prompt, avatar_color, config_json, status, hired_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'idle', datetime('now'))",
        params![
          name,
          role,
          motor,
          payload.model,
          payload.system_prompt,
          payload.avatar_color,
          config_json,
        ],
      )
      .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    drop(conn);

    let _ = self.record_event(
      Some(&id.to_string()),
      None,
      "hire",
      Some(&config_json),
    );

    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    get_agent(&conn, id)
  }

  /// Ajanı işten çıkarır: `fired` durumu + açık görevlerin backlog'a dönüşü.
  ///
  /// Worktree aksiyonu (`FireOptions.worktree_action`) WP-05'te uygulanır;
  /// log temizliği (`keep_logs`) WP-11'de.
  pub fn fire(&self, id: i64, options: &FireOptions) -> Result<AgentRecord, String> {
    {
      let conn = self
        .conn
        .lock()
        .map_err(|_| "db lock poisoned".to_string())?;

      let status: Option<String> = conn
        .query_row(
          "SELECT status FROM agents WHERE id = ?1",
          params![id],
          |row| row.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;

      let Some(status) = status else {
        return Err(format!("ajan bulunamadı: {id}"));
      };
      if status == "fired" {
        return Err(format!("ajan {id} zaten işten çıkarılmış"));
      }

      conn
        .execute(
          "UPDATE agents SET status = 'fired', fired_at = datetime('now') WHERE id = ?1",
          params![id],
        )
        .map_err(|e| e.to_string())?;

      if options.move_open_tasks_to_backlog {
        conn
          .execute(
            "UPDATE tasks SET column = 'backlog', assigned_agent_id = NULL
             WHERE assigned_agent_id = ?1 AND column IN ('todo', 'in_progress', 'review')",
            params![id],
          )
          .map_err(|e| e.to_string())?;
      }
    }

    tracing::info!(
      agent_id = id,
      worktree_action = %options.worktree_action,
      keep_logs = options.keep_logs,
      "ajan işten çıkarıldı (worktree aksiyonu WP-05'te uygulanır)"
    );
    let _ = self.record_event(
      Some(&id.to_string()),
      None,
      "fire",
      Some(
        &serde_json::json!({
          "worktreeAction": options.worktree_action,
          "keepLogs": options.keep_logs,
        })
        .to_string(),
      ),
    );

    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    get_agent(&conn, id)
  }

  /// Kalıcı silme — yalnızca `fired` kayıtlar için (yanlışlıkla aktif ajan silinmesin).
  pub fn delete_agent(&self, id: i64) -> Result<(), String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;

    let status: Option<String> = conn
      .query_row(
        "SELECT status FROM agents WHERE id = ?1",
        params![id],
        |row| row.get(0),
      )
      .optional()
      .map_err(|e| e.to_string())?;

    let Some(status) = status else {
      return Err(format!("ajan bulunamadı: {id}"));
    };
    if status != "fired" {
      return Err("yalnızca işten çıkarılmış ajanlar kalıcı silinebilir".to_string());
    }

    // FK: events.agent_id → agents.id; önce olaylar temizlenir.
    conn
      .execute("DELETE FROM events WHERE agent_id = ?1", params![id])
      .map_err(|e| e.to_string())?;
    conn
      .execute("DELETE FROM agents WHERE id = ?1", params![id])
      .map_err(|e| e.to_string())?;
    Ok(())
  }

  /// Kısmi güncelleme: COALESCE ile None alanlar korunur.
  pub fn update_agent(&self, id: i64, patch: &AgentPatch) -> Result<AgentRecord, String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;

    let updated = conn
      .execute(
        "UPDATE agents SET
           name = COALESCE(?2, name),
           role = COALESCE(?3, role),
           motor = COALESCE(?4, motor),
           model = COALESCE(?5, model),
           status = COALESCE(?6, status),
           avatar_color = COALESCE(?7, avatar_color)
         WHERE id = ?1",
        params![
          id,
          patch.name,
          patch.role,
          patch.motor,
          patch.model,
          patch.status,
          patch.avatar_color,
        ],
      )
      .map_err(|e| e.to_string())?;

    if updated == 0 {
      return Err(format!("ajan bulunamadı: {id}"));
    }
    get_agent(&conn, id)
  }

  /// Tek ajan kaydı.
  pub fn get_agent(&self, id: i64) -> Result<AgentRecord, String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    get_agent(&conn, id)
  }

  /// Tüm ajan kayıtları (fired dahil; filtreleme frontend'de).
  pub fn list_agents(&self) -> Result<Vec<AgentRecord>, String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;

    let mut stmt = conn
      .prepare(
        "SELECT id, name, role, motor, model, status, worktree_path, created_at,
                avatar_color, config_json, hired_at, fired_at
         FROM agents ORDER BY id",
      )
      .map_err(|e| e.to_string())?;

    let rows = stmt
      .query_map([], row_to_agent)
      .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
      .map_err(|e| e.to_string())
  }

  // ---- Görev protokolü (FAZ1 WP-10; docs 13.1–13.2) --------------------------

  /// Yeni görev kaydı (kanban veri omurgası; UI M2'de — şimdilik "Görev Ver" akışı).
  pub fn create_task(
    &self,
    title: &str,
    description: Option<&str>,
    acceptance_criteria: Option<&str>,
    priority: i64,
    budget: Option<f64>,
  ) -> Result<TaskRecord, String> {
    let title = title.trim().to_string();
    if title.is_empty() {
      return Err("görev başlığı boş olamaz".to_string());
    }
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    conn
      .execute(
        "INSERT INTO tasks (title, description, acceptance_criteria, priority, budget, column)
         VALUES (?1, ?2, ?3, ?4, ?5, 'backlog')",
        params![title, description, acceptance_criteria, priority, budget],
      )
      .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    get_task(&conn, id)
  }

  pub fn get_task(&self, id: i64) -> Result<TaskRecord, String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    get_task(&conn, id)
  }

  /// Görev listesi; `agent_id` verilirse yalnızca o ajana atanmışlar.
  pub fn list_tasks(&self, agent_id: Option<i64>) -> Result<Vec<TaskRecord>, String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    let mut stmt = conn
      .prepare(
        "SELECT id, title, description, acceptance_criteria, column, assigned_agent_id,
                priority, budget, spent_cost, worktree_path, created_at, started_at, completed_at
         FROM tasks
         WHERE (?1 IS NULL OR assigned_agent_id = ?1)
         ORDER BY id",
      )
      .map_err(|e| e.to_string())?;
    let rows = stmt
      .query_map(params![agent_id], row_to_task)
      .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
      .map_err(|e| e.to_string())
  }

  /// Görevi ajana atar: `in_progress` + `started_at` + worktree yolu.
  /// Ajan başına tek aktif görev kuralı (M2'de kaldırılabilir).
  pub fn assign_task(
    &self,
    task_id: i64,
    agent_id: i64,
    worktree_path: &str,
  ) -> Result<TaskRecord, String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;

    // Ajanın başka açık görevi var mı?
    let open: i64 = conn
      .query_row(
        "SELECT COUNT(*) FROM tasks
         WHERE assigned_agent_id = ?1 AND column IN ('todo','in_progress','review')",
        params![agent_id],
        |r| r.get(0),
      )
      .map_err(|e| e.to_string())?;
    if open > 0 {
      return Err(format!("ajan {agent_id} zaten bir görev üzerinde çalışıyor"));
    }

    let updated = conn
      .execute(
        "UPDATE tasks SET assigned_agent_id = ?2, column = 'in_progress',
                started_at = COALESCE(started_at, datetime('now')), worktree_path = ?3
         WHERE id = ?1 AND column = 'backlog'",
        params![task_id, agent_id, worktree_path],
      )
      .map_err(|e| e.to_string())?;
    if updated == 0 {
      return Err(format!("görev {task_id} atanamadı (backlog'da değil veya yok)"));
    }
    get_task(&conn, task_id)
  }

  /// Görev kapanışı: `column` (review|failed), maliyet/token sayaçları, `completed_at`.
  pub fn finalize_task(
    &self,
    task_id: i64,
    column: &str,
    cost: f64,
    tokens_in: u64,
    tokens_out: u64,
  ) -> Result<TaskRecord, String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    let updated = conn
      .execute(
        "UPDATE tasks SET column = ?2, spent_cost = ?3,
                spent_tokens_input = spent_tokens_input + ?4,
                spent_tokens_output = spent_tokens_output + ?5,
                completed_at = COALESCE(completed_at, datetime('now'))
         WHERE id = ?1",
        params![task_id, column, cost, tokens_in, tokens_out],
      )
      .map_err(|e| e.to_string())?;
    if updated == 0 {
      return Err(format!("görev bulunamadı: {task_id}"));
    }
    get_task(&conn, task_id)
  }

  // ---- Ayarlar (FAZ1 WP-01; WP-06 repo_path kalıcılığı bunu kullanır) -------

  pub fn setting_get(&self, key: &str) -> Result<Option<String>, String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    conn
      .query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |row| row.get(0),
      )
      .optional()
      .map_err(|e| e.to_string())
  }

  pub fn setting_set(&self, key: &str, value: &str) -> Result<(), String> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| "db lock poisoned".to_string())?;
    conn
      .execute(
        "INSERT INTO settings(key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
      )
      .map_err(|e| e.to_string())?;
    Ok(())
  }

  /// Repo yolu seçimi (docs 5.1 "Proje" çipi; WP-06): canonicalize + `.git`
  /// doğrulaması + worktree kökü reddi + `settings.repo_path`'e yazma.
  pub fn repo_select(&self, path: &str) -> Result<String, String> {
    let raw = PathBuf::from(path);
    let canonical = raw
      .canonicalize()
      .map_err(|e| format!("yol doğrulanamadı: {e}"))?;

    // Worktree kökü reddi: kullanıcı .git/agenthub-worktrees içini seçmesin.
    let canonical_str = canonical.to_string_lossy().to_string();
    if canonical_str.contains(".git/agenthub-worktrees") {
      return Err("worktree kökü seçilemez — ana repoyu seçin".to_string());
    }

    // .git dizin (ana repo) veya dosya (linked worktree) olabilir; en az biri şart.
    let git_path = canonical.join(".git");
    if !git_path.exists() {
      return Err(format!(
        "'{}' bir git deposu değil (.git bulunamadı)",
        canonical.display()
      ));
    }

    self.setting_set("repo_path", &canonical_str)?;
    Ok(canonical_str)
  }

  /// Denetim/olay kaydı (spawn, exit, hire, fire, stopped, ...).
  ///
  /// Frontend ajan id'lerini string taşır ("1", "2", ...). Sayısal olmayan bir
  /// id gelirse FOREIGN KEY ihlali (ve çağıran tarafta sessiz yutulma) yerine
  /// olay `agent_id = NULL` ile kaydedilir — olay asla düşmez.
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
    let agent_pk: Option<i64> = agent_id.and_then(|raw| raw.parse::<i64>().ok());
    conn
      .execute(
        "INSERT INTO events (agent_id, task_id, event_type, payload) VALUES (?1, ?2, ?3, ?4)",
        params![agent_pk, task_id, event_type, payload],
      )
      .map_err(|e| e.to_string())?;
    Ok(())
  }
}

/// Migration'ları `PRAGMA user_version` üzerinden sırayla uygular.
/// Her migration tek bir işlemde koşar; idempotent (IF NOT EXISTS + skip).
fn migrate(conn: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
  let current: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
  for (idx, sql) in MIGRATIONS.iter().enumerate() {
    let version = (idx + 1) as i64;
    if version <= current {
      continue;
    }
    let tx = conn.transaction()?;
    tx.execute_batch(sql)?;
    tx.pragma_update(None, "user_version", version)?;
    tx.commit()?;
  }
  Ok(())
}

/// Ortak satır → AgentRecord eşlemesi (list/get/update paylaşır).
fn row_to_agent(row: &rusqlite::Row<'_>) -> rusqlite::Result<AgentRecord> {
  Ok(AgentRecord {
    id: row.get(0)?,
    name: row.get(1)?,
    role: row.get(2)?,
    motor: row.get(3)?,
    model: row.get(4)?,
    status: row.get(5)?,
    worktree_path: row.get(6)?,
    created_at: row.get(7)?,
    avatar_color: row.get(8)?,
    config_json: row.get(9)?,
    hired_at: row.get(10)?,
    fired_at: row.get(11)?,
  })
}

/// Frontend'e dönen görev kaydı (camelCase).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRecord {
  pub id: i64,
  pub title: String,
  pub description: Option<String>,
  pub acceptance_criteria: Option<String>,
  pub column: String,
  pub assigned_agent_id: Option<i64>,
  pub priority: i64,
  pub budget: Option<f64>,
  pub spent_cost: f64,
  pub worktree_path: Option<String>,
  pub created_at: Option<String>,
  pub started_at: Option<String>,
  pub completed_at: Option<String>,
}

fn row_to_task(row: &rusqlite::Row<'_>) -> rusqlite::Result<TaskRecord> {
  Ok(TaskRecord {
    id: row.get(0)?,
    title: row.get(1)?,
    description: row.get(2)?,
    acceptance_criteria: row.get(3)?,
    column: row.get(4)?,
    assigned_agent_id: row.get(5)?,
    priority: row.get(6)?,
    budget: row.get(7)?,
    spent_cost: row.get(8)?,
    worktree_path: row.get(9)?,
    created_at: row.get(10)?,
    started_at: row.get(11)?,
    completed_at: row.get(12)?,
  })
}

fn get_task(conn: &Connection, id: i64) -> Result<TaskRecord, String> {
  conn
    .query_row(
      "SELECT id, title, description, acceptance_criteria, column, assigned_agent_id,
              priority, budget, spent_cost, worktree_path, created_at, started_at, completed_at
       FROM tasks WHERE id = ?1",
      params![id],
      row_to_task,
    )
    .map_err(|e| {
      if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
        format!("görev bulunamadı: {id}")
      } else {
        e.to_string()
      }
    })
}

fn get_agent(conn: &Connection, id: i64) -> Result<AgentRecord, String> {
  conn
    .query_row(
      "SELECT id, name, role, motor, model, status, worktree_path, created_at,
              avatar_color, config_json, hired_at, fired_at
       FROM agents WHERE id = ?1",
      params![id],
      row_to_agent,
    )
    .map_err(|e| {
      if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
        format!("ajan bulunamadı: {id}")
      } else {
        e.to_string()
      }
    })
}

// ---- Tauri komutları --------------------------------------------------------

#[tauri::command]
pub fn agent_hire(db: State<AppDb>, payload: HirePayload) -> Result<AgentRecord, String> {
  db.hire(&payload)
}

#[tauri::command]
pub fn agent_fire(
  db: State<AppDb>,
  id: i64,
  options: FireOptions,
) -> Result<AgentRecord, String> {
  db.fire(id, &options)
}

#[tauri::command]
pub fn agent_delete(db: State<AppDb>, id: i64) -> Result<(), String> {
  db.delete_agent(id)
}

#[tauri::command]
pub fn agent_update(db: State<AppDb>, id: i64, patch: AgentPatch) -> Result<AgentRecord, String> {
  db.update_agent(id, &patch)
}

#[tauri::command]
pub fn agent_get(db: State<AppDb>, id: i64) -> Result<AgentRecord, String> {
  db.get_agent(id)
}

#[tauri::command]
pub fn agent_list_all(db: State<AppDb>) -> Result<Vec<AgentRecord>, String> {
  db.list_agents()
}

#[tauri::command]
pub fn settings_get(db: State<AppDb>, key: String) -> Result<Option<String>, String> {
  db.setting_get(&key)
}

#[tauri::command]
pub fn settings_set(db: State<AppDb>, key: String, value: String) -> Result<(), String> {
  db.setting_set(&key, &value)
}

#[tauri::command]
pub fn repo_select(db: State<AppDb>, path: String) -> Result<String, String> {
  db.repo_select(&path)
}

#[tauri::command]
pub fn task_create(
  db: State<AppDb>,
  title: String,
  description: Option<String>,
  acceptance_criteria: Option<String>,
  priority: Option<i64>,
  budget: Option<f64>,
) -> Result<TaskRecord, String> {
  db.create_task(&title, description.as_deref(), acceptance_criteria.as_deref(), priority.unwrap_or(3), budget)
}

#[tauri::command]
pub fn task_get(db: State<AppDb>, id: i64) -> Result<TaskRecord, String> {
  db.get_task(id)
}

#[tauri::command]
pub fn task_list(db: State<AppDb>, agent_id: Option<i64>) -> Result<Vec<TaskRecord>, String> {
  db.list_tasks(agent_id)
}

#[tauri::command]
pub fn task_finalize(
  db: State<AppDb>,
  id: i64,
  column: String,
  cost: Option<f64>,
  tokens_in: Option<u64>,
  tokens_out: Option<u64>,
) -> Result<TaskRecord, String> {
  db.finalize_task(id, &column, cost.unwrap_or(0.0), tokens_in.unwrap_or(0), tokens_out.unwrap_or(0))
}

// ---- Unit testler ------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::tempdir;

  fn open_test_db() -> AppDb {
    let dir = tempdir().expect("tempdir");
    AppDb::open(dir.path().to_path_buf()).expect("open db")
  }

  fn hire_payload() -> HirePayload {
    HirePayload {
      name: "Test Ajan".into(),
      role: "Backend Dev".into(),
      motor: "claude".into(),
      model: Some("sonnet".into()),
      effort: Some("medium".into()),
      max_budget_usd: Some(1.5),
      max_turns: Some(10),
      permissions_profile: "standard".into(),
      system_prompt: None,
      avatar_color: Some("#0ea5e9".into()),
      skills: vec!["rust".into()],
      mcp_servers: vec![],
    }
  }

  #[test]
  fn migration_v0_to_v2_applies_and_seeds() {
    let db = open_test_db();
    let conn = db.conn.lock().unwrap();
    let version: i64 = conn
      .query_row("PRAGMA user_version", [], |r| r.get(0))
      .unwrap();
    assert_eq!(version, 2);

    let count: i64 = conn
      .query_row("SELECT COUNT(*) FROM agents", [], |r| r.get(0))
      .unwrap();
    // Starter company seed (yalnızca boş tabloya yazılır).
    assert_eq!(count, 3);
  }

  #[test]
  fn migration_is_idempotent() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_path_buf();
    let _ = AppDb::open(path.clone()).unwrap();
    // İkinci açılışta migration tekrar koşmaz, seed çoğalmaz.
    let db = AppDb::open(path).unwrap();
    let conn = db.conn.lock().unwrap();
    let count: i64 = conn
      .query_row("SELECT COUNT(*) FROM agents", [], |r| r.get(0))
      .unwrap();
    assert_eq!(count, 3);
  }

  #[test]
  fn hire_roundtrip() {
    let db = open_test_db();
    let record = db.hire(&hire_payload()).unwrap();

    assert!(record.id > 0);
    assert_eq!(record.name, "Test Ajan");
    assert_eq!(record.role, "Backend Dev");
    assert_eq!(record.motor, "claude");
    assert_eq!(record.model.as_deref(), Some("sonnet"));
    assert_eq!(record.status, "idle");
    assert_eq!(record.avatar_color.as_deref(), Some("#0ea5e9"));
    assert!(record.hired_at.is_some());

    let cfg = record.config_json.as_deref().unwrap();
    assert!(cfg.contains("permissions_profile"));
    assert!(cfg.contains("max_budget_usd"));
    assert!(cfg.contains("skills"));

    let fetched = db.get_agent(record.id).unwrap();
    assert_eq!(fetched.name, "Test Ajan");
  }

  #[test]
  fn hire_validates_required_fields() {
    let db = open_test_db();
    let mut p = hire_payload();
    p.name = "  ".into();
    assert!(db.hire(&p).is_err());

    let mut p = hire_payload();
    p.motor = "".into();
    assert!(db.hire(&p).is_err());
  }

  #[test]
  fn fire_transitions_and_moves_tasks() {
    let db = open_test_db();
    let agent = db.hire(&hire_payload()).unwrap();

    // Açık görev simüle et (todo).
    {
      let conn = db.conn.lock().unwrap();
      conn
        .execute(
          "INSERT INTO tasks (title, column, assigned_agent_id) VALUES ('açık iş', 'in_progress', ?1)",
          params![agent.id],
        )
        .unwrap();
      conn
        .execute(
          "INSERT INTO tasks (title, column, assigned_agent_id) VALUES ('bitmiş iş', 'done', ?1)",
          params![agent.id],
        )
        .unwrap();
    }

    let fired = db
      .fire(agent.id, &FireOptions::default())
      .expect("fire ok");
    assert_eq!(fired.status, "fired");
    assert!(fired.fired_at.is_some());

    let conn = db.conn.lock().unwrap();
    let open_column: String = conn
      .query_row(
        "SELECT column FROM tasks WHERE title = 'açık iş'",
        [],
        |r| r.get(0),
      )
      .unwrap();
    assert_eq!(open_column, "backlog");
    let done_column: String = conn
      .query_row(
        "SELECT column FROM tasks WHERE title = 'bitmiş iş'",
        [],
        |r| r.get(0),
      )
      .unwrap();
    assert_eq!(done_column, "done");

    let unassigned: Option<i64> = conn
      .query_row(
        "SELECT assigned_agent_id FROM tasks WHERE title = 'açık iş'",
        [],
        |r| r.get(0),
      )
      .unwrap();
    assert!(unassigned.is_none());
  }

  #[test]
  fn fire_twice_fails() {
    let db = open_test_db();
    let agent = db.hire(&hire_payload()).unwrap();
    db.fire(agent.id, &FireOptions::default()).unwrap();
    assert!(db.fire(agent.id, &FireOptions::default()).is_err());
  }

  #[test]
  fn delete_only_fired() {
    let db = open_test_db();
    let agent = db.hire(&hire_payload()).unwrap();

    // Aktif ajan silinemez.
    assert!(db.delete_agent(agent.id).is_err());

    db.fire(agent.id, &FireOptions::default()).unwrap();
    db.delete_agent(agent.id).unwrap();

    assert!(db.get_agent(agent.id).is_err());
    // Olaylar da temizlenmiş olmalı.
    let conn = db.conn.lock().unwrap();
    let events: i64 = conn
      .query_row(
        "SELECT COUNT(*) FROM events WHERE agent_id = ?1",
        params![agent.id],
        |r| r.get(0),
      )
      .unwrap();
    assert_eq!(events, 0);
  }

  #[test]
  fn update_merges_patch() {
    let db = open_test_db();
    let agent = db.hire(&hire_payload()).unwrap();

    let patch = AgentPatch {
      role: Some("CTO".into()),
      model: Some("opus".into()),
      ..Default::default()
    };
    let updated = db.update_agent(agent.id, &patch).unwrap();
    assert_eq!(updated.role, "CTO");
    assert_eq!(updated.model.as_deref(), Some("opus"));
    // Dokunulmayan alanlar korunur.
    assert_eq!(updated.name, "Test Ajan");
    assert_eq!(updated.motor, "claude");
  }

  #[test]
  fn settings_roundtrip() {
    let db = open_test_db();
    assert_eq!(db.setting_get("repo_path").unwrap(), None);

    db.setting_set("repo_path", "/tmp/repo").unwrap();
    assert_eq!(
      db.setting_get("repo_path").unwrap(),
      Some("/tmp/repo".to_string())
    );

    db.setting_set("repo_path", "/tmp/repo2").unwrap();
    assert_eq!(
      db.setting_get("repo_path").unwrap(),
      Some("/tmp/repo2".to_string())
    );
  }

  #[test]
  fn record_event_with_nonnumeric_agent_id_uses_null() {
    let db = open_test_db();
    db.record_event(Some("abc"), None, "spawn", None).unwrap();

    let conn = db.conn.lock().unwrap();
    let agent_id: Option<i64> = conn
      .query_row(
        "SELECT agent_id FROM events WHERE event_type = 'spawn'",
        [],
        |r| r.get(0),
      )
      .unwrap();
    assert!(agent_id.is_none());
  }

  fn init_git_repo(dir: &std::path::Path) {
    let output = std::process::Command::new("git")
      .arg("init")
      .current_dir(dir)
      .output()
      .expect("git init çalışmalı");
    assert!(output.status.success(), "git init başarısız");
  }

  #[test]
  fn repo_select_validates_and_persists() {
    let db = open_test_db();
    let dir = tempdir().unwrap();
    let repo = dir.path().join("repo");
    std::fs::create_dir_all(&repo).unwrap();
    init_git_repo(&repo);

    let result = db.repo_select(repo.to_str().unwrap()).unwrap();
    assert!(result.ends_with("repo"));
    assert_eq!(db.setting_get("repo_path").unwrap(), Some(result));
  }

  #[test]
  fn task_create_roundtrip() {
    let db = open_test_db();
    let task = db
      .create_task("JWT ekle", Some("auth akışı"), Some("testler geçer"), 2, Some(0.5))
      .unwrap();
    assert!(task.id > 0);
    assert_eq!(task.title, "JWT ekle");
    assert_eq!(task.column, "backlog");
    assert_eq!(task.budget, Some(0.5));

    let fetched = db.get_task(task.id).unwrap();
    assert_eq!(fetched.title, "JWT ekle");

    let mut p = "  ".to_string();
    assert!(db.create_task(&p, None, None, 3, None).is_err());
    p = "geçerli".to_string();
    assert!(db.create_task(&p, None, None, 3, None).is_ok());
  }

  #[test]
  fn task_assign_sets_in_progress_and_blocks_second() {
    let db = open_test_db();
    let agent = db.hire(&hire_payload()).unwrap();
    let task = db.create_task("Görev A", None, None, 3, None).unwrap();

    let assigned = db
      .assign_task(task.id, agent.id, "/tmp/wt")
      .expect("atama başarılı");
    assert_eq!(assigned.column, "in_progress");
    assert_eq!(assigned.assigned_agent_id, Some(agent.id));
    assert_eq!(assigned.worktree_path.as_deref(), Some("/tmp/wt"));
    assert!(assigned.started_at.is_some());

    // Ajan başına tek açık görev.
    let task2 = db.create_task("Görev B", None, None, 3, None).unwrap();
    assert!(db.assign_task(task2.id, agent.id, "/tmp/wt2").is_err());

    // Backlog dışındaki göreve atama olmaz.
    let task3 = db.create_task("Görev C", None, None, 3, None).unwrap();
    db.finalize_task(task3.id, "done", 0.0, 0, 0).unwrap();
    assert!(db.assign_task(task3.id, agent.id, "/tmp/wt3").is_err());
  }

  #[test]
  fn task_finalize_updates_cost_and_column() {
    let db = open_test_db();
    let task = db.create_task("Görev", None, None, 3, None).unwrap();
    let updated = db.finalize_task(task.id, "review", 1.25, 500, 120).unwrap();
    assert_eq!(updated.column, "review");
    assert!((updated.spent_cost - 1.25).abs() < 1e-9);
    assert!(updated.completed_at.is_some());
  }

  #[test]
  fn repo_select_rejects_non_git_and_worktree_root() {
    let db = open_test_db();
    let dir = tempdir().unwrap();

    // Git olmayan dizin → Err.
    let plain = dir.path().join("plain");
    std::fs::create_dir_all(&plain).unwrap();
    assert!(db.repo_select(plain.to_str().unwrap()).is_err());

    // Worktree kökü (.git/agenthub-worktrees içi) → Err.
    let repo = dir.path().join("repo2");
    std::fs::create_dir_all(&repo).unwrap();
    init_git_repo(&repo);
    let wts = repo.join(".git/agenthub-worktrees/ornek");
    std::fs::create_dir_all(&wts).unwrap();
    assert!(db.repo_select(wts.to_str().unwrap()).is_err());
  }
}

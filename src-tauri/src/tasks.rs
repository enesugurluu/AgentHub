//! Görev protokolü yardımcıları (AjanOfis docs 13.1–13.2; ADR-6/WP-10).
//!
//! - `write_agent_task`: worktree köküne `AGENT_TASK.md` yazar (görev + kabul
//!   kriterleri + branch + kısıtlar — docs 10.5 deseni).
//! - `decide_completion`: exit'te tamamlanma kararı — parser sinyali > dosya
//!   sinyali (`TASK_BLOCKED.md`/`TASK_COMPLETE.md`) > exit kodu.

use std::fs;
use std::path::{Path, PathBuf};

use crate::db::{AgentRecord, TaskRecord};
use crate::pty::runtime::parser::OutputSignal;

/// Görev dosyası adları (docs 13.2).
pub const TASK_COMPLETE_FILE: &str = "TASK_COMPLETE.md";
pub const TASK_BLOCKED_FILE: &str = "TASK_BLOCKED.md";

/// Worktree köküne `AGENT_TASK.md` yazar; yolunu döndürür.
pub fn write_agent_task(
  worktree: &Path,
  task: &TaskRecord,
  agent: &AgentRecord,
  branch: &str,
) -> Result<PathBuf, String> {
  let budget_line = task
    .budget
    .map(|b| format!("**Bütçe:** ${b} (aşılırsa dur)"))
    .unwrap_or_else(|| "**Bütçe:** tanımlı değil".to_string());
  let acceptance = task
    .acceptance_criteria
    .as_deref()
    .unwrap_or("Belirtilmedi — görev tanımına göre kabul edilebilir çıktı üret.");

  let content = format!(
    "# Görev: {title}\n\n\
     **Ajan:** {agent} · **Branch:** {branch} · **Worktree:** {worktree}\n\
     {budget_line}\n\n\
     ## Görev Tanımı\n{description}\n\n\
     ## Kabul Kriterleri\n{acceptance}\n\n\
     ## Kısıtlar\n\
     - Yalnızca bu worktree içinde çalış.\n\
     - package.json gibi paylaşılan config dosyalarını değiştirme; gerekirse not et.\n\
     - Tamamladığında worktree köküne `{complete}` oluştur ve değiştirdiğin dosyaları listele.\n\
     - Takılırsan `{blocked}` oluştur ve nedeni yaz.\n",
    title = task.title,
    agent = agent.name,
    branch = branch,
    worktree = worktree.display(),
    budget_line = budget_line,
    description = task.description.as_deref().unwrap_or("—"),
    acceptance = acceptance,
    complete = TASK_COMPLETE_FILE,
    blocked = TASK_BLOCKED_FILE,
  );

  let path = worktree.join("AGENT_TASK.md");
  fs::write(&path, content)
    .map_err(|e| format!("AGENT_TASK.md yazılamadı ({}): {e}", path.display()))?;
  Ok(path)
}

/// Görev kapanış kararı (docs 13.2; öncelik sırası):
/// 1. `TASK_BLOCKED.md` varsa → `failed` (neden = dosya içeriği)
/// 2. `TASK_COMPLETE.md` varsa → `review`
/// 3. parser `TaskFailed` sinyali → `failed`
/// 4. parser `TaskCompleted` sinyali → `review`
/// 5. exit kodu 0 → `review`, ≠ 0 → `failed`
pub fn decide_completion(
  worktree: Option<&Path>,
  last_completion: Option<&OutputSignal>,
  exit_code: u32,
) -> (String, String) {
  // (1) dosya sinyali — dosyalar parser'dan daha kesindir.
  if let Some(wt) = worktree {
    if let Ok(content) = fs::read_to_string(wt.join(TASK_BLOCKED_FILE)) {
      let reason = content.trim().chars().take(200).collect();
      return ("failed".to_string(), reason);
    }
    if wt.join(TASK_COMPLETE_FILE).exists() {
      return ("review".to_string(), "TASK_COMPLETE.md".to_string());
    }
  }

  // (2) parser sinyali.
  if let Some(sig) = last_completion {
    match sig {
      crate::pty::runtime::parser::OutputSignal::TaskFailed { reason } => {
        return ("failed".to_string(), reason.clone());
      }
      crate::pty::runtime::parser::OutputSignal::TaskCompleted { summary } => {
        return ("review".to_string(), summary.clone());
      }
      _ => {}
    }
  }

  // (3) exit kodu.
  if exit_code == 0 {
    ("review".to_string(), "exit 0".to_string())
  } else {
    (
      "failed".to_string(),
      format!("exit code {exit_code}"),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::pty::runtime::parser::OutputSignal;
  use tempfile::tempdir;

  fn fake_task() -> TaskRecord {
    TaskRecord {
      id: 1,
      title: "JWT auth ekle".into(),
      description: Some("Kullanıcı girişi için JWT akışı kur.".into()),
      acceptance_criteria: Some("Testler geçiyor.".into()),
      column: "backlog".into(),
      assigned_agent_id: None,
      priority: 2,
      budget: Some(0.5),
      spent_cost: 0.0,
      worktree_path: None,
      created_at: None,
      started_at: None,
      completed_at: None,
    }
  }

  fn fake_agent() -> AgentRecord {
    AgentRecord {
      id: 1,
      name: "Test Ajan".into(),
      role: "Backend Dev".into(),
      motor: "claude".into(),
      model: None,
      status: "idle".into(),
      worktree_path: None,
      created_at: None,
      avatar_color: None,
      config_json: None,
      hired_at: None,
      fired_at: None,
    }
  }

  #[test]
  fn agent_task_md_contains_all_fields() {
    let dir = tempdir().unwrap();
    let task = fake_task();
    let agent = fake_agent();
    let path = write_agent_task(dir.path(), &task, &agent, "agent/test-1").unwrap();
    let content = fs::read_to_string(path).unwrap();
    assert!(content.contains("# Görev: JWT auth ekle"));
    assert!(content.contains("**Ajan:** Test Ajan"));
    assert!(content.contains("**Branch:** agent/test-1"));
    assert!(content.contains("**Bütçe:** $0.5"));
    assert!(content.contains("Kullanıcı girişi için JWT akışı kur."));
    assert!(content.contains(TASK_COMPLETE_FILE));
    assert!(content.contains(TASK_BLOCKED_FILE));
  }

  #[test]
  fn decide_prefers_blocked_file() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join(TASK_BLOCKED_FILE), "bağımlılık yok").unwrap();
    fs::write(dir.path().join(TASK_COMPLETE_FILE), "x").unwrap();
    let (column, reason) = decide_completion(Some(dir.path()), None, 0);
    assert_eq!(column, "failed");
    assert!(reason.contains("bağımlılık yok"));
  }

  #[test]
  fn decide_prefers_complete_file_over_exit_code() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join(TASK_COMPLETE_FILE), "bitti").unwrap();
    let (column, _) = decide_completion(Some(dir.path()), None, 1);
    assert_eq!(column, "review");
  }

  #[test]
  fn decide_parser_signal_beats_exit_code() {
    let (column, reason) = decide_completion(
      None,
      Some(&OutputSignal::TaskFailed {
        reason: "ağ hatası".into(),
      }),
      0,
    );
    assert_eq!(column, "failed");
    assert_eq!(reason, "ağ hatası");

    let (column, _) = decide_completion(
      None,
      Some(&OutputSignal::TaskCompleted {
        summary: "ok".into(),
      }),
      1,
    );
    assert_eq!(column, "review");
  }

  #[test]
  fn decide_falls_back_to_exit_code() {
    let (column, reason) = decide_completion(None, None, 0);
    assert_eq!(column, "review");
    assert_eq!(reason, "exit 0");

    let (column, reason) = decide_completion(None, None, 3);
    assert_eq!(column, "failed");
    assert_eq!(reason, "exit code 3");
  }
}

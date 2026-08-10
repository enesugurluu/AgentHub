//! Claude Code adaptörü (AjanOfis docs Bölüm 7.2 — `claude.rs`).
//!
//! - `detect()`:  `claude --version` ile kurulum + sürüm tespiti
//! - `health()`:  sürüm komutu tekrar çalıştırılır (doctor'a benzer hızlı kapı)
//! - `spawn_cli()`: worktree dizininde interaktif `claude` REPL'i açar
//!
//! Kurulum notu (docs 03): native installer — `curl -fsSL https://claude.ai/install.sh | bash`

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use portable_pty::{CommandBuilder, native_pty_system, PtySize};

use crate::pty::adapters::{
  spawn_pty_isolated, stop_child_tree, DetectResult, EngineAdapter, EngineMetadata, HealthReport,
  ResourceUtil, SpawnOptions, SpawnedPty,
};

/// Claude Code CLI adaptörü.
#[derive(Debug, Default, Clone, Copy)]
pub struct ClaudeAdapter;

/// `claude --version` çıktısını döndürür (kurulu değilse None).
fn detect_version() -> Option<String> {
  let output = Command::new("claude").arg("--version").output().ok()?;
  if !output.status.success() {
    return None;
  }
  let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
  if text.is_empty() {
    None
  } else {
    Some(text)
  }
}

impl ClaudeAdapter {
  fn version(&self) -> Option<String> {
    detect_version()
  }
}

impl EngineAdapter for ClaudeAdapter {
  fn id(&self) -> &str {
    "claude-code"
  }

  fn metadata(&self) -> EngineMetadata {
    EngineMetadata {
      engine_type: "claude".to_string(),
      version: self.version(),
      capabilities: vec![
        "worktree".to_string(),
        "budget".to_string(),
        "effort".to_string(),
        "print".to_string(),
        "interactive".to_string(),
        "doctor".to_string(),
      ],
    }
  }

  fn detect(&self) -> bool {
    detect_version().is_some()
  }

  fn detect_info(&self) -> DetectResult {
    DetectResult {
      detected: self.detect(),
      version: self.version(),
      capabilities: self.metadata().capabilities,
    }
  }

  fn health(&self) -> Result<(), String> {
    let version = detect_version().ok_or_else(|| {
      "claude CLI bulunamadı. Kurulum: curl -fsSL https://claude.ai/install.sh | bash".to_string()
    })?;
    // En az 2.1.90 önerilir (güvenlik düzeltmeleri, docs 03).
    tracing::debug!("claude version: {version}");
    Ok(())
  }

  fn health_report(&self) -> HealthReport {
    let res = self.health();
    // `ok`/`message` önce ayrıştırılır; böylece `res.err()` değeri tüketmeden
    // sonra `res.is_ok()` çağrılmaz (E0382 borrow-of-moved-value).
    let ok = res.is_ok();
    let message = res.err();

    // Sistemsel kaynak kullanımı (sysinfo — AjanOfis docs Bölüm 3.3).
    // CPU yüzdesi iki ölçüm arasındaki delta'dan hesaplanır; tek refresh anlamlı
    // değer üretmez (hep ~0). Kısa bir aralıkla iki kez örneklenir.
    let mut sys = sysinfo::System::new();
    sys.refresh_cpu_usage();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    HealthReport {
      ok,
      message,
      uptime: None,
      resource_utilization: Some(ResourceUtil {
        cpu_percent: Some(sys.global_cpu_usage()),
        memory_bytes: Some(sys.used_memory()),
        process_count: Some(sys.processes().len() as u32),
      }),
      operational_status: if ok {
        "operational".to_string()
      } else {
        "degraded".to_string()
      },
    }
  }

  fn spawn(&self, cmd: CommandBuilder, cols: u16, rows: u16) -> Result<SpawnedPty, String> {
    spawn_pty_isolated(cmd, cols, rows)
  }

  fn spawn_cli(&self, opts: SpawnOptions, cols: u16, rows: u16) -> Result<SpawnedPty, String> {
    let (program, args) = build_claude_command(&opts)?;
    let mut cmd = CommandBuilder::new(program);
    for arg in args {
      cmd.arg(arg);
    }
    cmd.cwd(&opts.workdir);
    for (key, value) in &opts.env {
      cmd.env(key, value);
    }

    // PTY'nin gerçekten açılabildiğini spawn öncesi doğrula (sağlık kapısı).
    let pty_system = native_pty_system();
    let _ = pty_system
      .openpty(PtySize {
        rows: 1,
        cols: 1,
        pixel_width: 0,
        pixel_height: 0,
      })
      .map_err(|e| e.to_string())?;

    spawn_pty_isolated(cmd, cols, rows)
  }

  fn stop(&self, child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String> {
    // Unix'te süreç grubu, Windows'ta Job Object ile ağaç temizliği.
    stop_child_tree(child)
  }
}

/// `SpawnOptions` → claude komut satırı (test-only değil: hem `spawn_cli` hem
/// golden argv testleri buradan beslenir — WP-02/13).
///
/// Flag eşlemesi (docs 7.2):
/// - `non_interactive` → `-p --output-format stream-json` (WP-04 parser girdisi)
/// - `model` → `--model <v>` · `effort` → `--effort <x>` · budget → `--max-budget-usd`
/// - `max_turns` → `--max-turns <n>` · `task_file` → içerik prompt argümanı
pub(crate) fn build_claude_command(opts: &SpawnOptions) -> Result<(String, Vec<String>), String> {
  let mut args: Vec<String> = Vec::new();

  if opts.non_interactive {
    args.push("-p".to_string());
    args.push("--output-format".to_string());
    args.push("stream-json".to_string());
  }
  if let Some(model) = &opts.model {
    args.push("--model".to_string());
    args.push(model.clone());
  }
  if let Some(effort) = &opts.effort {
    args.push("--effort".to_string());
    args.push(effort.as_str().to_string());
  }
  if let Some(budget) = opts.max_budget_usd {
    args.push("--max-budget-usd".to_string());
    args.push(budget.to_string());
  }
  if let Some(turns) = opts.max_turns {
    args.push("--max-turns".to_string());
    args.push(turns.to_string());
  }
  if let Some(task_file) = &opts.task_file {
    let content = std::fs::read_to_string(task_file)
      .map_err(|e| format!("AGENT_TASK.md okunamadı ({}): {e}", task_file.display()))?;
    args.push(content);
  }
  for extra in &opts.args {
    args.push(extra.clone());
  }

  Ok(("claude".to_string(), args))
}

/// Adaptör için yardımcılar (gelecekte codex/gemini aynı deseni kullanır).
#[allow(dead_code)]
pub(crate) fn claude_worktree_path(workdir: &Path) -> PathBuf {
  workdir.join("AGENT_TASK.md")
}

#[allow(dead_code)]
pub(crate) fn claude_timeout() -> Duration {
  Duration::from_secs(30)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::pty::adapters::Effort;

  fn base_opts() -> SpawnOptions {
    SpawnOptions {
      workdir: PathBuf::from("/tmp/wt"),
      env: vec![],
      args: vec![],
      model: None,
      effort: None,
      max_budget_usd: None,
      max_turns: None,
      non_interactive: false,
      task_file: None,
    }
  }

  #[test]
  fn claude_command_interactive_minimal() {
    let (program, args) = build_claude_command(&base_opts()).unwrap();
    assert_eq!(program, "claude");
    assert!(args.is_empty());
  }

  #[test]
  fn claude_command_non_interactive_flags() {
    let mut opts = base_opts();
    opts.non_interactive = true;
    opts.model = Some("sonnet".to_string());
    opts.effort = Some(Effort::High);
    opts.max_budget_usd = Some(2.5);
    opts.max_turns = Some(6);
    let (_, args) = build_claude_command(&opts).unwrap();

    assert!(args.contains(&"-p".to_string()));
    assert!(args.contains(&"stream-json".to_string()));
    assert!(args.windows(2).any(|w| w[0] == "--model" && w[1] == "sonnet"));
    assert!(args.windows(2).any(|w| w[0] == "--effort" && w[1] == "high"));
    assert!(args.windows(2).any(|w| w[0] == "--max-budget-usd" && w[1] == "2.5"));
    assert!(args.windows(2).any(|w| w[0] == "--max-turns" && w[1] == "6"));
  }

  #[test]
  fn claude_command_task_file_reads_content() {
    let dir = tempfile::tempdir().unwrap();
    let task_file = dir.path().join("AGENT_TASK.md");
    std::fs::write(&task_file, "görevi tamamla").unwrap();
    let mut opts = base_opts();
    opts.task_file = Some(task_file);
    let (_, args) = build_claude_command(&opts).unwrap();
    assert!(args.iter().any(|a| a == "görevi tamamla"));
  }

  #[test]
  fn claude_command_missing_task_file_errors() {
    let mut opts = base_opts();
    opts.task_file = Some(PathBuf::from("/nonexistent/AGENT_TASK.md"));
    assert!(build_claude_command(&opts).is_err());
  }

  #[test]
  fn effort_as_str_matches_cli_values() {
    assert_eq!(Effort::Low.as_str(), "low");
    assert_eq!(Effort::Medium.as_str(), "medium");
    assert_eq!(Effort::High.as_str(), "high");
    assert_eq!(Effort::XHigh.as_str(), "xhigh");
    assert_eq!(Effort::Max.as_str(), "max");
  }
}

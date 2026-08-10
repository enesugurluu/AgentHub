//! Claude Code adaptörü (AjanOfis docs Bölüm 7.2 — `claude.rs`).
//!
//! - `detect()`:  `claude --version` ile kurulum + sürüm tespiti
//! - `health()`:  sürüm komutu tekrar çalıştırılır (doctor'a benzer hızlı kapı)
//! - `spawn_cli()`: worktree dizininde interaktif `claude` REPL'i açar
//!
//! Kurulum notu (docs 03): native installer — `curl -fsSL https://claude.ai/install.sh | bash`

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use portable_pty::{CommandBuilder, native_pty_system, PtySize};

use crate::pty::adapters::{
  spawn_pty_isolated, CliSpawnOptions, DetectResult, EngineAdapter, EngineMetadata, HealthReport,
  ResourceUtil, SpawnedPty,
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
    let mut sys = sysinfo::System::new();
    sys.refresh_cpu_usage();
    sys.refresh_memory();

    HealthReport {
      ok,
      message,
      uptime: None,
      resource_utilization: Some(ResourceUtil {
        cpu_percent: Some(sys.global_cpu_usage()),
        memory_bytes: Some(sys.used_memory()),
        process_count: None,
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

  fn spawn_cli(&self, opts: CliSpawnOptions, cols: u16, rows: u16) -> Result<SpawnedPty, String> {
    let mut cmd = CommandBuilder::new("claude");
    for arg in &opts.args {
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
    child.kill().map_err(|e| e.to_string())?;
    let _ = child.wait();
    Ok(())
  }
}

/// Adaptör için yardımcılar (gelecekte codex/gemini aynı deseni kullanır).
#[allow(dead_code)]
pub(crate) fn claude_worktree_path(workdir: &PathBuf) -> PathBuf {
  workdir.join("AGENT_TASK.md")
}

#[allow(dead_code)]
pub(crate) fn claude_timeout() -> Duration {
  Duration::from_secs(30)
}

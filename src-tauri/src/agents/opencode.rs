//! OpenCode adaptörü (AjanOfis docs Bölüm 7.2 — `opencode.rs`).
//!
//! - `detect()`: `opencode --version` ile kurulum + sürüm tespiti
//! - `health()`: sürüm kapısı
//! - `spawn_cli()`: non-interactive `opencode run` (JSONL event çıktısı — WP-04 parser)
//!
//! Kurulum: `curl -fsSL https://opencode.ai/install | bash`
//! Not: budget/turn/effort flag'leri CLI-native değil → capability ilan edilmez (WP-13);
//! cost bilgisi JSONL `cost` event'inden parse edilir (WP-04).

use crate::agents::{command_from, detect_binary_version, read_task_content};
use crate::pty::adapters::{
  spawn_pty_isolated, stop_child_tree, DetectResult, EngineAdapter, EngineMetadata, SpawnOptions,
  SpawnedPty,
};

/// OpenCode adaptörü.
#[derive(Debug, Default, Clone, Copy)]
pub struct OpencodeAdapter;

impl EngineAdapter for OpencodeAdapter {
  fn id(&self) -> &str {
    "opencode-adapter"
  }

  fn metadata(&self) -> EngineMetadata {
    EngineMetadata {
      engine_type: "opencode".to_string(),
      version: detect_binary_version("opencode", &["--version"]),
      capabilities: vec![
        "print".to_string(),
        "worktree".to_string(),
        "model".to_string(),
        "json".to_string(),
      ],
    }
  }

  fn detect(&self) -> bool {
    detect_binary_version("opencode", &["--version"]).is_some()
  }

  fn detect_info(&self) -> DetectResult {
    DetectResult {
      detected: self.detect(),
      version: self.metadata().version,
      capabilities: self.metadata().capabilities,
      install_hint: Some("curl -fsSL https://opencode.ai/install | bash".to_string()),
    }
  }

  fn health(&self) -> Result<(), String> {
    match detect_binary_version("opencode", &["--version"]) {
      Some(version) => {
        tracing::debug!("opencode version: {version}");
        Ok(())
      }
      None => Err(
        "opencode bulunamadı. Kurulum: curl -fsSL https://opencode.ai/install | bash".to_string(),
      ),
    }
  }

  fn spawn_cli(&self, opts: SpawnOptions, cols: u16, rows: u16) -> Result<SpawnedPty, String> {
    let (program, args) = build_opencode_command(&opts)?;
    spawn_pty_isolated(command_from(program, args, &opts), cols, rows)
  }

  fn stop(&self, child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String> {
    stop_child_tree(child)
  }

  fn install_command(&self) -> Option<Vec<String>> {
    Some(vec![
      "bash".to_string(),
      "-lc".to_string(),
      "curl -fsSL https://opencode.ai/install | bash".to_string(),
    ])
  }
}

/// `SpawnOptions` → opencode komut satırı (golden argv testleri bunu doğrular).
///
/// Flag eşlemesi (uygulama sırasında `opencode --help` ile doğrulanacak):
/// - `non_interactive` → `run`
/// - `model` → `--model <v>` · `task_file` → içerik prompt argümanı
pub(crate) fn build_opencode_command(opts: &SpawnOptions) -> Result<(String, Vec<String>), String> {
  let mut args: Vec<String> = Vec::new();

  if opts.non_interactive {
    args.push("run".to_string());
  }
  if let Some(model) = &opts.model {
    args.push("--model".to_string());
    args.push(model.clone());
  }
  if let Some(content) = read_task_content(&opts.task_file)? {
    args.push(content);
  }
  for extra in &opts.args {
    args.push(extra.clone());
  }

  Ok(("opencode".to_string(), args))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::agents::test_util::{base_opts, with_fake_binary};
  use std::path::PathBuf;

  #[test]
  fn detect_with_fake_binary() {
    // Mock matrisi yalnızca Unix (PATH'e sahte binary); Windows'ta boş test.
    #[cfg(unix)]
    {
      with_fake_binary("opencode", "echo 0.9.0", || {
        let adapter = OpencodeAdapter;
        assert!(adapter.detect());
        assert_eq!(adapter.metadata().version.as_deref(), Some("0.9.0"));
      });
    }
  }

  #[test]
  fn opencode_command_minimal() {
    let (program, args) = build_opencode_command(&base_opts()).unwrap();
    assert_eq!(program, "opencode");
    assert!(args.is_empty());
  }

  #[test]
  fn opencode_command_non_interactive_model_task() {
    let dir = tempfile::tempdir().unwrap();
    let task_file = dir.path().join("AGENT_TASK.md");
    std::fs::write(&task_file, "opencode görevi").unwrap();

    let mut opts = base_opts();
    opts.non_interactive = true;
    opts.model = Some("claude-sonnet".to_string());
    opts.task_file = Some(task_file);

    let (_, args) = build_opencode_command(&opts).unwrap();
    assert_eq!(args[0], "run");
    assert!(args.windows(2).any(|w| w[0] == "--model" && w[1] == "claude-sonnet"));
    assert!(args.iter().any(|a| a == "opencode görevi"));
  }

  #[test]
  fn opencode_missing_task_file_errors() {
    let mut opts = base_opts();
    opts.task_file = Some(PathBuf::from("/nonexistent/AGENT_TASK.md"));
    assert!(build_opencode_command(&opts).is_err());
  }
}

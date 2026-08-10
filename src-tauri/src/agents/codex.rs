//! OpenAI Codex CLI adaptörü (AjanOfis docs Bölüm 7.2 — `codex.rs`).
//!
//! - `detect()`: `codex --version` ile kurulum + sürüm tespiti
//! - `health()`: sürüm kapısı
//! - `spawn_cli()`: non-interactive `codex exec` (worktree cwd'sinde)
//!
//! Kurulum: `npm i -g @openai/codex`
//! Not: budget/turn/effort flag'leri CLI-native değil → capability ilan edilmez;
//! `SpawnOptions`'taki ilgili alanlar yok sayılır (WP-13).

use crate::agents::{command_from, detect_binary_version, read_task_content};
use crate::pty::adapters::{
  spawn_pty_isolated, stop_child_tree, DetectResult, EngineAdapter, EngineMetadata, SpawnOptions,
  SpawnedPty,
};

/// OpenAI Codex CLI adaptörü.
#[derive(Debug, Default, Clone, Copy)]
pub struct CodexAdapter;

impl EngineAdapter for CodexAdapter {
  fn id(&self) -> &str {
    "codex-cli"
  }

  fn metadata(&self) -> EngineMetadata {
    EngineMetadata {
      engine_type: "codex".to_string(),
      version: detect_binary_version("codex", &["--version"]),
      capabilities: vec![
        "print".to_string(),
        "worktree".to_string(),
        "model".to_string(),
      ],
    }
  }

  fn detect(&self) -> bool {
    detect_binary_version("codex", &["--version"]).is_some()
  }

  fn detect_info(&self) -> DetectResult {
    DetectResult {
      detected: self.detect(),
      version: self.metadata().version,
      capabilities: self.metadata().capabilities,
      install_hint: Some("npm i -g @openai/codex".to_string()),
    }
  }

  fn health(&self) -> Result<(), String> {
    match detect_binary_version("codex", &["--version"]) {
      Some(version) => {
        tracing::debug!("codex version: {version}");
        Ok(())
      }
      None => Err("codex CLI bulunamadı. Kurulum: npm i -g @openai/codex".to_string()),
    }
  }

  fn spawn_cli(&self, opts: SpawnOptions, cols: u16, rows: u16) -> Result<SpawnedPty, String> {
    let (program, args) = build_codex_command(&opts)?;
    spawn_pty_isolated(command_from(program, args, &opts), cols, rows)
  }

  fn stop(&self, child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String> {
    stop_child_tree(child)
  }

  fn install_command(&self) -> Option<Vec<String>> {
    Some(vec![
      "npm".to_string(),
      "i".to_string(),
      "-g".to_string(),
      "@openai/codex".to_string(),
    ])
  }
}

/// `SpawnOptions` → codex komut satırı (golden argv testleri bunu doğrular).
///
/// Flag eşlemesi (uygulama sırasında `codex --help` ile doğrulanacak):
/// - `non_interactive` → `exec`
/// - `model` → `--model <v>` · `task_file` → içerik prompt argümanı
/// - budget/turn/effort desteklenmez (capability yok) → yok sayılır
pub(crate) fn build_codex_command(opts: &SpawnOptions) -> Result<(String, Vec<String>), String> {
  let mut args: Vec<String> = Vec::new();

  if opts.non_interactive {
    args.push("exec".to_string());
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

  Ok(("codex".to_string(), args))
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
      with_fake_binary("codex", "echo 1.2.3", || {
        let adapter = CodexAdapter;
        assert!(adapter.detect());
        assert_eq!(adapter.metadata().version.as_deref(), Some("1.2.3"));
        assert_eq!(adapter.health(), Ok(()));
      });
    }
  }

  #[test]
  fn codex_command_minimal() {
    let (program, args) = build_codex_command(&base_opts()).unwrap();
    assert_eq!(program, "codex");
    assert!(args.is_empty());
  }

  #[test]
  fn codex_command_non_interactive_model_task() {
    let dir = tempfile::tempdir().unwrap();
    let task_file = dir.path().join("AGENT_TASK.md");
    std::fs::write(&task_file, "codex görevi").unwrap();

    let mut opts = base_opts();
    opts.non_interactive = true;
    opts.model = Some("gpt-5".to_string());
    opts.task_file = Some(task_file);

    let (_, args) = build_codex_command(&opts).unwrap();
    assert_eq!(args[0], "exec");
    assert!(args.windows(2).any(|w| w[0] == "--model" && w[1] == "gpt-5"));
    assert!(args.iter().any(|a| a == "codex görevi"));
  }

  #[test]
  fn codex_missing_task_file_errors() {
    let mut opts = base_opts();
    opts.task_file = Some(PathBuf::from("/nonexistent/AGENT_TASK.md"));
    assert!(build_codex_command(&opts).is_err());
  }
}

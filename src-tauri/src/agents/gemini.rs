//! Google Gemini CLI adaptörü (AjanOfis docs Bölüm 7.2 — `gemini.rs`).
//!
//! - `detect()`: `gemini --version` ile kurulum + sürüm tespiti
//! - `health()`: sürüm kapısı
//! - `spawn_cli()`: non-interactive `gemini run -p` (worktree cwd'sinde)
//!
//! Kurulum: `npm i -g @google/gemini-cli`
//! Not: budget/turn/effort flag'leri CLI-native değil → capability ilan edilmez (WP-13).

use crate::agents::{command_from, detect_binary_version, read_task_content};
use crate::pty::adapters::{
  spawn_pty_isolated, stop_child_tree, DetectResult, EngineAdapter, EngineMetadata, SpawnOptions,
  SpawnedPty,
};

/// Google Gemini CLI adaptörü.
#[derive(Debug, Default, Clone, Copy)]
pub struct GeminiAdapter;

impl EngineAdapter for GeminiAdapter {
  fn id(&self) -> &str {
    "gemini-cli"
  }

  fn metadata(&self) -> EngineMetadata {
    EngineMetadata {
      engine_type: "gemini".to_string(),
      version: detect_binary_version("gemini", &["--version"]),
      capabilities: vec![
        "print".to_string(),
        "worktree".to_string(),
        "model".to_string(),
      ],
    }
  }

  fn detect(&self) -> bool {
    detect_binary_version("gemini", &["--version"]).is_some()
  }

  fn detect_info(&self) -> DetectResult {
    DetectResult {
      detected: self.detect(),
      version: self.metadata().version,
      capabilities: self.metadata().capabilities,
      install_hint: Some("npm i -g @google/gemini-cli".to_string()),
    }
  }

  fn health(&self) -> Result<(), String> {
    match detect_binary_version("gemini", &["--version"]) {
      Some(version) => {
        tracing::debug!("gemini version: {version}");
        Ok(())
      }
      None => Err(
        "gemini CLI bulunamadı. Kurulum: npm i -g @google/gemini-cli".to_string(),
      ),
    }
  }

  fn spawn_cli(&self, opts: SpawnOptions, cols: u16, rows: u16) -> Result<SpawnedPty, String> {
    let (program, args) = build_gemini_command(&opts)?;
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
      "@google/gemini-cli".to_string(),
    ])
  }
}

/// `SpawnOptions` → gemini komut satırı (golden argv testleri bunu doğrular).
///
/// Flag eşlemesi (uygulama sırasında `gemini --help` ile doğrulanacak):
/// - `non_interactive` → `run -p`
/// - `model` → `--model <v>` · `task_file` → içerik prompt argümanı
pub(crate) fn build_gemini_command(opts: &SpawnOptions) -> Result<(String, Vec<String>), String> {
  let mut args: Vec<String> = Vec::new();

  if opts.non_interactive {
    args.push("run".to_string());
    args.push("-p".to_string());
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

  Ok(("gemini".to_string(), args))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::agents::test_util::{base_opts, with_fake_binary};
  use std::path::PathBuf;

  #[test]
  fn detect_with_fake_binary() {
    with_fake_binary("gemini", "echo 2.0.0", || {
      let adapter = GeminiAdapter;
      assert!(adapter.detect());
      assert_eq!(adapter.metadata().version.as_deref(), Some("2.0.0"));
    });
  }

  #[test]
  fn gemini_command_minimal() {
    let (program, args) = build_gemini_command(&base_opts()).unwrap();
    assert_eq!(program, "gemini");
    assert!(args.is_empty());
  }

  #[test]
  fn gemini_command_non_interactive_model() {
    let mut opts = base_opts();
    opts.non_interactive = true;
    opts.model = Some("gemini-2.5-pro".to_string());
    let (_, args) = build_gemini_command(&opts).unwrap();
    assert_eq!(args[0], "run");
    assert_eq!(args[1], "-p");
    assert!(args.windows(2).any(|w| w[0] == "--model" && w[1] == "gemini-2.5-pro"));
  }

  #[test]
  fn gemini_missing_task_file_errors() {
    let mut opts = base_opts();
    opts.task_file = Some(PathBuf::from("/nonexistent/AGENT_TASK.md"));
    assert!(build_gemini_command(&opts).is_err());
  }
}

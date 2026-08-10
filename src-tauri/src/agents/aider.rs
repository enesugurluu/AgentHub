//! Aider adaptörü (AjanOfis docs Bölüm 7.2 — `aider.rs`).
//!
//! - `detect()`: `aider --version` ile kurulum + sürüm tespiti
//! - `health()`: sürüm kapısı
//! - `spawn_cli()`: `aider --message "<task>"` (git-aware, tek seferlik görev)
//!
//! Kurulum: `pip install aider-install`
//! Not: budget/turn/effort flag'leri CLI-native değil → capability ilan edilmez (WP-13);
//! `--no-auto-commits` ve `--architect` opsiyonel bayraklardır.

use crate::agents::{command_from, detect_binary_version, read_task_content};
use crate::pty::adapters::{
  spawn_pty_isolated, stop_child_tree, DetectResult, EngineAdapter, EngineMetadata, SpawnOptions,
  SpawnedPty,
};

/// Aider adaptörü.
#[derive(Debug, Default, Clone, Copy)]
pub struct AiderAdapter;

impl EngineAdapter for AiderAdapter {
  fn id(&self) -> &str {
    "aider-adapter"
  }

  fn metadata(&self) -> EngineMetadata {
    EngineMetadata {
      engine_type: "aider".to_string(),
      version: detect_binary_version("aider", &["--version"]),
      capabilities: vec![
        "print".to_string(),
        "worktree".to_string(),
        "model".to_string(),
      ],
    }
  }

  fn detect(&self) -> bool {
    detect_binary_version("aider", &["--version"]).is_some()
  }

  fn detect_info(&self) -> DetectResult {
    DetectResult {
      detected: self.detect(),
      version: self.metadata().version,
      capabilities: self.metadata().capabilities,
      install_hint: Some("pip install aider-install".to_string()),
    }
  }

  fn health(&self) -> Result<(), String> {
    match detect_binary_version("aider", &["--version"]) {
      Some(version) => {
        tracing::debug!("aider version: {version}");
        Ok(())
      }
      None => Err("aider bulunamadı. Kurulum: pip install aider-install".to_string()),
    }
  }

  fn spawn_cli(&self, opts: SpawnOptions, cols: u16, rows: u16) -> Result<SpawnedPty, String> {
    let (program, args) = build_aider_command(&opts)?;
    spawn_pty_isolated(command_from(program, args, &opts), cols, rows)
  }

  fn stop(&self, child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String> {
    stop_child_tree(child)
  }

  fn install_command(&self) -> Option<Vec<String>> {
    Some(vec![
      "pip".to_string(),
      "install".to_string(),
      "aider-install".to_string(),
    ])
  }
}

/// `SpawnOptions` → aider komut satırı (golden argv testleri bunu doğrular).
///
/// Flag eşlemesi (uygulama sırasında `aider --help` ile doğrulanacak):
/// - `non_interactive` veya `task_file` → `--message <içerik>` (aider git-aware tek seferlik)
/// - `model` → `--model <v>` · `args` içinden `--no-auto-commits`, `--architect` geçebilir
pub(crate) fn build_aider_command(opts: &SpawnOptions) -> Result<(String, Vec<String>), String> {
  let mut args: Vec<String> = Vec::new();

  // Aider görev modunda --message ile çalışır; task_file içeriği prompt olarak geçer.
  // task_file yoksa interaktif oturum başlatılır (flag'siz — claude REPL'i gibi).
  if let Some(content) = read_task_content(&opts.task_file)? {
    args.push("--message".to_string());
    args.push(content);
  } else if opts.non_interactive {
    return Err(
      "aider için --message gereklidir (non_interactive modda task_file zorunlu)".to_string(),
    );
  }

  if let Some(model) = &opts.model {
    args.push("--model".to_string());
    args.push(model.clone());
  }
  for extra in &opts.args {
    args.push(extra.clone());
  }

  Ok(("aider".to_string(), args))
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
      with_fake_binary("aider", "echo 0.87.0", || {
        let adapter = AiderAdapter;
        assert!(adapter.detect());
        assert_eq!(adapter.metadata().version.as_deref(), Some("0.87.0"));
      });
    }
  }

  #[test]
  fn aider_interactive_without_task_file() {
    // task_file yok + non_interactive değil → interactive oturum (hata değil).
    let (program, args) = build_aider_command(&base_opts()).unwrap();
    assert_eq!(program, "aider");
    assert!(args.is_empty());
  }

  #[test]
  fn aider_non_interactive_without_task_file_errors() {
    // non_interactive + task_file yok → açıklayıcı hata (boş --message üretme).
    let mut opts = base_opts();
    opts.non_interactive = true;
    assert!(build_aider_command(&opts).is_err());
  }

  #[test]
  fn aider_command_with_task_file() {
    let dir = tempfile::tempdir().unwrap();
    let task_file = dir.path().join("AGENT_TASK.md");
    std::fs::write(&task_file, "aider görevi").unwrap();

    let mut opts = base_opts();
    opts.task_file = Some(task_file);
    opts.model = Some("sonnet".to_string());
    opts.args = vec!["--no-auto-commits".to_string()];

    let (program, args) = build_aider_command(&opts).unwrap();
    assert_eq!(program, "aider");
    assert!(args.windows(2).any(|w| w[0] == "--message" && w[1] == "aider görevi"));
    assert!(args.windows(2).any(|w| w[0] == "--model" && w[1] == "sonnet"));
    assert!(args.contains(&"--no-auto-commits".to_string()));
  }

  #[test]
  fn aider_missing_task_file_errors() {
    let mut opts = base_opts();
    opts.task_file = Some(PathBuf::from("/nonexistent/AGENT_TASK.md"));
    assert!(build_aider_command(&opts).is_err());
  }
}

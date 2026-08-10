//! CLI ajan adaptörleri (AjanOfis docs Bölüm 7).
//!
//! Her AI CLI (`claude`, `codex`, `gemini`, `opencode`, `aider`) aynı
//! `EngineAdapter` arayüzü arkasında soyutlanır. FAZ0: Claude Code; FAZ1 (WP-03):
//! Codex, Gemini, OpenCode, Aider — hepsi aynı kalıpta eklenir.
//!
//! Ortak yardımcılar: `detect_binary_version` (kurulum+sürüm tespiti),
//! `command_from` (SpawnOptions → CommandBuilder), `read_task_content`.

pub mod aider;
pub mod claude;
pub mod codex;
pub mod gemini;
pub mod opencode;

pub use aider::AiderAdapter;
pub use claude::ClaudeAdapter;
pub use codex::CodexAdapter;
pub use gemini::GeminiAdapter;
pub use opencode::OpencodeAdapter;

use std::path::PathBuf;

use portable_pty::CommandBuilder;

use crate::pty::adapters::SpawnOptions;

/// `--version` benzeri bir komutla CLI kurulumunu ve sürümünü tespit eder.
/// Çıktı başarılı değilse veya boşsa `None` (kurulu değil kabul edilir).
pub(crate) fn detect_binary_version(binary: &str, args: &[&str]) -> Option<String> {
  let output = std::process::Command::new(binary).args(args).output().ok()?;
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

/// `(program, args)` + `SpawnOptions` → `CommandBuilder` (cwd + env dahil).
/// Tüm CLI adaptörleri spawn'da bunu kullanır; golden argv testleri yalnızca
/// `build_<engine>_command` fonksiyonlarını test eder (PTY açmadan).
pub(crate) fn command_from(program: String, args: Vec<String>, opts: &SpawnOptions) -> CommandBuilder {
  let mut cmd = CommandBuilder::new(program);
  for arg in args {
    cmd.arg(arg);
  }
  cmd.cwd(&opts.workdir);
  for (key, value) in &opts.env {
    cmd.env(key, value);
  }
  cmd
}

/// `AGENT_TASK.md` içeriğini okur (WP-10; prompt argümanı olarak iletilir).
pub(crate) fn read_task_content(task_file: &Option<PathBuf>) -> Result<Option<String>, String> {
  match task_file {
    Some(path) => Ok(Some(
      std::fs::read_to_string(path)
        .map_err(|e| format!("görev dosyası okunamadı ({}): {e}", path.display()))?,
    )),
    None => Ok(None),
  }
}

#[cfg(test)]
pub(crate) mod test_util {
  use std::path::PathBuf;

  use crate::pty::adapters::SpawnOptions;

  #[cfg(unix)]
  use std::sync::Mutex;

  /// PATH değişimi testler arası yarışmasın diye küresel kilit (yalnız Unix).
  #[cfg(unix)]
  static PATH_LOCK: Mutex<()> = Mutex::new(());

  /// `name` adında sahte bir CLI binary'si kurar (`script` çalıştırır),
  /// PATH'in başına ekler ve `test`'i çalıştırır; sonra PATH'i geri alır.
  ///
  /// Mock matrisi yalnızca Unix'te gerçektir; Windows'ta no-op (Windows'ta gerçek
  /// CLI varlığı elle doğrulanır). Fonksiyon her platformda derlensin diye cfg
  /// gövde içindedir.
  pub(crate) fn with_fake_binary(name: &str, script: &str, test: impl FnOnce()) {
    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;

      let _guard = PATH_LOCK.lock().unwrap();
      let dir = tempfile::tempdir().expect("tempdir");
      let bin = dir.path().join(name);
      std::fs::write(&bin, format!("#!/bin/sh\n{script}\n")).expect("write fake binary");
      std::fs::set_permissions(&bin, std::fs::Permissions::from_mode(0o755)).expect("chmod +x");

      let prev = std::env::var("PATH").unwrap_or_default();
      std::env::set_var("PATH", format!("{}:{}", dir.path().display(), prev));
      test();
      std::env::set_var("PATH", prev);
    }
    #[cfg(not(unix))]
    {
      let _ = (name, script);
      // Mock matrisi yalnızca Unix; Windows'ta no-op.
    }
  }

  /// Golden argv testleri için boş SpawnOptions.
  pub(crate) fn base_opts() -> SpawnOptions {
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
}

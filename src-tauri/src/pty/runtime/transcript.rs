//! JSONL oturum kaydı (AjanOfis docs Bölüm 12.2; ADR-8/WP-11).
//!
//! Her ajan oturumu `~/.agentcompany/logs/<agent>/<name>-<epoch>.jsonl` dosyasına
//! yazılır: output (ham bayt → text), input, progress, exit, session_buffer.
//!
//! NOT (2026-08-10): `chrono`/`dirs` crates'i bu ortamda lock'a eklenemediği için
//! (crates.io erişilemez, CI `--locked`) zaman damgası **epoch saniye** (SystemTime),
//! ev dizini **HOME/USERPROFILE** env'inden çözülür. İleride crates eklenirse
//! `chrono` ile RFC3339'a geçilebilir.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Epoch saniye (chrono yokken — docs 12.2 `ts` alanı için).
pub fn epoch_seconds() -> i64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map(|d| d.as_secs() as i64)
    .unwrap_or(0)
}

/// Ev dizini: `~/.agentcompany/logs` (docs 12.2); HOME (Unix) / USERPROFILE (Win).
pub fn agentcompany_logs_dir() -> Option<PathBuf> {
  let home = std::env::var("HOME")
    .or_else(|_| std::env::var("USERPROFILE"))
    .ok()?;
  Some(PathBuf::from(home).join(".agentcompany").join("logs"))
}

/// Dosya adı için güvenli slug (path traversal önlemi).
pub fn slugify(name: &str) -> String {
  name.chars()
    .map(|c| {
      if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
        c
      } else {
        '-'
      }
    })
    .collect::<String>()
    .trim_matches('-')
    .to_string()
}

/// Yeni oturum dosyası yolu oluşturur (dizin de kurulur). `name` boşsa "manual".
pub fn open_transcript(base_dir: &Path, slug: &str, name: &str) -> Result<PathBuf, String> {
  let safe_slug = slugify(slug);
  let safe_name = if name.trim().is_empty() {
    "manual".to_string()
  } else {
    slugify(name)
  };
  let dir = base_dir.join(&safe_slug);
  fs::create_dir_all(&dir).map_err(|e| format!("log dizini oluşturulamadı: {e}"))?;
  Ok(dir.join(format!("{safe_name}-{}.jsonl", epoch_seconds())))
}

/// JSONL dosyasına tek satır ekler (append).
pub fn append_transcript_entry(path: &Path, entry: serde_json::Value) -> Result<(), String> {
  let mut line = serde_json::to_string(&entry).map_err(|e| e.to_string())?;
  line.push('\n');
  let mut file = fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(path)
    .map_err(|e| format!("transcript açılamadı ({}): {e}", path.display()))?;
  file.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
  Ok(())
}

/// Ham bayt → transcript `output` satırı (utf8-lossy; JSON escape sayesinde
/// çok baytlı karakterler tek satırda bozulmadan saklanır).
pub fn output_entry(bytes: &[u8]) -> serde_json::Value {
  serde_json::json!({
    "ts": epoch_seconds(),
    "type": "output",
    "content": String::from_utf8_lossy(bytes),
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::tempdir;

  #[test]
  fn slugify_sanitizes_path_traversal() {
    assert_eq!(slugify("../../x"), "x");
    assert_eq!(slugify("Backend Dev"), "Backend-Dev");
    assert_eq!(slugify("Ada"), "Ada");
  }

  #[test]
  fn open_and_append_jsonl() {
    let dir = tempdir().unwrap();
    let path = open_transcript(dir.path(), "test-ajan", "manual").unwrap();
    assert!(path.to_string_lossy().contains("test-ajan"));
    assert!(path.to_string_lossy().contains("manual-"));

    append_transcript_entry(&path, output_entry(b"selam \xF0\x9F\x98\x80\n")).unwrap();
    append_transcript_entry(
      &path,
      serde_json::json!({ "ts": 0, "type": "exit", "code": 0 }),
    )
    .unwrap();

    let content = fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 2);
    // Her satır geçerli JSON ve tek satır.
    for line in &lines {
      let v: serde_json::Value = serde_json::from_str(line).unwrap();
      assert!(v.get("ts").is_some());
    }
    // Çok baytlı emoji bozulmadan saklanır.
    assert!(content.contains("selam 😀"));
  }

  #[test]
  fn output_entry_contains_type_and_content() {
    let entry = output_entry(b"abc");
    assert_eq!(entry["type"], "output");
    assert_eq!(entry["content"], "abc");
  }
}

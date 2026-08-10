use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::time::Duration;

use super::{
  spawn_pty_isolated, DetectResult, EngineAdapter, EngineMetadata, HealthReport, SpawnedPty,
};

/// Built-in adapter backed by the `portable-pty` crate's native PTY implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct PortablePtyAdapter;

impl EngineAdapter for PortablePtyAdapter {
  fn id(&self) -> &str {
    "portable-pty-native"
  }

  fn metadata(&self) -> EngineMetadata {
    EngineMetadata {
      engine_type: "pty".to_string(),
      version: Some("1.0.0".to_string()),
      capabilities: vec!["native-pty".to_string()],
    }
  }

  fn detect(&self) -> bool {
    // `portable-pty` compiles on all our supported targets. If PTY APIs are missing,
    // `health()` will surface it.
    true
  }

  fn detect_info(&self) -> DetectResult {
    DetectResult {
      detected: self.detect(),
      version: self.metadata().version,
      capabilities: self.metadata().capabilities,
    }
  }

  fn health(&self) -> Result<(), String> {
    let pty_system = native_pty_system();
    let _ = pty_system
      .openpty(PtySize {
        rows: 1,
        cols: 1,
        pixel_width: 0,
        pixel_height: 0,
      })
      .map_err(|e| e.to_string())?;
    Ok(())
  }

  fn health_report(&self) -> HealthReport {
    let res = self.health();
    HealthReport {
      ok: res.is_ok(),
      message: res.err(),
      uptime: Some(Duration::from_secs(0)),
      resource_utilization: None,
      operational_status: "operational".to_string(),
    }
  }

  fn spawn(&self, cmd: CommandBuilder, cols: u16, rows: u16) -> Result<SpawnedPty, String> {
    spawn_pty_isolated(cmd, cols, rows)
  }

  fn stop(&self, child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String> {
    child.kill().map_err(|e| e.to_string())?;
    let _ = child.wait();
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::time::Duration;

  #[test]
  fn test_metadata() {
    let adapter = PortablePtyAdapter;
    let meta = adapter.metadata();
    assert_eq!(meta.engine_type, "pty");
    assert_eq!(meta.version, Some("1.0.0".to_string()));
    assert_eq!(meta.capabilities, vec!["native-pty".to_string()]);
  }

  #[test]
  fn test_detect() {
    let adapter = PortablePtyAdapter;
    assert!(adapter.detect());
  }

  #[test]
  fn test_detect_info() {
    let adapter = PortablePtyAdapter;
    let info = adapter.detect_info();
    assert!(info.detected);
    assert_eq!(info.version, Some("1.0.0".to_string()));
    assert_eq!(info.capabilities, vec!["native-pty".to_string()]);
  }

  #[test]
  fn test_health() {
    let adapter = PortablePtyAdapter;
    assert!(adapter.health().is_ok());
  }

  #[test]
  fn test_health_report() {
    let adapter = PortablePtyAdapter;
    let report = adapter.health_report();
    assert!(report.ok);
    assert_eq!(report.message, None);
    assert_eq!(report.uptime, Some(Duration::from_secs(0)));
    assert_eq!(report.operational_status, "operational");
  }

  #[test]
  fn test_spawn_and_stop() {
    let adapter = PortablePtyAdapter;
    // On Windows we would use "cmd", on Unix "sh"
    #[cfg(windows)]
    let mut cmd = CommandBuilder::new("cmd");
    #[cfg(not(windows))]
    let mut cmd = CommandBuilder::new("sh");

    // Test spawning
    let mut spawned = adapter.spawn(cmd, 80, 24).expect("should spawn process");

    // Ensure child can be stopped
    let stop_res = adapter.stop(spawned.child.as_mut());
    assert!(stop_res.is_ok(), "should successfully stop the child process");
  }
}

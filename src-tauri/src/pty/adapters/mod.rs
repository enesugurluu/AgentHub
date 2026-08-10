use std::io::{Read, Write};
use std::time::Duration;
use serde::{Deserialize, Serialize};

use portable_pty::CommandBuilder;

mod portable_pty_native;

pub use portable_pty_native::PortablePtyAdapter;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EngineMetadata {
    pub engine_type: String,
    pub version: Option<String>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DetectResult {
    pub detected: bool,
    pub version: Option<String>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUtil {
    pub cpu_percent: Option<f32>,
    pub memory_bytes: Option<u64>,
    pub process_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthReport {
    pub ok: bool,
    pub message: Option<String>,
    pub uptime: Option<Duration>,
    pub resource_utilization: Option<ResourceUtil>,
    pub operational_status: String,
}

/// Pluggable backend for creating and managing PTY-backed engine processes.
///
/// This abstraction is intentionally small so it can be mocked in unit tests and
/// swapped at runtime using the adapter registry.
pub trait EngineAdapter: Send + Sync + 'static {
  /// Stable identifier for registry lookups.
  fn id(&self) -> &str;

  /// Returns metadata about the engine (type, version, capabilities).
  fn metadata(&self) -> EngineMetadata {
      EngineMetadata::default()
  }

  /// Returns true if this adapter can run on the current host (OS, availability of
  /// underlying PTY APIs, etc).
  fn detect(&self) -> bool;

  /// Returns detailed detection status, along with capabilities and version info.
  fn detect_info(&self) -> DetectResult {
      DetectResult {
          detected: self.detect(),
          version: self.metadata().version,
          capabilities: self.metadata().capabilities,
      }
  }

  /// Performs a cheap health check (configuration / dependencies) and returns an
  /// error message if the adapter is not usable.
  fn health(&self) -> Result<(), String>;

  /// Returns a detailed health report including uptime and resource utilization.
  fn health_report(&self) -> HealthReport {
      let res = self.health();
      HealthReport {
          ok: res.is_ok(),
          message: res.err(),
          uptime: None,
          resource_utilization: None,
          operational_status: "unknown".to_string(),
      }
  }

  fn spawn(&self, cmd: CommandBuilder, cols: u16, rows: u16) -> Result<SpawnedPty, String>;

  fn stop(&self, child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String>;
}

pub struct SpawnedPty {
  pub reader: Box<dyn Read + Send>,
  pub writer: Box<dyn Write + Send>,
  pub child: Box<dyn portable_pty::Child + Send + Sync>,
}

use std::io::{Read, Write};

use portable_pty::CommandBuilder;

mod portable_pty_native;

pub use portable_pty_native::PortablePtyAdapter;

/// Pluggable backend for creating and managing PTY-backed engine processes.
///
/// This abstraction is intentionally small so it can be mocked in unit tests and
/// swapped at runtime using the adapter registry.
pub trait EngineAdapter: Send + Sync + 'static {
  /// Stable identifier for registry lookups.
  fn id(&self) -> &str;

  /// Returns true if this adapter can run on the current host (OS, availability of
  /// underlying PTY APIs, etc).
  fn detect(&self) -> bool;

  /// Performs a cheap health check (configuration / dependencies) and returns an
  /// error message if the adapter is not usable.
  fn health(&self) -> Result<(), String>;

  fn spawn(&self, cmd: CommandBuilder, cols: u16, rows: u16) -> Result<SpawnedPty, String>;

  fn stop(&self, child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String>;
}

pub struct SpawnedPty {
  pub reader: Box<dyn Read + Send>,
  pub writer: Box<dyn Write + Send>,
  pub child: Box<dyn portable_pty::Child + Send + Sync>,
}

use portable_pty::{native_pty_system, CommandBuilder, PtySize};

use super::{EngineAdapter, SpawnedPty};

/// Built-in adapter backed by the `portable-pty` crate's native PTY implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct PortablePtyAdapter;

impl EngineAdapter for PortablePtyAdapter {
  fn id(&self) -> &str {
    "portable-pty-native"
  }

  fn detect(&self) -> bool {
    // `portable-pty` compiles on all our supported targets. If PTY APIs are missing,
    // `health()` will surface it.
    true
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

  fn spawn(&self, cmd: CommandBuilder, cols: u16, rows: u16) -> Result<SpawnedPty, String> {
    let pty_system = native_pty_system();
    let pair = pty_system
      .openpty(PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
      })
      .map_err(|e| e.to_string())?;

    let child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
    let reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
    let writer = pair.master.take_writer().map_err(|e| e.to_string())?;

    Ok(SpawnedPty {
      reader,
      writer,
      child,
    })
  }

  fn stop(&self, child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String> {
    child.kill().map_err(|e| e.to_string())?;
    let _ = child.wait();
    Ok(())
  }
}

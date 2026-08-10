use std::{collections::HashMap, io::Write, sync::Mutex};

pub mod engine_adapter_registry;

pub use engine_adapter_registry::{EngineAdapterQuery, EngineAdapterRegistry};

#[derive(Default)]
pub struct PtyManager {
  pub sessions: Mutex<HashMap<String, PtySession>>,
}

pub struct PtySession {
  pub adapter_id: String,
  pub execution_id: String,
  pub writer: Box<dyn Write + Send>,
  pub child: Box<dyn portable_pty::Child + Send + Sync>,
  #[cfg(target_os = "windows")]
  pub job_handle: Option<isize>,
}

#[cfg(target_os = "windows")]
impl Drop for PtySession {
  fn drop(&mut self) {
    if let Some(handle) = self.job_handle {
      unsafe {
        windows_sys::Win32::Foundation::CloseHandle(handle as _);
      }
    }
  }
}

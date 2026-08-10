use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;

use portable_pty::CommandBuilder;
use serde::{Deserialize, Serialize};

mod portable_pty_native;

pub use portable_pty_native::PortablePtyAdapter;

// IPC üzerinden frontend'e giden struct'larda camelCase zorunlu (TS tipleriyle birebir eşleşme).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EngineMetadata {
    pub engine_type: String,
    pub version: Option<String>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DetectResult {
    pub detected: bool,
    pub version: Option<String>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceUtil {
    pub cpu_percent: Option<f32>,
    pub memory_bytes: Option<u64>,
    pub process_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HealthReport {
    pub ok: bool,
    pub message: Option<String>,
    pub uptime: Option<Duration>,
    pub resource_utilization: Option<ResourceUtil>,
    pub operational_status: String,
}

/// CLI ajanlarını (`claude`, `codex`, ...) spawn ederken adaptöre verilen seçenekler.
///
/// Komut yapısı (program, flag'ler) adaptörün kendi sorumluluğundadır — her CLI'nin
/// kurulum/fleg eşleşmesi farklıdır (AjanOfis docs Bölüm 7.2).
#[derive(Debug, Clone)]
pub struct CliSpawnOptions {
  /// Ajanın çalışacağı dizin (worktree yolu).
  pub workdir: PathBuf,
  /// Sürece enjekte edilecek ortam değişkenleri.
  pub env: Vec<(String, String)>,
  /// Ek argümanlar (ör. `-p`, `--model`, `--max-budget-usd`).
  pub args: Vec<String>,
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

  /// CLI ajanlarını (claude, codex, ...) kendi komut kurallarıyla spawn eder.
  /// Varsayılan: desteklenmiyor — sadece CLI adaptörleri override eder.
  fn spawn_cli(
    &self,
    _opts: CliSpawnOptions,
    _cols: u16,
    _rows: u16,
  ) -> Result<SpawnedPty, String> {
    Err(format!(
      "adapter '{}' does not support CLI spawning",
      self.id()
    ))
  }

  /// PTY boyutunu günceller (xterm fit → `pty_resize` IPC).
  ///
  /// portable-pty 0.9'da resize `Child` üzerinde değil, PTY'nin **master**
  /// ucunda (`MasterPty::resize`) gerçekleşir.
  fn resize(
    &self,
    master: &dyn portable_pty::MasterPty,
    cols: u16,
    rows: u16,
  ) -> Result<(), String> {
    master
      .resize(portable_pty::PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
      })
      .map_err(|e| e.to_string())
  }

  fn stop(&self, child: &mut (dyn portable_pty::Child + Send + Sync)) -> Result<(), String>;
}

pub struct SpawnedPty {
  pub reader: Box<dyn Read + Send>,
  pub writer: Box<dyn Write + Send>,
  pub master: Box<dyn portable_pty::MasterPty + Send>,
  pub child: Box<dyn portable_pty::Child + Send + Sync>,
  #[cfg(target_os = "windows")]
  pub job_handle: Option<isize>,
}

/// Ortak PTY spawn yardımcısı: `portable-pty` ile süreç açar ve Windows'ta
/// Job Objects (KILL_ON_JOB_CLOSE) ile child ağacını izole eder.
/// Tüm adaptörler bu fonksiyonu kullanır (izolasyon tek noktada).
pub(crate) fn spawn_pty_isolated(
  cmd: CommandBuilder,
  cols: u16,
  rows: u16,
) -> Result<SpawnedPty, String> {
  use portable_pty::{native_pty_system, PtySize};

  let pty_system = native_pty_system();
  let pair = pty_system
    .openpty(PtySize {
      rows,
      cols,
      pixel_width: 0,
      pixel_height: 0,
    })
    .map_err(|e| e.to_string())?;

  // NOT: `mut` yalnızca Windows hata yollarındaki child.kill() (&mut self) için
  // gerekli; Unix build'inde unused_mut üretir — allow her iki hedefi de temizler.
  #[allow(unused_mut)]
  let mut child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;

  // Windows'ta Job Object handle'ı struct kurulumunda (aşağıda) kullanıldığı
  // için bildirim `#[cfg]` bloğunun DIŞINDA tutulur; değer ataması yalnızca
  // Windows'ta yapılır.
  #[allow(unused_mut)]
  let mut final_job_handle: Option<isize> = None;

  #[cfg(not(target_os = "windows"))]
  {
    // Windows dışında değer atanmaz; uyarıyı bastırmak için kullanıldığını işaretle.
    let _ = &final_job_handle;
  }

  #[cfg(target_os = "windows")]
  {
    use std::mem;
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    use windows_sys::Win32::System::JobObjects::{
      AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject,
      JobObjectExtendedLimitInformation, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
      JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    };
    use windows_sys::Win32::System::Threading::{
      OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE,
    };

    if let Some(pid) = child.process_id() {
      unsafe {
        let job: HANDLE = CreateJobObjectW(std::ptr::null(), std::ptr::null());
        if job != 0 {
          let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = mem::zeroed();
          info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

          let res = SetInformationJobObject(
            job,
            JobObjectExtendedLimitInformation,
            &info as *const _ as *const std::ffi::c_void,
            mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
          );

          if res != 0 {
            let proc_handle = OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, 0, pid);
            if proc_handle != 0 {
              let assign_res = AssignProcessToJobObject(job, proc_handle);
              CloseHandle(proc_handle);
              if assign_res == 0 {
                // Failed to assign. Kill the process immediately and return error.
                let _ = child.kill();
                let _ = child.wait();
                CloseHandle(job);
                return Err("Failed to assign process to Job Object".to_string());
              }

              final_job_handle = Some(job as isize);
            } else {
               let _ = child.kill();
               let _ = child.wait();
               CloseHandle(job);
               return Err("Failed to open process for job assignment".to_string());
            }
          } else {
               let _ = child.kill();
               let _ = child.wait();
               CloseHandle(job);
               return Err("Failed to set job object information".to_string());
          }
        } else {
           let _ = child.kill();
           let _ = child.wait();
           return Err("Failed to create job object".to_string());
        }
      }
    }
  }

  let reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
  let writer = pair.master.take_writer().map_err(|e| e.to_string())?;

  Ok(SpawnedPty {
    reader,
    writer,
    master: pair.master,
    child,
    #[cfg(target_os = "windows")]
    job_handle: final_job_handle,
  })
}

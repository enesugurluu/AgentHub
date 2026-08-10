use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::time::Duration;

use super::{DetectResult, EngineAdapter, EngineMetadata, HealthReport, SpawnedPty};

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

    #[cfg(target_os = "windows")]
    {
      use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
      use windows_sys::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, SetInformationJobObject,
        JobObjectExtendedLimitInformation, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
      };
      use windows_sys::Win32::System::Threading::{
        OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE,
      };
      use std::mem;

      let mut final_job_handle: Option<isize> = None;

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
      child,
      #[cfg(target_os = "windows")]
      job_handle: final_job_handle,
    })
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

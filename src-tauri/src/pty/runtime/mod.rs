use std::io::Read;
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};
use crate::pty::registry::PtyManager;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PtyOutputEvent {
  pub agent_id: String,
  pub execution_id: String,
  pub data: String,
}

pub fn start_output_pump(app: AppHandle, agent_id: String, execution_id: String, mut reader: Box<dyn Read + Send>) {
  // Output pump thread
  let agent_id_clone = agent_id.clone();
  let execution_id_clone = execution_id.clone();
  let app_clone = app.clone();

  thread::spawn(move || {
    let mut buf = [0u8; 4096];
    loop {
      match reader.read(&mut buf) {
        Ok(0) => break,
        Ok(n) => {
          let data = String::from_utf8_lossy(&buf[..n]).to_string();
          let _ = app_clone.emit(
            "agent://output",
            PtyOutputEvent {
              agent_id: agent_id_clone.clone(),
              execution_id: execution_id_clone.clone(),
              data,
            },
          );
        }
        Err(_) => break,
      }
    }
  });

  // Lifecycle monitor thread
  thread::spawn(move || {
    loop {
      thread::sleep(Duration::from_millis(500));

      let state: State<PtyManager> = app.state();

      let mut remove = false;
      if let Ok(mut sessions) = state.sessions.lock() {
        if let Some(session) = sessions.get_mut(&agent_id) {
          if session.execution_id == execution_id {
            // We have mutable access to the child now
            if let Ok(Some(_status)) = session.child.try_wait() {
                remove = true;
            }
          } else {
             // A different execution has taken over this agent ID, so this monitor is obsolete.
             break;
          }
        } else {
            // Session is already gone
            break;
        }

        if remove {
            sessions.remove(&agent_id);
        }
      }

      if remove {
         let _ = app.emit(
           "agent://status",
           PtyStatusEvent {
             agent_id: agent_id.clone(),
             execution_id: execution_id.clone(),
             status: "exited".to_string(),
           }
         );
         break;
      }
    }
  });
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PtyStatusEvent {
  pub agent_id: String,
  pub execution_id: String,
  pub status: String,
}


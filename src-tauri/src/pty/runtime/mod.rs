//! PTY çıktı pompası ve yaşam döngüsü monitörü.
//!
//! - Çıktı, per-session `Channel<PtyEvent>` üzerinden **ham bayt** olarak akar
//!   (UTF-8 çok baytlı karakterlerin chunk sınırında bozulmasını önler;
//!   xterm `Uint8Array`'i doğrudan işler).
//! - Çıkış, `Exit { code }` olayı ile frontend'e bildirilir ve `events`
//!   tablosuna yazılır.

use std::io::Read;
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::ipc::Channel;
use tauri::{AppHandle, Manager, State};

use crate::db::AppDb;
use crate::pty::registry::PtyManager;

/// Frontend'e giden PTY olayı (serde tag'i: `kind.type = "output" | "exit"`).
///
/// NOT: Channel payload'ları serde ile birebir serialize edilir; Tauri alan
/// adlarını dönüştürmez. Frontend `agentId`/`executionId` beklediği için
/// camelCase rename zorunlu — aksi halde tüm olaylar sessizce düşer.
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PtyEvent {
  pub agent_id: String,
  pub execution_id: String,
  pub kind: PtyEventKind,
}

#[derive(Serialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PtyEventKind {
  Output { data: Vec<u8> },
  Exit { code: u32 },
}

pub fn start_output_pump(
  app: AppHandle,
  agent_id: String,
  execution_id: String,
  mut reader: Box<dyn Read + Send>,
  channel: Channel<PtyEvent>,
) {
  // -- Çıktı pompası: reader → Channel (ham bayt) ------------------------------
  {
    let agent_id = agent_id.clone();
    let execution_id = execution_id.clone();
    let channel = channel.clone();

    thread::spawn(move || {
      let mut buf = [0u8; 8192];
      loop {
        match reader.read(&mut buf) {
          Ok(0) => break,
          Ok(n) => {
            let event = PtyEvent {
              agent_id: agent_id.clone(),
              execution_id: execution_id.clone(),
              kind: PtyEventKind::Output {
                data: buf[..n].to_vec(),
              },
            };
            if channel.send(event).is_err() {
              // Frontend kanalı kapandıysa (pencere/sekme kapatıldı) pompayı bitir.
              break;
            }
          }
          Err(_) => break,
        }
      }
    });
  }

  // -- Yaşam döngüsü monitörü: try_wait → Exit olayı + DB kaydı ------------------
  thread::spawn(move || {
    loop {
      thread::sleep(Duration::from_millis(250));

      let state: State<PtyManager> = app.state();

      let mut remove = false;
      let mut exit_code: u32 = 0;

      if let Ok(mut sessions) = state.sessions.lock() {
        if let Some(session) = sessions.get_mut(&agent_id) {
          if session.execution_id == execution_id {
            // Mutable erişimle child durumunu kontrol et.
            if let Ok(Some(status)) = session.child.try_wait() {
              exit_code = status.exit_code();
              remove = true;
            }
          } else {
            // Bu agent ID'sini farklı bir execution devralmış; monitor artık geçersiz.
            break;
          }
        } else {
          // Oturum zaten kaldırılmış (agent_stop tarafından).
          break;
        }

        if remove {
          sessions.remove(&agent_id);
        }
      }

      if remove {
        let event = PtyEvent {
          agent_id: agent_id.clone(),
          execution_id: execution_id.clone(),
          kind: PtyEventKind::Exit { code: exit_code },
        };
        let _ = channel.send(event);

        if let Some(db) = app.try_state::<AppDb>() {
          let payload = serde_json::json!({ "executionId": execution_id, "code": exit_code });
          let _ = db.record_event(
            Some(&agent_id),
            None,
            "exit",
            Some(&payload.to_string()),
          );
        }
        break;
      }
    }
  });
}

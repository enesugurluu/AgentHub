use std::io::Read;
use std::thread;

use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Serialize, Clone)]
pub struct PtyOutputEvent {
  pub id: String,
  pub data: String,
}

pub fn start_output_pump(app: AppHandle, id: String, mut reader: Box<dyn Read + Send>) {
  thread::spawn(move || {
    let mut buf = [0u8; 4096];
    loop {
      match reader.read(&mut buf) {
        Ok(0) => break,
        Ok(n) => {
          let data = String::from_utf8_lossy(&buf[..n]).to_string();
          let _ = app.emit(
            "pty://output",
            PtyOutputEvent {
              id: id.clone(),
              data,
            },
          );
        }
        Err(_) => break,
      }
    }
  });
}

